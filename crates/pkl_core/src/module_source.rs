/// Describes a Pkl module to be evaluated.
///
/// Modules can be loaded from a file path, a URI, or provided as inline text.
#[derive(Debug, Clone)]
pub struct ModuleSource {
    /// The module URI (e.g. "file:///path/to/config.pkl", "https://...", "modulepath:...").
    pub uri: String,
    /// Optional inline Pkl source text. If `None`, the evaluator resolves the URI itself.
    pub contents: Option<String>,
}

impl ModuleSource {
    /// Create a source from a file path. The path is converted to a `file://` URI.
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Self {
        let path = path.as_ref();
        let uri = format!("file://{}", path.display());
        Self {
            uri,
            contents: None,
        }
    }

    /// Create a source from a module URI (the evaluator will fetch it).
    pub fn from_uri(uri: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            contents: None,
        }
    }

    /// Create a source from inline Pkl source text.
    ///
    /// For CliEvaluator: the text is piped via stdin.
    ///
    /// For ServerEvaluator: uses a `repl:` URI. The module text is sent
    /// as part of the Evaluate request. If the server requests the module
    /// content via ReadModuleRequest, a registered `repl:` client module
    /// reader handles it.
    pub fn from_text(text: impl Into<String>) -> Self {
        let id = uuid::Uuid::new_v4();
        Self {
            uri: format!("repl:{}", id),
            contents: Some(text.into()),
        }
    }
}
