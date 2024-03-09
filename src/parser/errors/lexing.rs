use super::locating::{generate_source, get_error_location};
use super::{InvalidFloatError, InvalidIntError, ParsingError, UnexpectedError};
use crate::lexer::LexingError;
use crate::prelude::PklParser;

pub fn parse_lexing_error<'source>(
    parser: &mut PklParser<'source>,
    err: LexingError,
) -> ParsingError {
    match err {
        LexingError::InvalidInteger => ParsingError::InvalidInt(InvalidIntError {
            src: generate_source("main.pkl", parser.lexer.source()),
            at: get_error_location(&mut parser.lexer).into(),
        }),
        LexingError::InvalidFloat => ParsingError::InvalidFloat(InvalidFloatError {
            src: generate_source("main.pkl", parser.lexer.source()),
            at: get_error_location(&mut parser.lexer).into(),
        }),
        LexingError::UnknownError => ParsingError::UnexpectedToken(UnexpectedError {
            src: generate_source("main.pkl", parser.lexer.source()),
            at: get_error_location(&mut parser.lexer).into(),
            advice: "Error is unknown, contact the library maintainers to report it! (the error takes place in the lexer)".to_string(),
        }),
    }
}
