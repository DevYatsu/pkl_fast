use crate::prelude::{ParsingError, ParsingResult, PklLexer, PklToken};

pub fn parse_string_literal<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<&'source str> {
    let token = lexer.next();

    if let Some(Ok(PklToken::StringLiteral(value))) = token {
        Ok(value)
    } else {
        if token.is_some() {
            Err(ParsingError::invalid_string(lexer))
        } else {
            Err(ParsingError::eof(lexer))
        }
    }
}
