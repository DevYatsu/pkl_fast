use miette::{diagnostic, Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;
pub mod locating;

#[derive(Error, Diagnostic, Debug)]
#[diagnostic(code(pkl_fast::expected_string), help("try removing a character"))]
#[error("Unexpected character")] //(expected {expected:?})
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
#[error("Invalid value (expected a string)")] //(expected {expected:?})
pub struct InvalidStringError {
    #[label("here")]
    pub at: SourceSpan,

    #[source_code]
    pub src: NamedSource<String>,
}

#[derive(Error, Diagnostic, Debug)]
#[diagnostic(
    code(pkl_fast::expected_string),
    help("valid identifier = alphanumeric word (except first letter not numeric)")
)]
#[error("Expected a valid identifier")] //(expected {expected:?})
pub struct InvalidIdentifierError {
    #[label("here")]
    pub at: SourceSpan,

    #[source_code]
    pub src: NamedSource<String>,
}
