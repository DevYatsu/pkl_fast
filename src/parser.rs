mod constant;
pub mod import;

#[derive(Debug)]
pub enum Statement<'a> {
    Import(&'a str),
    GlobbedImport(&'a str),
}

pub type ParsingResult<T> = std::result::Result<T, ParsingError>;

#[derive(Debug)]
pub enum ParsingError {
    InvalidSyntax(String),
    UnexpectedToken(String),
    ExpectedSpace,
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::InvalidSyntax(message) => {
                write!(f, "Invalid syntax: {}", message)
            }
            ParsingError::UnexpectedToken(token) => {
                write!(f, "Unexpected token: {}", token)
            }
            ParsingError::ExpectedSpace => {
                write!(f, "Expected a whitespace")
            }
        }
    }
}
