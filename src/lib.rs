//! `pkl_fast` is a Rust library designed to efficiently work with Apple's Pkl file format. This library provides utilities for lexing and parsing (into statements).
//!
//! > **Note: In the future I intend to add utilities to generate a symbol table from the statements.
//!
//! ## Table of Contents
//!
//! - [Installing](#installing)
//! - [Lexer](#lexer)
//! - [Parser](#parser)
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
//!
//! ## Example
//!
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
