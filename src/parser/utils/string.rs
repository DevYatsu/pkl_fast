use crate::{
    parser::value::parse_value,
    prelude::{ParsingError, ParsingResult, PklLexer, PklValue},
};

pub fn parse_string_literal<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<&'source str> {
    let value = parse_value(lexer)?;
    println!("{value}");

    if let PklValue::String(value) = value {
        Ok(value)
    } else {
        Err(ParsingError::unexpected(lexer))
    }
}
