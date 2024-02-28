use crate::parser::{
    errors::{lexing::parse_lexing_error, ParsingError},
    statement::Statement,
    utils::{parse_identifier, retrieve_next_token},
};

use crate::lexer::PklToken;
use logos::Lexer;

use self::{
    statement::ClassType,
    types::parse_type,
    utils::expect_token,
    value::{parse_object, parse_value},
};

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
                Ok(PklToken::Import) => self.parse_import()?,
                Ok(PklToken::GlobbedImport) => self.parse_globbed_import()?,
                Ok(PklToken::Amends) => self.parse_amends()?,
                Ok(PklToken::Module) => self.parse_module()?,
                Ok(PklToken::Extends) => self.parse_extends()?,
                Ok(PklToken::As) => {
                    if let Some(statement) = self.statements.last_mut() {
                        match statement {
                            Statement::Import { imported_as, .. }
                            | Statement::GlobbedImport { imported_as, .. } => {
                                let imported_as_new_value = parse_identifier(&mut self.lexer)?;
                                *imported_as = Some(imported_as_new_value);
                            }
                            _ => return Err(ParsingError::invalid_as_statement(&mut self.lexer)),
                        }
                    } else {
                        return Err(ParsingError::unexpected(&mut self.lexer));
                    }

                    continue;
                }
                Ok(PklToken::Identifier(id)) => {
                    // match for variable declaration, object declaration and variable assignment
                    let statement = self.parse_var_statement(id)?;

                    if self.new_line_parsed {
                        self.new_line_parsed = !self.new_line_parsed;
                    }else {
                        expect_token(&mut self.lexer, PklToken::NewLine)?;
                    }

                    statement
                }
                Ok(PklToken::ModuleInfo) => self.parse_module_info()?,
                Ok(PklToken::DeprecatedInstruction) => self.parse_deprecated()?,
                Ok(PklToken::TypeAlias) => self.parse_typealias()?,
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
                let value = parse_value(lexer)?;

                Statement::VariableDeclaration {
                    name,
                    value,
                    optional_type: None,
                }
            }
            PklToken::OpenBracket => {
                let value = parse_object(lexer, None)?;

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
                            value: variable_type.default_value(lexer)?,
                            optional_type: Some(variable_type),
                        });
                    }
                    _ => return Err(ParsingError::unexpected(lexer)),
                };

                let value = parse_value(lexer)?;

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
