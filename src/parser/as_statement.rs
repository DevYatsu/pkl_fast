use crate::parser::utils::jump_spaces_and_then;

use super::{
    errors::{
        locating::{generate_source, get_error_location},
        InvalidIdentifierError, UnexpectedError,
    },
    ParsingError, ParsingResult, PklLexer, PklToken, Statement,
};

pub fn parse_as<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<&'source str> {
    let imported_as_new_value = jump_spaces_and_then(lexer, |token, lexer| {
        if let Some(Ok(PklToken::Identifier)) = token {
            let value: &str = lexer.slice();

            Ok(value)
        } else {
            Err(ParsingError::InvalidIdentifier(InvalidIdentifierError {
                src: generate_source("main.pkl", lexer.source()),
                at: get_error_location(lexer).into(),
            }))
        }
    })?;

    return Ok(imported_as_new_value);
}
