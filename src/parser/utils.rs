use crate::prelude::{ParsingError, ParsingResult, PklLexer, PklToken};
mod identifier;
mod string;

pub fn retrieve_next_token<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<PklToken> {
    let token = lexer.next();

    if token.is_none() {
        return Err(ParsingError::eof(lexer));
    }

    match token.unwrap() {
        Err(e) => Err(ParsingError::lexing(lexer, e)),
        token => Ok(token?),
    }
}

pub fn parse_token<'source>(
    lexer: &mut PklLexer<'source>,
    target_token: PklToken,
) -> ParsingResult<()> {
    let token = lexer.next();

    if token.is_none() {
        return Err(ParsingError::eof(lexer));
    }

    match token.unwrap() {
        Err(e) => Err(ParsingError::lexing(lexer, e))?,
        Ok(token) if token == target_token => Ok(()),
        _ => Err(ParsingError::unexpected(lexer))?,
    }
}

pub use identifier::parse_identifier;
pub use string::parse_string_literal;
