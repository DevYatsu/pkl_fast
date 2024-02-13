use self::{
    errors::{InvalidIdentifierError, InvalidStringError, UnexpectedError},
    import::ImportClause,
};
use logos::Lexer;
use miette::{diagnostic, Diagnostic};
use pkl_fast::lexer::PklToken;
use thiserror::Error;

mod amends;
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
    GlobbedImport(&'a str),
    Amends(&'a str),
    Module(&'a str),
    Extends(&'a str),
}

#[derive(Error, Diagnostic, Debug)]
pub enum ParsingError {
    #[error("Invalid syntax")]
    InvalidSyntax(String),

    #[error(transparent)]
    #[diagnostic(transparent)]
    UnexpectedToken(#[from] UnexpectedError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    InvalidString(#[from] InvalidStringError),

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
            _ => continue,
        };

        println!("{:?}", statement);
        statements.push(statement);
    }

    Ok(statements)
}
