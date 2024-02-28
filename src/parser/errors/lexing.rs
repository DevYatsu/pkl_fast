use super::locating::{generate_source, get_error_location};
use super::{InvalidFloatError, InvalidIntError, ParsingError, PklLexer, UnexpectedError};
use crate::lexer::LexingError;

pub fn parse_lexing_error<'source>(
    lexer: &mut PklLexer<'source>,
    err: LexingError,
) -> ParsingError {
    match err {
        LexingError::InvalidInteger => ParsingError::InvalidInt(InvalidIntError {
            src: generate_source("main.pkl", lexer.source()),
            at: get_error_location(lexer).into(),
        }),
        LexingError::InvalidFloat => ParsingError::InvalidFloat(InvalidFloatError {
            src: generate_source("main.pkl", lexer.source()),
            at: get_error_location(lexer).into(),
        }),
        LexingError::UnknownError => ParsingError::UnexpectedToken(UnexpectedError {
            src: generate_source("main.pkl", lexer.source()),
            at: get_error_location(lexer).into(),
        }),
    }
}
