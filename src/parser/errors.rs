use miette::{diagnostic, Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;
pub mod locating;

#[derive(Error, Diagnostic, Debug)]
#[diagnostic(
    code(pkl_fast::expected_string),
    help("try putting a string or removing a character")
)]
#[error("Invalid value (expected a string)")] //(expected {expected:?})
pub struct ExpectedStringError {
    #[label("here")]
    pub at: SourceSpan,

    #[source_code]
    pub src: NamedSource<String>,
}
