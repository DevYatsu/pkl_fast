use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum TypeError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    MissingTypeGeneric(#[from] MissingTypeGenericError),
}

#[derive(Error, Diagnostic, Debug)]
#[error("Missing a type generic")]
#[diagnostic(
    code(pkl_fast::unexpected_end_of_input),
    help("Add a type Generic to the type")
)]
pub struct MissingTypeGenericError {
    #[label("here")]
    pub at: SourceSpan,

    #[source_code]
    pub src: NamedSource<String>,
}
