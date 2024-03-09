use std::{
    io,
    num::{ParseFloatError, ParseIntError},
};

use miette::{diagnostic, Diagnostic, NamedSource, SourceOffset, SourceSpan};
use thiserror::Error;

use crate::lexer::LexingError;

use self::{
    lexing::parse_lexing_error,
    locating::{generate_source, get_error_location, set_error_location},
};

use super::{types::errors::TypeError, PklParser};
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

    #[error(transparent)]
    #[diagnostic(transparent)]
    InvalidInterpolatedExpr(#[from] InterpolatedExprError),
}

#[derive(Error, Diagnostic, Debug)]
#[diagnostic(code(pkl_fast::error::unexpected))]
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
    code(pkl_fast::error::expected_string),
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
    code(pkl_fast::error::expected_identifier),
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
    code(pkl_fast::error::expected_identifier),
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
    code(pkl_fast::error::expected_identifier),
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
    code(pkl_fast::error::invalid_as),
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
#[diagnostic(code(pkl_fast::error::unexpected_end_of_input))]
pub struct UnexpectedEndOfInputError {
    #[label("here")]
    pub at: SourceSpan,

    #[source_code]
    pub src: NamedSource<String>,

    #[help]
    advice: String,
}

#[derive(Error, Diagnostic, Debug)]
#[error("No default value for given type")]
#[diagnostic(code(pkl_fast::error::no_default_value))]
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
    code(pkl_fast::error::unterminated_string),
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
    code(pkl_fast::error::char_escape),
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
    code(pkl_fast::error::unicode_escape),
    help("A unicode escape should have '\\u{{HEX}}' format")
)]
pub struct UnicodeEscapeError {
    #[label("here")]
    pub at: SourceSpan,

    #[source_code]
    pub src: NamedSource<String>,
}

#[derive(Error, Diagnostic, Debug)]
#[error("Invalid Interpolated Expression")]
#[diagnostic(
    code(pkl_fast::error::interpolation),
    help("An interpolated string should have '\\(<expr>)' format. Write a valid expression!")
)]
pub struct InterpolatedExprError {
    #[label("here")]
    pub at: SourceSpan,

    #[source_code]
    pub src: NamedSource<String>,
}

impl ParsingError {
    pub fn eof(parser: &mut PklParser<'_>, advice: &str) -> Self {
        ParsingError::UnexpectedEndOfInput(UnexpectedEndOfInputError {
            src: generate_source("main.pkl", parser.lexer.source()),
            at: get_error_location(&mut parser.lexer),
            advice: format!("Add {advice}? Maybe you should..."),
        })
    }
    pub fn unexpected(parser: &mut PklParser<'_>, advice: String) -> Self {
        ParsingError::UnexpectedToken(UnexpectedError {
            src: generate_source("main.pkl", parser.lexer.source()),
            at: get_error_location(&mut parser.lexer),
            advice,
        })
    }
    pub fn invalid_string(parser: &mut PklParser<'_>) -> Self {
        ParsingError::InvalidString(InvalidStringError {
            src: generate_source("main.pkl", parser.lexer.source()),
            at: get_error_location(&mut parser.lexer),
        })
    }
    pub fn invalid_id(parser: &mut PklParser<'_>) -> Self {
        ParsingError::InvalidIdentifier(InvalidIdentifierError {
            src: generate_source("main.pkl", parser.lexer.source()),
            at: get_error_location(&mut parser.lexer),
        })
    }
    pub fn lexing(parser: &mut PklParser<'_>, e: LexingError) -> Self {
        parse_lexing_error(parser, e)
    }
    pub fn invalid_as_statement(parser: &mut PklParser<'_>) -> Self {
        ParsingError::AsStatementUnsupported(InvalidAsStatement {
            src: generate_source("main.pkl", parser.lexer.source()),
            at: get_error_location(&mut parser.lexer),
        })
    }
    pub fn no_default_value(parser: &mut PklParser<'_>, type_name: &str) -> Self {
        ParsingError::NoDefaultValue(NoDefaultValueError {
            src: generate_source("main.pkl", parser.lexer.source()),
            at: get_error_location(&mut parser.lexer),
            advice: format!("Type `{type_name}` does not possess a default value"),
        })
    }
    pub fn invalid_char_escape(parser: &mut PklParser<'_>, index: usize) -> Self {
        ParsingError::InvalidEscapedChar(EscapedCharError {
            src: generate_source("main.pkl", parser.lexer.source()),
            at: set_error_location(&mut parser.lexer, index, 2),
        })
    }
    pub fn invalid_unicode(parser: &mut PklParser<'_>, index: usize, length: usize) -> Self {
        ParsingError::InvalidUnicodeEscape(UnicodeEscapeError {
            src: generate_source("main.pkl", parser.lexer.source()),
            at: set_error_location(&mut parser.lexer, index, length),
        })
    }
    pub fn invalid_interpolated_expr(
        parser: &mut PklParser<'_>,
        index: usize,
        length: usize,
    ) -> Self {
        let offset: SourceOffset = parser.lexer.span().start.into();
        ParsingError::InvalidInterpolatedExpr(InterpolatedExprError {
            src: generate_source("main.pkl", parser.lexer.source()),
            at: SourceSpan::new((offset.offset() + index).into(), length),
        })
    }

    fn unexpected_token(parser: &mut PklParser<'_>, expected: &str) -> Self {
        ParsingError::UnexpectedToken(UnexpectedError {
            src: generate_source("main.pkl", parser.lexer.source()),
            at: get_error_location(&mut parser.lexer),
            advice: format!("Expected `{expected}`"),
        })
    }

    pub fn expected_simple_string(parser: &mut PklParser<'_>) -> Self {
        ParsingError::UnexpectedToken(UnexpectedError {
            src: generate_source("main.pkl", parser.lexer.source()),
            at: get_error_location(&mut parser.lexer),
            advice: format!(
                "Expected a simple `String` without characters escape or interpolation"
            ),
        })
    }
    pub fn expected_string(parser: &mut PklParser<'_>) -> Self {
        Self::unexpected_token(parser, "String")
    }
    pub fn expected_identifier(parser: &mut PklParser<'_>) -> Self {
        Self::unexpected_token(parser, "Identifier")
    }
    pub fn expected_expression(parser: &mut PklParser<'_>) -> Self {
        Self::unexpected_token(parser, "Expression")
    }

    pub fn get_at(&self) -> SourceSpan {
        match self {
            ParsingError::ParseIntError(_) => {
                unreachable!("No source span available for ParseIntError")
            }
            ParsingError::IoError(_) => unreachable!("No source span available for IoError"),
            ParsingError::ParseFloatError(_) => {
                unreachable!("No source span available for ParseFloatError")
            }
            ParsingError::LexingError(_) => {
                unreachable!("No source span available for LexingError")
            }
            ParsingError::TypeError(_) => unreachable!("No source span available for TypeError"),
            ParsingError::UnexpectedToken(e) => e.at,
            ParsingError::InvalidString(e) => e.at,
            ParsingError::InvalidInt(e) => e.at,
            ParsingError::InvalidFloat(e) => e.at,
            ParsingError::InvalidIdentifier(e) => e.at,
            ParsingError::AsStatementUnsupported(e) => e.at,
            ParsingError::UnexpectedEndOfInput(e) => e.at,
            ParsingError::UnterminatedString(e) => e.at,
            ParsingError::NoDefaultValue(e) => e.at,
            ParsingError::InvalidEscapedChar(e) => e.at,
            ParsingError::InvalidUnicodeEscape(e) => e.at,
            ParsingError::InvalidInterpolatedExpr(e) => e.at,
        }
    }

    pub fn with_attributes(self, src: NamedSource<String>, at: SourceSpan) -> Self {
        match self {
            ParsingError::ParseIntError(_)
            | ParsingError::IoError(_)
            | ParsingError::ParseFloatError(_) => {
                unreachable!("No need to implement it for std errors")
            }
            ParsingError::LexingError(_) => {
                unreachable!("No need to implement it for lexing errors")
            }
            ParsingError::TypeError(_) => unreachable!("No need to implement it for type errors"),
            ParsingError::UnexpectedToken(e) => ParsingError::UnexpectedToken(UnexpectedError {
                src,
                at,
                advice: e.advice,
            }),
            ParsingError::InvalidString(_) => {
                ParsingError::InvalidString(InvalidStringError { src, at })
            }
            ParsingError::InvalidInt(_) => ParsingError::InvalidInt(InvalidIntError { src, at }),
            ParsingError::InvalidFloat(_) => {
                ParsingError::InvalidFloat(InvalidFloatError { src, at })
            }
            ParsingError::InvalidIdentifier(_) => {
                ParsingError::InvalidIdentifier(InvalidIdentifierError { src, at })
            }
            ParsingError::AsStatementUnsupported(_) => {
                ParsingError::AsStatementUnsupported(InvalidAsStatement { src, at })
            }
            ParsingError::UnexpectedEndOfInput(e) => {
                ParsingError::UnexpectedEndOfInput(UnexpectedEndOfInputError {
                    src,
                    at,
                    advice: e.advice,
                })
            }
            ParsingError::UnterminatedString(_) => {
                ParsingError::UnterminatedString(UnterminatedStringError { src, at })
            }
            ParsingError::NoDefaultValue(e) => ParsingError::NoDefaultValue(NoDefaultValueError {
                src,
                at,
                advice: e.advice,
            }),
            ParsingError::InvalidEscapedChar(_) => {
                ParsingError::InvalidEscapedChar(EscapedCharError { src, at })
            }
            ParsingError::InvalidUnicodeEscape(_) => {
                ParsingError::InvalidUnicodeEscape(UnicodeEscapeError { src, at })
            }
            ParsingError::InvalidInterpolatedExpr(_) => {
                ParsingError::InvalidInterpolatedExpr(InterpolatedExprError { src, at })
            }
        }
    }
}
