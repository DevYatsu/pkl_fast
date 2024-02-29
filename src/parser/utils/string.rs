use crate::{
    parser::value::parse_value,
    prelude::{ParsingError, ParsingResult, PklLexer, PklValue},
};

use super::retrieve_next_token;

pub fn parse_string_literal<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<&'source str> {
    let token = retrieve_next_token(lexer)?;
    let value = parse_value(lexer, token)?;

    if let PklValue::String(value) = value {
        Ok(value)
    } else {
        Err(ParsingError::unexpected(lexer))
    }
}
