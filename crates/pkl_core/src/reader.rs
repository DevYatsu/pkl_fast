//! External reader traits for custom resource and module loading.
//!
//! These traits allow users to register custom URI scheme handlers with the
//! `ServerEvaluator`. When Pkl code calls `read("myscheme:...")` or
//! `import "myscheme:..."`, the registered reader handles it.
//!
//! # Example
//!
//! ```rust,no_run
//! use pkl_core::reader::{ResourceReader, PathElement};
//! use std::collections::HashMap;
//!
//! struct SecretsReader {
//!     secrets: HashMap<String, Vec<u8>>,
//! }
//!
//! impl ResourceReader for SecretsReader {
//!     fn scheme(&self) -> &str { "secrets" }
//!     fn read(&self, uri: &str) -> Result<Vec<u8>, String> {
//!         let key = uri.strip_prefix("secrets:").unwrap_or(uri);
//!         self.secrets.get(key).cloned().ok_or_else(|| "not found".to_string())
//!     }
//!     fn is_globbable(&self) -> bool { false }
//!     fn has_hierarchical_uris(&self) -> bool { false }
//!     fn list_elements(&self, _uri: &str) -> Result<Vec<PathElement>, String> {
//!         Err("not supported".to_string())
//!     }
//! }
//! ```

use std::fmt;

/// A single path element returned by `list_elements`.
#[derive(Debug, Clone)]
pub struct PathElement {
    name: String,
    is_directory: bool,
}

impl PathElement {
    pub fn new(name: impl Into<String>, is_directory: bool) -> Self {
        Self { name: name.into(), is_directory }
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn is_directory(&self) -> bool { self.is_directory }
}

/// Error returned by `ResourceReader::read` to indicate the resource was not found.
#[derive(Debug, Clone)]
pub struct ResourceNotFound(pub String);

impl fmt::Display for ResourceNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "resource not found: {}", self.0)
    }
}

/// A custom resource reader for Pkl.
///
/// Handles `read("scheme:...")` and `read?("scheme:...")` calls in Pkl code.
pub trait ResourceReader: Send + Sync {
    /// The URI scheme this reader handles (e.g. "secrets", "db").
    fn scheme(&self) -> &str;

    /// Read the byte contents of the resource at the given URI.
    fn read(&self, uri: &str) -> Result<Vec<u8>, String>;

    /// Whether this reader supports globbing (`read*("scheme:pattern")`).
    fn is_globbable(&self) -> bool { false }

    /// Whether URIs for this scheme use hierarchical paths (/).
    fn has_hierarchical_uris(&self) -> bool { false }

    /// List elements at a path (only called if is_globbable or hierarchical).
    fn list_elements(&self, _uri: &str) -> Result<Vec<PathElement>, String> {
        Err("list_elements not supported".to_string())
    }
}

/// A custom module reader for Pkl.
///
/// Handles `import "scheme:..."` and `import("scheme:...")` calls in Pkl code.
pub trait ModuleReader: Send + Sync {
    /// The URI scheme this reader handles.
    fn scheme(&self) -> &str;

    /// Read the string contents of the module at the given URI.
    fn read(&self, uri: &str) -> Result<String, String>;

    /// Whether this reader is considered local (enables `...` triple-dot imports).
    fn is_local(&self) -> bool { false }

    /// Whether this reader supports globbing (`import* "scheme:pattern"`).
    fn is_globbable(&self) -> bool { false }

    /// Whether URIs for this scheme use hierarchical paths.
    fn has_hierarchical_uris(&self) -> bool { false }

    /// List elements at a path.
    fn list_elements(&self, _uri: &str) -> Result<Vec<PathElement>, String> {
        Err("list_elements not supported".to_string())
    }
}
