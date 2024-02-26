use crate::{
    parser::errors::{
        locating::{generate_source, get_error_location},
        InvalidIdentifierError,
    },
    prelude::{ParsingError, ParsingResult, PklLexer, PklToken},
};

pub fn parse_as<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<&'source str> {
    let token = lexer.next();

    if let Some(Ok(PklToken::Identifier)) = token {
        Ok(lexer.slice())
    } else {
        if token.is_some() {
            Err(ParsingError::invalid_id(lexer))
        } else {
            Err(ParsingError::eof(lexer))
        }
    }
}
