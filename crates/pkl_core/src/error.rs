use thiserror::Error;

/// Top-level error type for all Pkl operations.
#[derive(Debug, Error)]
pub enum PklError {
    /// Pkl CLI exited with a non-zero status or sent an error response.
    #[error("Pkl evaluation error: {0}")]
    EvalError(String),

    /// Failed to decode the binary response from Pkl.
    #[error("Decode error: {0}")]
    DecodeError(String),

    /// I/O error talking to the Pkl subprocess.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// The Pkl CLI binary was not found or could not be executed.
    #[error("Pkl CLI error: {0}")]
    CliError(String),

    /// Type mismatch during decoding.
    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch {
        expected: &'static str,
        actual: String,
    },

    /// A required property was missing.
    #[error("Missing property `{0}`")]
    MissingProperty(String),

    /// Unknown Pkl object code encountered.
    #[error("Unknown Pkl object code: 0x{0:02x}")]
    UnknownObjectCode(u8),

    /// Custom error context.
    #[error("{0}")]
    Custom(String),
}

/// Convenience alias for Results using PklError.
pub type PklResult<T> = Result<T, PklError>;
