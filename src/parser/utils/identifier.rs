use crate::prelude::{ParsingError, ParsingResult, PklLexer, PklToken};

pub fn parse_identifier<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<&'source str> {
    let token = lexer.next();

    if let Some(Ok(PklToken::Identifier)) = token {
        let id: &str = lexer.slice();
        Ok(id)
    } else {
        if token.is_some() {
            Err(ParsingError::invalid_id(lexer))
        } else {
            Err(ParsingError::eof(lexer))
        }
    }
}
