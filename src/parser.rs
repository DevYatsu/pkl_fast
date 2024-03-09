use crate::parser::{
    errors::{lexing::parse_lexing_error, ParsingError},
    statement::Statement,
    utils::retrieve_next_token,
};

use crate::lexer::PklToken;
use logos::Lexer;

use self::{
    expression::{parse_expr, Expression},
    statement::{
        import::{import_clause, parse_import_value},
        parse_class_field, ClassType,
    },
    types::parse_type,
    utils::{
        expect_statement_end, expect_token, hashmap_while_not_token2, list_while_not_token2,
        parse_identifier, parse_opt_newlines, retrieve_opt_next_token,
    },
    value::parse_object,
};

pub mod errors;
mod expression;
mod macros;
mod operator;
pub mod statement;
mod types;

#[macro_use]
mod utils;
pub mod value;

pub type ParsingResult<T> = miette::Result<T, ParsingError>;
pub type PklLexer<'source> = Lexer<'source, PklToken<'source>>;

pub fn parse<'source>(
    lexer: PklLexer<'source>,
    strings_vec: Vec<&'source str>,
) -> ParsingResult<Vec<statement::Statement<'source>>> {
    let mut parser = PklParser::new(lexer, strings_vec);

    parser.parse()?;
    Ok(parser.statements)
}

#[derive(Debug, Clone)]
/// PklParser is the main parser struct, possessing the `parse` method to parse the tokens in the lexer.
///
/// **IMPORTANT NOTE**:
/// All parsing functions (in the entire library) are designed to operate on the next token from the lexer,
/// except for the function that requires an optional token as a second argument, which will be consumed instead of consuming the next one.
pub struct PklParser<'source> {
    pub statements: Vec<Statement<'source>>,
    lexer: PklLexer<'source>,
    new_line_parsed: bool,
    strings_vec: Vec<&'source str>,
}

impl<'source> PklParser<'source> {
    /// The function to initialize an instance of PklParser.
    pub fn new(lexer: PklLexer<'source>, strings_vec: Vec<&'source str>) -> Self {
        Self {
            statements: vec![],
            new_line_parsed: false,
            lexer,
            strings_vec,
        }
    }

    /// This function parses the tokens in the lexer.
    ///
    /// To access the parsed statements, use the `statements` field.
    pub fn parse(&mut self) -> ParsingResult<()> {
        while let Some(token) = self.lexer.next() {
            let statement = match token {
                Ok(PklToken::Import) => {
                    let stmt = self.parse_import()?;

                    if self.new_line_parsed {
                        self.new_line_parsed = !self.new_line_parsed;
                    } else {
                        expect_statement_end(self)?;
                    }

                    stmt
                }
                Ok(PklToken::GlobbedImport) => {
                    let stmt = self.parse_globbed_import()?;

                    if self.new_line_parsed {
                        self.new_line_parsed = !self.new_line_parsed;
                    } else {
                        expect_statement_end(self)?;
                    }

                    stmt
                }
                Ok(PklToken::Amends) => {
                    let stmt = self.parse_amends()?;
                    expect_statement_end(self)?;
                    stmt
                }
                Ok(PklToken::Module) => {
                    let stmt = self.parse_module()?;
                    if self.new_line_parsed {
                        self.new_line_parsed = !self.new_line_parsed;
                    } else {
                        expect_statement_end(self)?;
                    }
                    stmt
                }
                Ok(PklToken::Extends) => {
                    let stmt = self.parse_extends()?;
                    expect_statement_end(self)?;
                    stmt
                }
                Ok(PklToken::Local) => {
                    // local variable declaration
                    let id = parse_identifier(self)?;
                    let stmt = self.parse_var_statement(id, true)?;

                    if self.new_line_parsed {
                        self.new_line_parsed = !self.new_line_parsed;
                    } else {
                        expect_statement_end(self)?;
                    }

                    stmt
                }
                Ok(PklToken::Identifier(id)) => {
                    // match for variable declaration, object declaration and variable assignment
                    let stmt = self.parse_var_statement(id, false)?;

                    if self.new_line_parsed {
                        self.new_line_parsed = !self.new_line_parsed;
                    } else {
                        expect_statement_end(self)?;
                    }

                    stmt
                }
                Ok(PklToken::ModuleInfo) => {
                    let stmt = self.parse_module_info()?;
                    expect_statement_end(self)?;

                    stmt
                }
                Ok(PklToken::DeprecatedInstruction) => {
                    let stmt = self.parse_deprecated()?;
                    expect_statement_end(self)?;

                    stmt
                }
                Ok(PklToken::TypeAlias) => {
                    let stmt = self.parse_typealias()?;

                    if self.new_line_parsed {
                        self.new_line_parsed = !self.new_line_parsed;
                    } else {
                        expect_statement_end(self)?;
                    }

                    stmt
                }
                Ok(PklToken::Class) => {
                    let stmt = self.parse_basic_class_declaration()?;
                    if self.new_line_parsed {
                        self.new_line_parsed = !self.new_line_parsed;
                    } else {
                        expect_statement_end(self)?;
                    }
                    stmt
                }
                Ok(PklToken::Abstract) => {
                    expect_token(self, PklToken::Class)?;

                    let stmt = self.parse_abstract_class_declaration()?;

                    if self.new_line_parsed {
                        self.new_line_parsed = !self.new_line_parsed;
                    } else {
                        expect_statement_end(self)?;
                    }

                    stmt
                }
                Ok(PklToken::Open) => {
                    let token = retrieve_next_token(self)?;

                    let stmt = match token {
                        PklToken::Module => self.parse_open_module()?,
                        PklToken::Class => self.parse_open_class_declaration()?,
                        _ => {
                            return Err(ParsingError::unexpected(
                                self,
                                "class declaration or module declaration".to_owned(),
                            ))
                        }
                    };

                    if self.new_line_parsed {
                        self.new_line_parsed = !self.new_line_parsed;
                    } else {
                        expect_statement_end(self)?;
                    }

                    stmt
                }
                Err(e) => return Err(parse_lexing_error(self, e)),
                Ok(PklToken::LineComment)
                | Ok(PklToken::DocComment)
                | Ok(PklToken::NewLine)
                | Ok(PklToken::BlockComment) => continue,
                Ok(token) => {
                    let (expr, next) = parse_expr(self, Some(token))?;

                    match next {
                        Some(PklToken::NewLine)
                        | Some(PklToken::DocComment)
                        | Some(PklToken::LineComment) => (),
                        None => (),
                        _ => return Err(ParsingError::unexpected(self, "line ending".to_owned())),
                    };

                    Statement::Expression(expr)
                }
            };

            self.statements.push(statement);
        }

        Ok(())
    }

    fn parse_import(&mut self) -> ParsingResult<Statement<'source>> {
        let value = parse_import_value(self)?;

        let next_token = retrieve_opt_next_token(self)?;

        let imported_as = match next_token {
            Some(PklToken::As) => Some(parse_identifier(self)?),
            Some(PklToken::NewLine)
            | Some(PklToken::LineComment)
            | Some(PklToken::DocComment)
            | None => {
                self.new_line_parsed = true;
                None
            }
            _ => {
                return Err(ParsingError::unexpected(
                    self,
                    "'as <identifier>' or line end".to_owned(),
                ))
            }
        };

        Ok(Statement::Import {
            clause: import_clause(value),
            imported_as,
        })
    }
    fn parse_globbed_import(&mut self) -> ParsingResult<Statement<'source>> {
        let value = parse_import_value(self)?;

        let next_token = retrieve_opt_next_token(self)?;

        let imported_as = match next_token {
            Some(PklToken::As) => Some(parse_identifier(self)?),
            Some(PklToken::NewLine)
            | Some(PklToken::LineComment)
            | Some(PklToken::DocComment)
            | None => {
                self.new_line_parsed = true;
                None
            }
            _ => {
                return Err(ParsingError::unexpected(
                    self,
                    "'as' identifier or line end".to_owned(),
                ))
            }
        };

        Ok(Statement::GlobbedImport {
            clause: import_clause(value),
            imported_as,
        })
    }
    fn parse_amends(&mut self) -> ParsingResult<Statement<'source>> {
        statement::parse_amends(self)
    }
    fn parse_extends(&mut self) -> ParsingResult<Statement<'source>> {
        statement::parse_extends(self)
    }

    fn parse_module(&mut self) -> ParsingResult<Statement<'source>> {
        self.parse_mod(false)
    }
    fn parse_open_module(&mut self) -> ParsingResult<Statement<'source>> {
        self.parse_mod(true)
    }

    fn parse_mod(&mut self, open: bool) -> ParsingResult<Statement<'source>> {
        let (value, next_token) = parse_expr(self, None)?;

        match next_token {
            Some(PklToken::NewLine)
            | Some(PklToken::LineComment)
            | Some(PklToken::DocComment)
            | None => {
                self.new_line_parsed = true;
            }
            _ => {
                return Err(ParsingError::unexpected(
                    self,
                    "'as <identifier>' or line end".to_owned(),
                ))
            }
        };

        Ok(Statement::Module { value, open })
    }

    fn parse_basic_class_declaration(&mut self) -> ParsingResult<Statement<'source>> {
        self.parse_class_declaration(ClassType::None)
    }
    fn parse_open_class_declaration(&mut self) -> ParsingResult<Statement<'source>> {
        self.parse_class_declaration(ClassType::Open)
    }
    fn parse_abstract_class_declaration(&mut self) -> ParsingResult<Statement<'source>> {
        self.parse_class_declaration(ClassType::Abstract)
    }

    fn parse_class_declaration(&mut self, _type: ClassType) -> ParsingResult<Statement<'source>> {
        let name = parse_identifier(self)?;
        let token = retrieve_next_token(self)?;

        let extends = match token {
            PklToken::Extends => {
                let value = parse_identifier(self)?;

                match retrieve_opt_next_token(self)? {
                    Some(PklToken::OpenBracket) => (),
                    Some(PklToken::NewLine)
                    | Some(PklToken::LineComment)
                    | Some(PklToken::DocComment) => {
                        self.new_line_parsed = true;
                        return Ok(Statement::ClassDeclaration {
                            name,
                            extends: Some(value),
                            _type,
                            fields: None,
                        });
                    }
                    _ => {
                        return Err(ParsingError::unexpected(
                            self,
                            "'{' or line ending".to_owned(),
                        ))
                    }
                };

                Some(value)
            }
            PklToken::OpenBracket => None,
            PklToken::NewLine | PklToken::LineComment | PklToken::DocComment => {
                self.new_line_parsed = true;
                return Ok(Statement::ClassDeclaration {
                    name,
                    extends: None,
                    _type,
                    fields: None,
                });
            }
            _ => return Err(ParsingError::unexpected(self, "'{'".to_owned())),
        };

        let fields = hashmap_while_not_token2(
            self,
            PklToken::NewLine,
            PklToken::CloseBracket,
            &parse_class_field,
        )?
        .into();

        Ok(Statement::ClassDeclaration {
            name,
            extends,
            _type,
            fields,
        })
    }

    // this function is defined here as it uses self.new_line_parsed
    fn parse_var_statement(
        &mut self,
        name: &'source str,
        is_local: bool,
    ) -> ParsingResult<Statement<'source>> {
        let token = retrieve_next_token(self)?;

        let optional_type = match token {
            PklToken::OpenBracket => {
                let (object, next_token) = parse_object(self, None)?;

                match next_token {
                    Some(PklToken::NewLine)
                    | Some(PklToken::BlockComment)
                    | Some(PklToken::DocComment) => {
                        self.new_line_parsed = true;
                    }
                    None => (),
                    _ => return Err(ParsingError::unexpected(self, "'='".to_owned())),
                };

                return Ok(Statement::VariableDeclaration {
                    name,
                    value: Expression::Value(object),
                    optional_type: None,
                    is_local,
                });
            }
            PklToken::Colon => {
                let (variable_type, next_token) = parse_opt_newlines(self, &parse_type)?;

                match next_token {
                    Some(PklToken::EqualSign) => {}
                    Some(PklToken::NewLine) => {
                        self.new_line_parsed = true;

                        return Ok(Statement::VariableDeclaration {
                            name,
                            value: Expression::Value(variable_type.default_value(self)?),
                            optional_type: Some(variable_type),
                            is_local,
                        });
                    }
                    None => {
                        return Ok(Statement::VariableDeclaration {
                            name,
                            value: Expression::Value(variable_type.default_value(self)?),
                            optional_type: Some(variable_type),
                            is_local,
                        })
                    }
                    _ => return Err(ParsingError::unexpected(self, "'='".to_owned())),
                };

                Some(variable_type)
            }
            PklToken::EqualSign => None,
            _ => Err(ParsingError::unexpected(self, "'=', ':' or '{'".to_owned()))?,
        };

        let (value, next_token) = parse_opt_newlines(self, &parse_expr)?;

        match next_token {
            Some(PklToken::NewLine) | Some(PklToken::DocComment) | Some(PklToken::LineComment) => {
                self.new_line_parsed = true;
            }
            None => (),
            _ => return Err(ParsingError::unexpected(self, "line end".to_owned())),
        }

        Ok(Statement::VariableDeclaration {
            name,
            value,
            optional_type,
            is_local,
        })
    }
    fn parse_typealias(&mut self) -> ParsingResult<Statement<'source>> {
        let token = retrieve_next_token(self)?;

        let (alias, generics_params) = match token {
            PklToken::Identifier(v) => (v, None),
            PklToken::GenericTypeAnnotationStart(name) => {
                let types = list_while_not_token2(
                    self,
                    PklToken::Comma,
                    PklToken::RightAngleBracket(">"),
                    &parse_type,
                )?;

                (name, Some(types))
            }
            _ => return Err(ParsingError::expected_identifier(self)),
        };

        expect_token(self, PklToken::EqualSign)?;

        let (equivalent_type, next_token) = parse_opt_newlines(self, &parse_type)?;

        match next_token {
            Some(PklToken::NewLine) | Some(PklToken::LineComment) | Some(PklToken::DocComment) => {
                self.new_line_parsed = true;
            }
            _ => return Err(ParsingError::unexpected(self, "line ending".to_owned())),
        };

        Ok(Statement::TypeAlias {
            alias,
            equivalent_type,
            generics_params,
        })
    }

    fn parse_module_info(&mut self) -> ParsingResult<Statement<'source>> {
        statement::parse_module_info(self)
    }

    fn parse_deprecated(&mut self) -> ParsingResult<Statement<'source>> {
        statement::parse_deprecated(self)
    }
}
