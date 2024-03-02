use crate::{
    parser::value::{parse_value, string::StringFragment},
    prelude::{ParsingError, ParsingResult, PklLexer, PklValue},
};

use super::retrieve_next_token;

pub fn parse_string_literal<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<&'source str> {
    let token = retrieve_next_token(lexer)?;
    let value = parse_value(lexer, token)?;

    if let PklValue::String(value) = value {
        if value.is_empty() {
            return Ok("");
        }

        if value.len() != 1
            || value.iter().any(|x| match x {
                StringFragment::RawText(_) => false,
                _ => true,
            })
        {
            return Err(ParsingError::expected_simple_string(lexer));
        }

        match value[0] {
            StringFragment::RawText(value) => Ok(value),
            _ => unreachable!(),
        }
    } else {
        Err(ParsingError::expected_string(lexer))
    }
}
