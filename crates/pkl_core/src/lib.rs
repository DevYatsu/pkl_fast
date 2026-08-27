//! `pkl_core` — Embed the [Pkl configuration language](https://pkl-lang.org) into Rust apps.
//!
//! This crate provides evaluation, deserialization, and type-safe access to Pkl
//! configuration files. It requires the `pkl` CLI binary to be installed.
//!
//! This is a Rust port of Apple's [pkl-go](https://github.com/apple/pkl-go) library,
//! following the same MessagePack-based server protocol.
//!
//! # Quick start
//!
//! ```rust,no_run
//! use pkl_core::{PklDecode, PklEvaluator, CliEvaluator, ModuleSource};
//!
//! #[derive(PklDecode)]
//! struct Config {
//!     host: String,
//!     port: u16,
//! }
//!
//! # async fn example() -> Result<(), pkl_core::PklError> {
//! let cfg: Config = CliEvaluator::new()
//!     .evaluate(&ModuleSource::from_file("config.pkl")).await?;
//! println!("{}:{}", cfg.host, cfg.port);
//! # Ok(())
//! # }
//! ```
//!
//! # Evaluators
//!
//! Two backends are available:
//!
//! - [`CliEvaluator`] — spawns `pkl eval` for each call. Simple, no setup.
//! - [`ServerEvaluator`] — persistent `pkl server` connection. Faster for repeated use.
//!
//! Use [`EvaluatorManager`] to share one server process across multiple evaluators:
//!
//! ```rust,no_run
//! # use pkl_core::*;
//! # async fn example() -> Result<(), pkl_core::PklError> {
//! let mgr = EvaluatorManager::new().await?;
//! let ev1 = mgr.new_evaluator().await?;
//! let ev2 = mgr.new_evaluator().await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Derive macro
//!
//! The [`PklDecode`](derive.PklDecode.html) derive macro supports:
//!
//! - `#[pkl(rename = "...")]` — override the Pkl property name
//! - `#[pkl(default)]` — use `Default::default()` when property is missing
//! - `#[pkl(skip)]` — skip this field entirely
//! - `#[pkl(flatten)]` — capture remaining properties into `BTreeMap<String, Value>`
//! - `#[pkl(tag = "field")]` — tagged union enums with discriminator dispatch
//! - Enum support: string literal unions, try-each-variant untagged unions
//!
//! # External readers
//!
//! Register custom URI handlers via [`EvaluatorOptions`]:
//!
//! ```rust,no_run
//! # use pkl_core::*;
//! use pkl_core::reader::ResourceReader;
//!
//! struct MyReader;
//! impl ResourceReader for MyReader {
//!     fn scheme(&self) -> &str { "secret" }
//!     fn read(&self, _uri: &str) -> Result<Vec<u8>, String> { Ok(b"data".to_vec()) }
//!     fn is_globbable(&self) -> bool { false }
//!     fn has_hierarchical_uris(&self) -> bool { false }
//!     fn list_elements(&self, _uri: &str) -> Result<Vec<pkl_core::reader::PathElement>, String> {
//!         Err("n/a".into())
//!     }
//! }
//!
//! # async fn example() -> Result<(), pkl_core::PklError> {
//! let e = ServerEvaluator::with_options(EvaluatorOptions {
//!     resource_readers: vec![Box::new(MyReader)],
//!     ..Default::default()
//! }).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Project support
//!
//! Use `EvaluatorManager::new_project_evaluator` to create an evaluator
//! that resolves `@dependency/...` imports from a `PklProject` file.
//!
//! ```rust,no_run
//! # use pkl_core::*;
//! # async fn example() -> Result<(), pkl_core::PklError> {
//! let mgr = EvaluatorManager::new().await?;
//! let ev = mgr.new_project_evaluator("/path/to/project").await?;
//! # Ok(())
//! # }
//! ```

pub mod cli_evaluator;
pub mod decode;
pub mod error;
pub mod evaluator;
pub mod module_source;
pub mod msgapi;
pub mod project;
pub mod reader;
pub mod serdes;
pub mod server_evaluator;
pub mod types;
pub mod util;
pub mod value;

pub use cli_evaluator::CliEvaluator;
pub use error::{PklError, PklResult};
pub use evaluator::PklEvaluator;
pub use module_source::ModuleSource;
pub use server_evaluator::{EvaluatorManager, EvaluatorOptions, ServerEvaluator};
pub use types::{DataSize, Duration};
pub use value::Value;
