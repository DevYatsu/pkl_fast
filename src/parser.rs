use crate::parser::errors::parse_lexing_error;

use self::{
    errors::{
        InvalidFloatError, InvalidIdentifierError, InvalidIntError, InvalidStringError,
        UnexpectedError,
    },
    import::ImportClause,
};
use logos::Lexer;
use miette::{diagnostic, Diagnostic};
use pkl_fast::lexer::{LexingError, PklToken};
use thiserror::Error;

mod amends;
mod as_statement;
mod constant;
mod extends;
mod import;
mod module;

mod errors;
mod utils;

pub type ParsingResult<T> = miette::Result<T, ParsingError>;
pub type PklLexer<'source> = Lexer<'source, PklToken>;

#[derive(Debug)]
pub enum Statement<'a> {
    Import {
        clause: ImportClause<'a>,
        imported_as: Option<&'a str>,
    },
    GlobbedImport {
        clause: ImportClause<'a>,
        imported_as: Option<&'a str>,
    },
    Amends(&'a str),
    Module(&'a str),
    Extends(&'a str),
}

#[derive(Error, Diagnostic, Debug)]
pub enum ParsingError {
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
}

pub fn parse<'source>(mut lexer: PklLexer<'source>) -> ParsingResult<Vec<Statement<'source>>> {
    let mut statements = vec![];

    while let Some(token) = lexer.next() {
        println!("{:?}", token);
        let statement = match token {
            Ok(PklToken::Import) => import::parse_import(&mut lexer)?,
            Ok(PklToken::GlobbedImport) => import::parse_globbed_import(&mut lexer)?,
            Ok(PklToken::Amends) => amends::parse_amends(&mut lexer)?,
            Ok(PklToken::Module) => module::parse_module(&mut lexer)?,
            Ok(PklToken::Extends) => extends::parse_extends(&mut lexer)?,
            Ok(PklToken::As) => {
                let imported_as_new_value = as_statement::parse_as(&mut lexer, &statements)?;
                if let Some(statement) = statements.last_mut() {
                    match statement {
                        Statement::Import { imported_as, .. }
                        | Statement::GlobbedImport { imported_as, .. } => {
                            *imported_as = Some(imported_as_new_value);
                        }
                        _ => todo!(),
                    }
                }
                continue;
            }
            Err(e) => return Err(parse_lexing_error(&mut lexer, e)),
            _ => continue,
        };

        println!("{:?}", statement);
        statements.push(statement);
    }

    Ok(statements)
}
