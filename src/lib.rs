//! `pkl_fast` is a Rust library designed to efficiently work with Apple's Pkl file format. This library provides utilities for lexing and parsing (into statements).
//!
//! > **Note: In the future I intend to add utilities to generate a symbol table from the statements.
//!
//! ## Table of Contents
//!
//! - [Installing](#installing)
//! - [Example](#example)
//! - [License](#license)
//!
//!
//! ## Installing
//!
//! ```sh
//! $ cargo add pkl_fast
//! ```
//!
//! ## Example
//!
//! ```rust
//! use pkl_fast::prelude::{parse, lex, ParsingResult};
//! use std::fs;
//!
//! fn main() -> ParsingResult<()> {
//!     let source: String = fs::read_to_string("file.pkl").unwrap_or("".to_owned());
//!     let tokens = lex(&source);
//!     let statements = parse(tokens)?;
//!     // statements now contains a representation of the source string as a Vec<Statements>
//!
//!     Ok(())
//! }
//!
//! ```
//!
//! ## License
//!
//! `pkl_fast` is released to the Rust community under the [Apache license
//! 2.0](./LICENSE).
//!
//! It also includes code taken from [`miette`](https://github.com/zkat/miette),
//! and some from [`thiserror`](https://github.com/dtolnay/thiserror) and [`logos`](https://github.com/maciejhirsz/logos), which are Apache licensed.

pub mod lexer;
pub mod parser;
pub use logos::Logos;

pub mod prelude {
    pub use crate::lexer::PklToken;
    pub use crate::parser::PklLexer;
    pub use crate::parser::{
        errors::ParsingError, parse, statement::Statement, value::PklValue, ParsingResult,
        PklParser,
    };
    pub use logos::Logos;

    pub fn lex<'source>(source: &'source str) -> PklLexer<'source> {
        PklToken::lexer(source)
    }
}
