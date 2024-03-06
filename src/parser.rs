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
        ClassType,
    },
    types::parse_type,
    utils::{
        expect_statement_end, expect_token, list_while_not_token2, parse_identifier,
        parse_opt_newlines, retrieve_opt_next_token,
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
) -> ParsingResult<Vec<statement::Statement<'source>>> {
    let mut parser = PklParser::new(lexer);

    parser.parse()?;
    Ok(parser.statements)
}

#[derive(Debug, Clone)]
/// PklParser is the main parser struct, possessing the `parse` method to parse the tokens in the lexer.
///
/// **IMPORTANT NOTE**: All parsing functions (in the entire library) are designed to operate on the next token from the lexer, except for the function that requires a token as a second argument.
pub struct PklParser<'source> {
    pub statements: Vec<Statement<'source>>,
    lexer: PklLexer<'source>,
    new_line_parsed: bool,
}

impl<'source> PklParser<'source> {
    /// The function to initialize an instance of PklParser.
    pub fn new(lexer: PklLexer<'source>) -> Self {
        Self {
            statements: vec![],
            new_line_parsed: false,
            lexer,
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
                        expect_statement_end(&mut self.lexer)?;
                    }

                    stmt
                }
                Ok(PklToken::GlobbedImport) => {
                    let stmt = self.parse_globbed_import()?;

                    if self.new_line_parsed {
                        self.new_line_parsed = !self.new_line_parsed;
                    } else {
                        expect_statement_end(&mut self.lexer)?;
                    }

                    stmt
                }
                Ok(PklToken::Amends) => {
                    let stmt = self.parse_amends()?;
                    expect_statement_end(&mut self.lexer)?;
                    stmt
                }
                Ok(PklToken::Module) => {
                    let stmt = self.parse_module()?;
                    expect_statement_end(&mut self.lexer)?;
                    stmt
                }
                Ok(PklToken::Extends) => {
                    let stmt = self.parse_extends()?;
                    expect_statement_end(&mut self.lexer)?;
                    stmt
                }
                Ok(PklToken::Identifier(id)) => {
                    // match for variable declaration, object declaration and variable assignment
                    let stmt = self.parse_var_statement(id)?;

                    if self.new_line_parsed {
                        self.new_line_parsed = !self.new_line_parsed;
                    } else {
                    }

                    stmt
                }
                Ok(PklToken::ModuleInfo) => {
                    let stmt = self.parse_module_info()?;
                    expect_statement_end(&mut self.lexer)?;

                    stmt
                }
                Ok(PklToken::DeprecatedInstruction) => {
                    let stmt = self.parse_deprecated()?;
                    expect_statement_end(&mut self.lexer)?;

                    stmt
                }
                Ok(PklToken::TypeAlias) => {
                    let stmt = self.parse_typealias()?;

                    if self.new_line_parsed {
                        self.new_line_parsed = !self.new_line_parsed;
                    } else {
                        expect_statement_end(&mut self.lexer)?;
                    }

                    stmt
                }
                Ok(PklToken::Class) => self.parse_class_declaration()?,
                Ok(PklToken::Abstract) => {
                    expect_token(&mut self.lexer, PklToken::Class)?;

                    self.parse_abstract_class_declaration()?
                }
                Ok(PklToken::Open) => {
                    let token = retrieve_next_token(&mut self.lexer)?;

                    match token {
                        PklToken::Module => self.parse_open_module()?,
                        PklToken::Class => self.parse_open_class_declaration()?,
                        _ => {
                            return Err(ParsingError::unexpected(
                                &mut self.lexer,
                                "class declaration or module declaration".to_owned(),
                            ))
                        }
                    }
                }
                Err(e) => return Err(parse_lexing_error(&mut self.lexer, e)),
                _ => continue,
            };

            self.statements.push(statement);
        }

        Ok(())
    }

    fn parse_import(&mut self) -> ParsingResult<Statement<'source>> {
        let value = parse_import_value(&mut self.lexer)?;

        let next_token = retrieve_opt_next_token(&mut self.lexer)?;

        let imported_as = match next_token {
            Some(PklToken::As) => Some(parse_identifier(&mut self.lexer)?),
            Some(PklToken::NewLine)
            | Some(PklToken::LineComment)
            | Some(PklToken::BlockComment)
            | None => {
                self.new_line_parsed = true;
                None
            }
            _ => {
                return Err(ParsingError::unexpected(
                    &mut self.lexer,
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
        let value = parse_import_value(&mut self.lexer)?;

        let next_token = retrieve_opt_next_token(&mut self.lexer)?;

        let imported_as = match next_token {
            Some(PklToken::As) => Some(parse_identifier(&mut self.lexer)?),
            Some(PklToken::NewLine)
            | Some(PklToken::LineComment)
            | Some(PklToken::BlockComment)
            | None => {
                self.new_line_parsed = true;
                None
            }
            _ => {
                return Err(ParsingError::unexpected(
                    &mut self.lexer,
                    "'as <identifier>' or line end".to_owned(),
                ))
            }
        };

        Ok(Statement::GlobbedImport {
            clause: import_clause(value),
            imported_as,
        })
    }
    fn parse_amends(&mut self) -> ParsingResult<Statement<'source>> {
        statement::parse_amends(&mut self.lexer)
    }
    fn parse_extends(&mut self) -> ParsingResult<Statement<'source>> {
        statement::parse_extends(&mut self.lexer)
    }

    fn parse_module(&mut self) -> ParsingResult<Statement<'source>> {
        statement::parse_module(&mut self.lexer, false)
    }
    fn parse_open_module(&mut self) -> ParsingResult<Statement<'source>> {
        statement::parse_module(&mut self.lexer, true)
    }

    fn parse_class_declaration(&mut self) -> ParsingResult<Statement<'source>> {
        statement::parse_class_declaration(&mut self.lexer, ClassType::None)
    }
    fn parse_open_class_declaration(&mut self) -> ParsingResult<Statement<'source>> {
        statement::parse_class_declaration(&mut self.lexer, ClassType::Open)
    }
    fn parse_abstract_class_declaration(&mut self) -> ParsingResult<Statement<'source>> {
        statement::parse_class_declaration(&mut self.lexer, ClassType::Abstract)
    }

    // this function is defined here as it uses self.new_line_parsed
    fn parse_var_statement(&mut self, name: &'source str) -> ParsingResult<Statement<'source>> {
        let lexer = &mut self.lexer;
        let token = retrieve_next_token(lexer)?;

        let optional_type = match token {
            PklToken::OpenBracket => {
                let value = Expression::Value(parse_object(lexer, None)?);

                return Ok(Statement::VariableDeclaration {
                    name,
                    value,
                    optional_type: None,
                });
            }
            PklToken::Colon => {
                let (variable_type, next_token) = parse_type(lexer, None)?;

                match next_token {
                    Some(PklToken::EqualSign) => {}
                    Some(PklToken::NewLine) => {
                        self.new_line_parsed = true;

                        return Ok(Statement::VariableDeclaration {
                            name,
                            value: expression::Expression::Value(
                                variable_type.default_value(lexer)?,
                            ),
                            optional_type: Some(variable_type),
                        });
                    }
                    None => {
                        return Ok(Statement::VariableDeclaration {
                            name,
                            value: expression::Expression::Value(
                                variable_type.default_value(lexer)?,
                            ),
                            optional_type: Some(variable_type),
                        })
                    }
                    _ => return Err(ParsingError::unexpected(lexer, "'='".to_owned())),
                };

                Some(variable_type)
            }
            PklToken::EqualSign => None,
            _ => Err(ParsingError::unexpected(
                lexer,
                "'=', ':' or '{'".to_owned(),
            ))?,
        };

        let (value, next_token) = parse_expr(lexer, None)?;

        match next_token {
            Some(PklToken::NewLine)
            | Some(PklToken::BlockComment)
            | Some(PklToken::LineComment) => {
                self.new_line_parsed = true;
            }
            None => (),
            _ => return Err(ParsingError::unexpected(lexer, "line end".to_owned())),
        }

        Ok(Statement::VariableDeclaration {
            name,
            value,
            optional_type,
        })
    }
    fn parse_typealias(&mut self) -> ParsingResult<Statement<'source>> {
        let lexer = &mut self.lexer;

        let token = retrieve_next_token(lexer)?;

        let (alias, generics_params) = match token {
            PklToken::Identifier(v) => (v, None),
            PklToken::GenericTypeAnnotationStart(name) => {
                let types = list_while_not_token2(
                    lexer,
                    PklToken::Comma,
                    PklToken::RightAngleBracket(">"),
                    &parse_type,
                )?;

                (name, Some(types))
            }
            _ => return Err(ParsingError::expected_identifier(lexer)),
        };

        expect_token(lexer, PklToken::EqualSign)?;

        let (equivalent_type, next_token) = parse_opt_newlines(lexer, &parse_type)?;

        match next_token {
            Some(PklToken::NewLine)
            | Some(PklToken::LineComment)
            | Some(PklToken::BlockComment) => {
                self.new_line_parsed = true;
            }
            _ => return Err(ParsingError::unexpected(lexer, "line ending".to_owned())),
        };

        Ok(Statement::TypeAlias {
            alias,
            equivalent_type,
            generics_params,
        })
    }

    fn parse_module_info(&mut self) -> ParsingResult<Statement<'source>> {
        statement::parse_module_info(&mut self.lexer)
    }

    fn parse_deprecated(&mut self) -> ParsingResult<Statement<'source>> {
        statement::parse_deprecated(&mut self.lexer)
    }
}
