use pkl_fast::lexer::PklToken;

use super::{
    errors::{
        locating::{generate_source, get_error_location},
        InvalidStringError,
    },
    utils::jump_spaces_and_then,
    ParsingError, ParsingResult, PklLexer, Statement,
};

pub fn parse_amends<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<Statement<'source>> {
    let predicate = |token, lexer: &mut PklLexer<'source>| {
        if let Some(Ok(PklToken::StringLiteral)) = token {
            let raw_value = lexer.slice(); // retrieve value with quotes: "value"

            let value = &raw_value[1..raw_value.len() - 1];
            Ok(Statement::Amends(value))
        } else {
            Err(ParsingError::InvalidString(InvalidStringError {
                src: generate_source("main.pkl", lexer.source()),
                at: get_error_location(lexer).into(),
            }))
        }
    };

    jump_spaces_and_then(lexer, &predicate)
}
