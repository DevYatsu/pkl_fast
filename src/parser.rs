use logos::Lexer;
use miette::{diagnostic, Diagnostic};
use pkl_fast::lexer::PklToken;
use thiserror::Error;

use self::errors::ExpectedStringError;

mod amends;
mod constant;
mod errors;
mod import;
mod module;
mod utils;

pub type ParsingResult<T> = miette::Result<T, ParsingError>;
pub type PklLexer<'source> = Lexer<'source, PklToken>;

#[derive(Debug)]
pub enum Statement<'a> {
    Import(&'a str),
    GlobbedImport(&'a str),
    Amends(&'a str),
    Module(&'a str),
}

#[derive(Error, Diagnostic, Debug)]
pub enum ParsingError {
    #[error("Invalid syntax")]
    InvalidSyntax(String),

    #[error("Unexpected token `{0}` found")]
    UnexpectedToken(String),

    #[error(transparent)]
    #[diagnostic(transparent)]
    ExpectedString(#[from] ExpectedStringError),
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
            _ => continue,
        };

        println!("{:?}", statement);
        statements.push(statement);
    }

    Ok(statements)
}
