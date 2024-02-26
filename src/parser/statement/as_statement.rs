use crate::{
    parser::errors::{
        locating::{generate_source, get_error_location},
        InvalidIdentifierError,
    },
    prelude::{ParsingError, ParsingResult, PklLexer, PklToken},
};

use super::Statement;

pub fn parse_as<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<&'source str> {
    let token = lexer.next();

    if let Some(Ok(PklToken::Identifier)) = token {
        let value: &str = lexer.slice();

        Ok(value)
    } else {
        Err(ParsingError::InvalidIdentifier(InvalidIdentifierError {
            src: generate_source("main.pkl", lexer.source()),
            at: get_error_location(lexer).into(),
        }))
    }
}
