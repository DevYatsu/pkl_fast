use logos::Lexer;
use pkl_fast::lexer::PklToken;

mod amends;
mod constant;
mod import;
mod module;
mod utils;

pub type ParsingResult<T> = std::result::Result<T, ParsingError>;
pub type PklLexer<'source> = Lexer<'source, PklToken>;

#[derive(Debug)]
pub enum Statement<'a> {
    Import(&'a str),
    GlobbedImport(&'a str),
    Amends(&'a str),
    Module(&'a str),
}

#[derive(Debug)]
pub enum ParsingError {
    InvalidSyntax(String),
    UnexpectedToken(String),
    ExpectedSpace,
    Expected(String),
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

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::InvalidSyntax(message) => {
                write!(f, "Invalid syntax: {}", message)
            }
            ParsingError::UnexpectedToken(token) => {
                write!(f, "Unexpected token: {}", token)
            }
            ParsingError::ExpectedSpace => {
                write!(f, "Expected a whitespace")
            }
            ParsingError::Expected(message) => {
                write!(f, "{}", message)
            }
        }
    }
}
