use miette::NamedSource;

use crate::{
    parser::errors::{locating::get_error_location, InvalidStringError},
    prelude::{ParsingError, ParsingResult, PklLexer, PklToken},
};

use super::Statement;
pub fn parse_extends<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<Statement<'source>> {
    let token = lexer.next();

    if let Some(Ok(PklToken::StringLiteral)) = token {
        let raw_value = lexer.slice(); // retrieve value with quotes: "value"
        let value = &raw_value[1..raw_value.len() - 1];
        Ok(Statement::Extends(value))
    } else {
        Err(ParsingError::InvalidString(InvalidStringError {
            src: NamedSource::new("main.pkl", lexer.source().to_string()),
            at: get_error_location(lexer).into(),
        }))
    }
}
