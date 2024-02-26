use crate::prelude::{ParsingError, ParsingResult, PklLexer, PklToken};

use super::Statement;

pub fn parse_amends<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<Statement<'source>> {
    let token = lexer.next();

    if let Some(Ok(PklToken::StringLiteral)) = token {
        let raw_value = lexer.slice(); // retrieve value with quotes: "value"
        let value = &raw_value[1..raw_value.len() - 1];
        Ok(Statement::Amends(value))
    } else {
        if token.is_some() {
            Err(ParsingError::invalid_string(lexer))
        } else {
            Err(ParsingError::eof(lexer))
        }
    }
}
