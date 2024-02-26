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
//! $ cargo add miette
//! ```
//!
//! ## Example
//!
//! ```rust
//! use pkl_fast::{Logos, lexer::PklToken, parser::{parse, ParsingError, Statement}};
//!
//! fn main() -> ParsingError<()> {
//!     let source: String = fs::read_to_string("file.pkl")?;
//!     let tokens: Lexer<PklToken> = PklToken::lexer(source);
//!     let statements: Vec<Statement<'_>> = parse(tokens)?;
//!     // statements now contains a representation of the source string as an AST (statements)
//! }
//!
//! ```
//!
//! ## License
//!
//! `miette` is released to the Rust community under the [Apache license
//! 2.0](./LICENSE).
//!
//! It also includes code taken from [`miette`](https://github.com/zkat/miette),
//! and some from [`thiserror`](https://github.com/dtolnay/thiserror) and [`logos`](https://github.com/maciejhirsz/logos), which are Apache licensed.

pub mod lexer;
pub mod parser;
pub use logos::Logos;

mod prelude {
    pub use crate::lexer::PklToken;
    pub use crate::parser::{parse, ParsingError, ParsingResult, value::PklValue};
}