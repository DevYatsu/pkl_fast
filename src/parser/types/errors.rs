use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum TypeError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Expected1Generic(#[from] Expected1GenericError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    Expected2Generic(#[from] Expected2GenericError),
}

#[derive(Error, Diagnostic, Debug)]
#[error("Expected an Enum that takes 1 generic argument")]
#[diagnostic(
    code(pkl_fast::types::generic::expected_1),
    help("Check the number of generic types")
)]
pub struct Expected1GenericError {
    #[label("here")]
    pub at: SourceSpan,

    #[source_code]
    pub src: NamedSource<String>,
}

#[derive(Error, Diagnostic, Debug)]
#[error("Expected an Enum that takes 2 generic arguments")]
#[diagnostic(
    code(pkl_fast::types::generic::expected_1),
    help("Check the number of generic types")
)]
pub struct Expected2GenericError {
    #[label("here")]
    pub at: SourceSpan,

    #[source_code]
    pub src: NamedSource<String>,
}
