use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

use crate::{
    parser::errors::locating::{generate_source, get_error_location},
    prelude::PklLexer,
};

#[derive(Error, Diagnostic, Debug)]
pub enum TypeError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    ExpectedGenerics(#[from] ExpectedGenericsError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    CannotGiveRestriction(#[from] CannotGiveRestrictionError),
}

impl TypeError {
    pub fn no_restrictions_type<'source>(lexer: &mut PklLexer<'source>, advice: String) -> Self {
        TypeError::CannotGiveRestriction(CannotGiveRestrictionError {
            at: get_error_location(lexer),
            src: generate_source("main.pkl", lexer.source()),
            advice,
        })
    }

    pub fn expect_generics<'source>(
        lexer: &mut PklLexer<'source>,
        number: u8,
        type_name: &'source str,
    ) -> Self {
        TypeError::ExpectedGenerics(ExpectedGenericsError {
            at: get_error_location(lexer),
            src: generate_source("main.pkl", lexer.source()),
            advice: format!("Expected `{}` generics for type `{}`!", number, type_name),
        })
    }
}

#[derive(Error, Diagnostic, Debug)]
#[error("Invalid generics provided.")]
#[diagnostic(code(pkl_fast::types::generic::expected_1))]
pub struct ExpectedGenericsError {
    #[label("here")]
    pub at: SourceSpan,

    #[source_code]
    pub src: NamedSource<String>,

    #[help]
    pub advice: String,
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

#[derive(Error, Diagnostic, Debug)]
#[error("Invalid type annotation, This type does not support restrictions.")]
#[diagnostic(code(pkl_fast::types::restrictions))]
pub struct CannotGiveRestrictionError {
    #[label("here")]
    pub at: SourceSpan,

    #[source_code]
    pub src: NamedSource<String>,

    #[help]
    pub advice: String,
}
