use std::{
    io,
    num::{ParseFloatError, ParseIntError},
};

use crate::lexer::LexingError;
use miette::{diagnostic, Diagnostic, NamedSource, SourceOffset, SourceSpan};
use thiserror::Error;

use self::{
    lexing::parse_lexing_error,
    locating::{generate_source, get_error_location},
};

use super::{types::errors::TypeError, PklLexer};
pub mod lexing;
pub mod locating;

#[derive(Error, Diagnostic, Debug)]
pub enum ParsingError {
    #[error(transparent)]
    #[diagnostic(code(pkl::io_error))]
    IoError(#[from] io::Error),

    #[error(transparent)]
    #[diagnostic(code(num::parse_int))]
    ParseIntError(#[from] ParseIntError),
    #[error(transparent)]
    #[diagnostic(code(num::parse_float))]
    ParseFloatError(#[from] ParseFloatError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    LexingError(#[from] LexingError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    TypeError(#[from] TypeError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    UnexpectedToken(#[from] UnexpectedError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    InvalidString(#[from] InvalidStringError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    InvalidInt(#[from] InvalidIntError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    InvalidFloat(#[from] InvalidFloatError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    InvalidIdentifier(#[from] InvalidIdentifierError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    AsStatementUnsupported(#[from] InvalidAsStatement),

    #[error(transparent)]
    #[diagnostic(transparent)]
    UnexpectedEndOfInput(#[from] UnexpectedEndOfInputError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    UnterminatedString(#[from] UnterminatedStringError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    NoDefaultValue(#[from] NoDefaultValueError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    InvalidEscapedChar(#[from] EscapedCharError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    InvalidUnicodeEscape(#[from] UnicodeEscapeError),
}

#[derive(Error, Diagnostic, Debug)]
#[diagnostic(code(pkl_fast::unexpected))]
#[error("Unexpected token")]
pub struct UnexpectedError {
    #[label("here")]
    pub at: SourceSpan,

    #[source_code]
    pub src: NamedSource<String>,

    #[help]
    advice: String,
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

#[derive(Error, Diagnostic, Debug)]
#[error("No default value for given type")]
#[diagnostic(code(pkl_fast::unexpected_end_of_input))]
pub struct NoDefaultValueError {
    #[label("here")]
    pub at: SourceSpan,

    #[source_code]
    pub src: NamedSource<String>,

    #[help]
    advice: String,
}

#[derive(Error, Diagnostic, Debug)]
#[error("String unexpected never ends")]
#[diagnostic(
    code(pkl_fast::unexpected_end_of_input),
    help("Add a `\"` at the end of the input")
)]
pub struct UnterminatedStringError {
    #[label("here")]
    pub at: SourceSpan,

    #[source_code]
    pub src: NamedSource<String>,
}

#[derive(Error, Diagnostic, Debug)]
#[error("Invalid Character Escape")]
#[diagnostic(
    code(pkl_fast::unexpected_end_of_input),
    help("Valid character escape: \\n, \\t, \\r, \\\", \\\\")
)]
pub struct EscapedCharError {
    #[label("here")]
    pub at: SourceSpan,

    #[source_code]
    pub src: NamedSource<String>,
}

#[derive(Error, Diagnostic, Debug)]
#[error("Invalid Unicode Escape")]
#[diagnostic(
    code(pkl_fast::unexpected_end_of_input),
    help("A unicode escape should have '\\u{{HEX}}' format")
)]
pub struct UnicodeEscapeError {
    #[label("here")]
    pub at: SourceSpan,

    #[source_code]
    pub src: NamedSource<String>,
}

impl ParsingError {
    pub fn eof(lexer: &mut PklLexer<'_>) -> Self {
        ParsingError::UnexpectedEndOfInput(UnexpectedEndOfInputError {
            src: generate_source("main.pkl", lexer.source()),
            at: get_error_location(lexer),
        })
    }
    pub fn unexpected(lexer: &mut PklLexer<'_>, advice: String) -> Self {
        ParsingError::UnexpectedToken(UnexpectedError {
            src: generate_source("main.pkl", lexer.source()),
            at: get_error_location(lexer),
            advice,
        })
    }
    pub fn invalid_string(lexer: &mut PklLexer<'_>) -> Self {
        ParsingError::InvalidString(InvalidStringError {
            src: generate_source("main.pkl", lexer.source()),
            at: get_error_location(lexer),
        })
    }
    pub fn invalid_id(lexer: &mut PklLexer<'_>) -> Self {
        ParsingError::InvalidIdentifier(InvalidIdentifierError {
            src: generate_source("main.pkl", lexer.source()),
            at: get_error_location(lexer),
        })
    }
    pub fn lexing(lexer: &mut PklLexer<'_>, e: LexingError) -> Self {
        parse_lexing_error(lexer, e)
    }
    pub fn invalid_as_statement(lexer: &mut PklLexer<'_>) -> Self {
        ParsingError::AsStatementUnsupported(InvalidAsStatement {
            src: generate_source("main.pkl", lexer.source()),
            at: get_error_location(lexer),
        })
    }
    pub fn no_default_value(lexer: &mut PklLexer<'_>, type_name: &str) -> Self {
        ParsingError::NoDefaultValue(NoDefaultValueError {
            src: generate_source("main.pkl", lexer.source()),
            at: get_error_location(lexer),
            advice: format!("Type `{type_name}` does not possess a default value"),
        })
    }
    pub fn invalid_char_escape(lexer: &mut PklLexer<'_>, index: usize) -> Self {
        let offset: SourceOffset = lexer.span().start.into();

        ParsingError::InvalidEscapedChar(EscapedCharError {
            src: generate_source("main.pkl", lexer.source()),
            at: SourceSpan::new((offset.offset() + index).into(), 2),
        })
    }
    pub fn invalid_unicode(lexer: &mut PklLexer<'_>, index: usize, length: usize) -> Self {
        let offset: SourceOffset = lexer.span().start.into();
        ParsingError::InvalidUnicodeEscape(UnicodeEscapeError {
            src: generate_source("main.pkl", lexer.source()),
            at: SourceSpan::new((offset.offset() + index).into(), length),
        })
    }

    fn unexpected_token(lexer: &mut PklLexer<'_>, expected: &str) -> Self {
        ParsingError::UnexpectedToken(UnexpectedError {
            src: generate_source("main.pkl", lexer.source()),
            at: get_error_location(lexer),
            advice: format!("Expected `{expected}`"),
        })
    }

    pub fn expected_simple_string(lexer: &mut PklLexer<'_>) -> Self {
        ParsingError::UnexpectedToken(UnexpectedError {
            src: generate_source("main.pkl", lexer.source()),
            at: get_error_location(lexer),
            advice: format!(
                "Expected a simple `String` without characters escape or interpolation"
            ),
        })
    }
    pub fn expected_string(lexer: &mut PklLexer<'_>) -> Self {
        Self::unexpected_token(lexer, "String")
    }
    pub fn expected_identifier(lexer: &mut PklLexer<'_>) -> Self {
        Self::unexpected_token(lexer, "Identifier")
    }
    pub fn expected_expression(lexer: &mut PklLexer<'_>) -> Self {
        Self::unexpected_token(lexer, "Expression")
    }
}
