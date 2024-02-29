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
    utils::{expect_statement_end, expect_token, parse_identifier}, value::{parse_object},
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
pub struct PklParser<'source> {
    pub statements: Vec<Statement<'source>>,
    lexer: PklLexer<'source>,
    new_line_parsed: bool,
}

impl<'source> PklParser<'source> {
    pub fn new(lexer: PklLexer<'source>) -> Self {
        Self {
            statements: vec![],
            new_line_parsed: false,
            lexer,
        }
    }

    pub fn parse(&mut self) -> ParsingResult<()> {
        while let Some(token) = self.lexer.next() {
            let statement = match token {
                Ok(PklToken::Import) => {
                    // no need to check if there is a statement end next, it was already checked in the function call below
                    self.parse_import()?
                }
                Ok(PklToken::GlobbedImport) => {
                    // no need to check if there is a statement end next, it was already checked in the function call below
                    self.parse_globbed_import()?
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
                        // skipping comments and newline
                        expect_statement_end(&mut self.lexer)?;
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
                    expect_statement_end(&mut self.lexer)?;

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
                        _ => return Err(ParsingError::unexpected(&mut self.lexer)),
                    }
                }
                Err(e) => return Err(parse_lexing_error(&mut self.lexer, e)),
                _ => continue,
            };

            self.statements.push(statement);
        }
        println!("{:?}", self.statements);
        Ok(())
    }

    fn parse_import(&mut self) -> ParsingResult<Statement<'source>> {
        let value = parse_import_value(&mut self.lexer)?;

        let next_token = retrieve_next_token(&mut self.lexer)?;

        let imported_as = match next_token {
            PklToken::As => Some(parse_identifier(&mut self.lexer)?),
            PklToken::NewLine | PklToken::LineComment | PklToken::BlockComment => None,
            _ => return Err(ParsingError::unexpected(&mut self.lexer)),
        };

        Ok(Statement::Import {
            clause: import_clause(value),
            imported_as,
        })
    }
    fn parse_globbed_import(&mut self) -> ParsingResult<Statement<'source>> {
        let value = parse_import_value(&mut self.lexer)?;

        let next_token = retrieve_next_token(&mut self.lexer)?;

        let imported_as = match next_token {
            PklToken::As => Some(parse_identifier(&mut self.lexer)?),
            PklToken::NewLine | PklToken::LineComment | PklToken::BlockComment => None,
            _ => return Err(ParsingError::unexpected(&mut self.lexer)),
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

        let statement = match token {
            PklToken::EqualSign => {
                let value = parse_expr(lexer)?;

                Statement::VariableDeclaration {
                    name,
                    value,
                    optional_type: None,
                }
            }
            PklToken::OpenBracket => {
                let value = Expression::Value(parse_object(lexer, None)?);

                Statement::VariableDeclaration {
                    name,
                    value,
                    optional_type: None,
                }
            }
            PklToken::Colon => {
                let variable_type = parse_type(lexer)?;

                let next_token = retrieve_next_token(lexer)?;

                match next_token {
                    PklToken::EqualSign => {}
                    PklToken::NewLine => {
                        self.new_line_parsed = true;

                        return Ok(Statement::VariableDeclaration {
                            name,
                            value: expression::Expression::Value(
                                variable_type.default_value(lexer)?,
                            ),
                            optional_type: Some(variable_type),
                        });
                    }
                    _ => return Err(ParsingError::unexpected(lexer)),
                };

                let value = parse_expr(lexer)?;

                Statement::VariableDeclaration {
                    name,
                    value,
                    optional_type: Some(variable_type),
                }
            }
            _ => Err(ParsingError::unexpected(lexer))?,
        };

        Ok(statement)
    }
    fn parse_typealias(&mut self) -> ParsingResult<Statement<'source>> {
        statement::parse_typealias(&mut self.lexer)
    }

    fn parse_module_info(&mut self) -> ParsingResult<Statement<'source>> {
        statement::parse_module_info(&mut self.lexer)
    }

    fn parse_deprecated(&mut self) -> ParsingResult<Statement<'source>> {
        statement::parse_deprecated(&mut self.lexer)
    }
}
