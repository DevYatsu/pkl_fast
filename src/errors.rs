use logos::Span;

/// Represents a parsing error in the PKL format.
///
/// A `PklError` is a tuple consisting of:
///
/// * `String` - A message describing the error.
/// * `Span` - The span in the source where the error occurred.
/// * `Option<String>` - The name of the file in which the error occurs.
pub enum PklError {
    WithContext(String, Span, Option<String>),
    WithoutContext(String, Option<String>),
}

impl PklError {
    pub fn new(msg: String, span: Span) -> Self {
        Self::WithContext(msg, span, None)
    }
    pub fn with_file_name(mut self, name: String) -> Self {
        match &mut self {
            PklError::WithContext(_, _, n) => *n = Some(name),
            PklError::WithoutContext(_, n) => *n = Some(name),
        };
        self
    }

    pub fn msg(&self) -> &str {
        match self {
            PklError::WithContext(m, _, _) => m,
            PklError::WithoutContext(m, _) => m,
        }
    }
    pub fn file_name(&self) -> &Option<String> {
        match self {
            PklError::WithContext(_, _, n) => n,
            PklError::WithoutContext(_, n) => n,
        }
    }
    pub fn span(&self) -> Option<Span> {
        match self {
            PklError::WithContext(_, span, _) => Some(span.to_owned()),
            PklError::WithoutContext(_, _) => None,
        }
    }
}

/// A result type for PKL parsing operations.
///
/// The `PklResult` type is a specialized `Result` type used throughout the PKL parsing code.
/// It represents either a successful result (`T`) or a `PklError`.
pub type PklResult<T> = std::result::Result<T, PklError>;

impl From<(String, Span)> for PklError {
    fn from(value: (String, Span)) -> Self {
        Self::WithContext(value.0, value.1, None)
    }
}
impl From<(String, Span, String)> for PklError {
    fn from(value: (String, Span, String)) -> Self {
        Self::WithContext(value.0, value.1, Some(value.2))
    }
}
