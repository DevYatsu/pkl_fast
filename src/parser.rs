use crate::parser::{
    errors::{
        lexing::parse_lexing_error,
        locating::{generate_source, get_error_location},
        InvalidAsStatement, ParsingError,
    },
    statement::Statement,
    utils::{parse_identifier, retrieve_next_token},
};

use crate::lexer::PklToken;
use logos::Lexer;

pub mod errors;
mod operator;
pub mod statement;
mod types;
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
    stored_value: Option<&'source str>,
    lexer: PklLexer<'source>,
}

impl<'source> PklParser<'source> {
    pub fn new(lexer: PklLexer<'source>) -> Self {
        Self {
            stored_value: None,
            statements: vec![],
            lexer,
        }
    }

    pub fn parse(&mut self) -> ParsingResult<()> {
        while let Some(token) = self.lexer.next() {
            let mut lexer = &mut self.lexer;

            let statement = match token {
                Ok(PklToken::Import) => self.parse_import()?,
                Ok(PklToken::GlobbedImport) => self.parse_globbed_import()?,
                Ok(PklToken::Amends) => self.parse_amends()?,
                Ok(PklToken::Module) => self.parse_module()?,
                Ok(PklToken::Extends) => self.parse_extends()?,
                Ok(PklToken::As) => {
                    let imported_as_new_value = parse_identifier(&mut lexer)?;
                    if let Some(statement) = self.statements.last_mut() {
                        match statement {
                            Statement::Import { imported_as, .. }
                            | Statement::GlobbedImport { imported_as, .. } => {
                                *imported_as = Some(imported_as_new_value);
                            }
                            _ => {
                                return Err(ParsingError::AsStatementUnsupported(
                                    InvalidAsStatement {
                                        src: generate_source("main.pkl", lexer.source()),
                                        at: get_error_location(&mut lexer).into(),
                                    },
                                ));
                            }
                        }
                    } else {
                        return Err(ParsingError::unexpected(&mut lexer));
                    }

                    continue;
                }
                Ok(PklToken::Identifier(id)) => {
                    // match for variable declaration, object declaration and variable assignment
                    self.parse_var_statement(id)?
                }
                Ok(PklToken::ModuleInfo) => self.parse_module_info()?,
                Ok(PklToken::DeprecatedInstruction) => self.parse_deprecated()?,
                Ok(PklToken::TypeAlias) => self.parse_typealias()?,
                Ok(PklToken::Class) => self.parse_class_declaration()?,
                Ok(PklToken::Abstract) => {
                    todo!()
                }
                Ok(PklToken::Open) => {
                    let token = retrieve_next_token(&mut lexer)?;

                    match token {
                        PklToken::Module => self.parse_open_module()?,
                        PklToken::Class => self.parse_open_class_declaration()?,
                        _ => return Err(ParsingError::unexpected(&mut lexer)),
                    }
                }
                Err(e) => return Err(parse_lexing_error(&mut lexer, e)),
                _ => continue,
            };

            self.statements.push(statement);
        }

        Ok(())
    }

    fn parse_import(&mut self) -> ParsingResult<Statement<'source>> {
        statement::parse_import(&mut self.lexer)
    }
    fn parse_globbed_import(&mut self) -> ParsingResult<Statement<'source>> {
        statement::parse_globbed_import(&mut self.lexer)
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
        statement::parse_class_declaration(&mut self.lexer, false)
    }
    fn parse_open_class_declaration(&mut self) -> ParsingResult<Statement<'source>> {
        statement::parse_class_declaration(&mut self.lexer, true)
    }

    fn parse_var_statement(&mut self, id: &'source str) -> ParsingResult<Statement<'source>> {
        statement::parse_var_statement(&mut self.lexer, id)
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
