use crate::lexer::LexingError;
use miette::{diagnostic, Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

use self::locating::{generate_source, get_error_location};

use super::{ParsingError, PklLexer};
pub mod locating;

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
        LexingError::NonAsciiCharacter => ParsingError::UnexpectedToken(UnexpectedError {
            src: generate_source("main.pkl", lexer.source()),
            at: get_error_location(lexer).into(),
        }),
    }
}

#[derive(Error, Diagnostic, Debug)]
#[diagnostic(code(pkl_fast::unexpected), help("try removing a character"))]
#[error("Unexpected character")]
pub struct UnexpectedError {
    #[label("here")]
    pub at: SourceSpan,

    #[source_code]
    pub src: NamedSource<String>,
}

#[derive(Error, Diagnostic, Debug)]
#[diagnostic(
    code(pkl_fast::expected_string),
    help("try putting a string or removing a character")
)]
#[error("Invalid value (expected a string)")]
pub struct InvalidStringError {
    #[label("here")]
    pub at: SourceSpan,

    #[source_code]
    pub src: NamedSource<String>,
}

#[derive(Error, Diagnostic, Debug)]
#[diagnostic(
    code(pkl_fast::expected_identifier),
    help("valid identifier = alphanumeric word (except first letter not numeric)")
)]
#[error("Expected a valid identifier")]
pub struct InvalidIdentifierError {
    #[label("here")]
    pub at: SourceSpan,

    #[source_code]
    pub src: NamedSource<String>,
}

#[derive(Error, Diagnostic, Debug)]
#[diagnostic(
    code(pkl_fast::expected_identifier),
    help("write a valid integer (ex: 123, -123_000, 0x012AFF, 0b00010111, 0o755)")
)]
#[error("Expected a valid integer")]
pub struct InvalidIntError {
    #[label("here")]
    pub at: SourceSpan,

    #[source_code]
    pub src: NamedSource<String>,
}

#[derive(Error, Diagnostic, Debug)]
#[diagnostic(
    code(pkl_fast::expected_identifier),
    help("write a valid float (ex: .23, -1.23, 1.23e2, 1.23e-2)")
)]
#[error("Expected a valid float")]
pub struct InvalidFloatError {
    #[label("here")]
    pub at: SourceSpan,

    #[source_code]
    pub src: NamedSource<String>,
}

#[derive(Error, Diagnostic, Debug)]
#[diagnostic(
    code(pkl_fast::invalid_as),
    help("The 'as' keyword is only supported in 'import' or 'import*' statements")
)]
#[error("Invalid 'as' statement")]
pub struct InvalidAsStatement {
    #[label("here")]
    pub at: SourceSpan,

    #[source_code]
    pub src: NamedSource<String>,
}

#[derive(Error, Diagnostic, Debug)]
#[error("Unexpected end of input")]
#[diagnostic(
    code(pkl_fast::unexpected_end_of_input),
    help("Try putting a string or removing a character")
)]
pub struct UnexpectedEndOfInputError {
    #[label("here")]
    pub at: SourceSpan,

    #[source_code]
    pub src: NamedSource<String>,
}
