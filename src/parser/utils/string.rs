use crate::prelude::{ParsingError, ParsingResult, PklLexer, PklToken};

pub fn parse_string_literal<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<&'source str> {
    let token = lexer.next();

    if let Some(Ok(PklToken::StringLiteral)) = token {
        let raw_value: &str = lexer.slice(); // retrieve value with quotes: "value"
        let value = &raw_value[1..raw_value.len() - 1];
        Ok(value)
    } else {
        if token.is_some() {
            Err(ParsingError::invalid_string(lexer))
        } else {
            Err(ParsingError::eof(lexer))
        }
    }
}
