use std::io;

use crate::parser::{
    errors::{
        locating::{generate_source, get_error_location},
        parse_lexing_error,
    },
    statement::Statement,
};

use self::errors::{
    InvalidAsStatement, InvalidFloatError, InvalidIdentifierError, InvalidIntError,
    InvalidStringError, UnexpectedEndOfInputError, UnexpectedError,
};
use crate::lexer::{LexingError, PklToken};
use logos::Lexer;
use miette::{diagnostic, Diagnostic};
use thiserror::Error;

pub mod errors;
mod operator;
pub mod statement;
pub mod value;

pub type ParsingResult<T> = miette::Result<T, ParsingError>;
pub type PklLexer<'source> = Lexer<'source, PklToken>;

#[derive(Error, Diagnostic, Debug)]
pub enum ParsingError {
    #[error(transparent)]
    #[diagnostic(code(pkl::io_error))]
    IoError(#[from] io::Error),

    #[error(transparent)]
    #[diagnostic(transparent)]
    LexingError(#[from] LexingError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    UnexpectedToken(#[from] UnexpectedError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    InvalidString(#[from] InvalidStringError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    InvalidInt(#[from] InvalidIntError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    InvalidFloat(#[from] InvalidFloatError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    InvalidIdentifier(#[from] InvalidIdentifierError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    AsStatementUnsupported(#[from] InvalidAsStatement),

    #[error(transparent)]
    #[diagnostic(transparent)]
    UnexpectedEndOfInput(#[from] UnexpectedEndOfInputError),
}

pub fn parse<'source>(
    mut lexer: PklLexer<'source>,
) -> ParsingResult<Vec<statement::Statement<'source>>> {
    let mut statements = vec![];

    while let Some(token) = lexer.next() {
        println!("{:?}", token);
        let statement = match token {
            Ok(PklToken::Import) => statement::parse_import(&mut lexer)?,
            Ok(PklToken::GlobbedImport) => statement::parse_globbed_import(&mut lexer)?,
            Ok(PklToken::Amends) => statement::parse_amends(&mut lexer)?,
            Ok(PklToken::Module) => statement::parse_module(&mut lexer)?,
            Ok(PklToken::Extends) => statement::parse_extends(&mut lexer)?,
            Ok(PklToken::As) => {
                let imported_as_new_value = statement::parse_as(&mut lexer)?;
                if let Some(statement) = statements.last_mut() {
                    match statement {
                        Statement::Import { imported_as, .. }
                        | Statement::GlobbedImport { imported_as, .. } => {
                            *imported_as = Some(imported_as_new_value);
                        }
                        _ => {
                            return Err(ParsingError::AsStatementUnsupported(InvalidAsStatement {
                                src: generate_source("main.pkl", lexer.source()),
                                at: get_error_location(&mut lexer).into(),
                            }));
                        }
                    }
                } else {
                    return Err(ParsingError::unexpected(&mut lexer));
                }

                continue;
            }

            Ok(PklToken::Identifier) => {
                // match for variable declaration, object declaration and variable assignment
                let identifier = lexer.slice();

                todo!()
            }
            Err(e) => return Err(parse_lexing_error(&mut lexer, e)),
            _ => continue,
        };

        println!("{:?}", statement);
        statements.push(statement);
    }

    Ok(statements)
}

impl ParsingError {
    pub fn eof(lexer: &mut PklLexer<'_>) -> Self {
        ParsingError::UnexpectedEndOfInput(UnexpectedEndOfInputError {
            src: generate_source("main.pkl", lexer.source()),
            at: get_error_location(lexer).into(),
        })
    }
    pub fn unexpected(lexer: &mut PklLexer<'_>) -> Self {
        ParsingError::UnexpectedToken(UnexpectedError {
            src: generate_source("main.pkl", lexer.source()),
            at: get_error_location(&mut lexer).into(),
        })
    }
}
