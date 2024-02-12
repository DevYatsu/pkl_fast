use logos::Lexer;
use pkl_fast::lexer::PklToken;

mod amends;
mod constant;
mod import;
mod utils;

pub type ParsingResult<T> = std::result::Result<T, ParsingError>;

#[derive(Debug)]
pub enum Statement<'a> {
    Import(&'a str),
    GlobbedImport(&'a str),
    Amends(&'a str),
}

#[derive(Debug)]
pub enum ParsingError {
    InvalidSyntax(String),
    UnexpectedToken(String),
    ExpectedSpace,
}

pub fn parse<'source>(
    mut lexer: Lexer<'source, PklToken>,
) -> ParsingResult<Vec<Statement<'source>>> {
    let mut statements = vec![];

    while let Some(token) = lexer.next() {
        println!("{:?}", token);
        let statement = match token {
            Ok(PklToken::Import) => import::parse_import(&mut lexer),
            Ok(PklToken::GlobbedImport) => import::parse_globbed_import(&mut lexer),
            Ok(PklToken::Amends) => amends::parse_amends(&mut lexer),
            _ => continue,
        };

        println!("{:?}", statement);
        statements.push(statement.unwrap());
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
        }
    }
}
