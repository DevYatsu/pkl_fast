use crate::lexer::PklToken;
use miette::NamedSource;

use super::{
    errors::{locating::get_error_location, InvalidStringError},
    utils::jump_spaces_and_then,
    ParsingError, ParsingResult, PklLexer, Statement,
};

pub fn parse_extends<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<Statement<'source>> {
    let predicate = |token, lexer: &mut PklLexer<'source>| {
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
    };

    jump_spaces_and_then(lexer, &predicate)
}
