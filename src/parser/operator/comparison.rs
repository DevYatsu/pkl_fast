#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
/// An enum representing comparison operators used in logical and conditional expressions.
pub enum ComparisonOperator {
    /// The equality operator `==`.
    Equal,
    /// The less than or equal to operator `<=`.
    LessThanOrEqual,
    /// The less than operator `<`.
    LessThan,
    /// The greater than or equal to operator `>=`.
    GreaterThanOrEqual,
    /// The greater than operator `>`.
    GreaterThan,
    /// The not equal to operator `!=`.
    NotEqual,
    /// The logical NOT NOT operator `!!`.
    NotNot,
    /// The logical NOT operator `!`.
    Not,
    /// The null-coalescing operator `??`.
    DoubleQuestion,
    /// The conditional operator `?`.
    Question,
    /// The logical AND operator `&&`.
    LogicalAnd,
    /// The bitwise AND operator `&`.
    BitwiseAnd,
    /// The logical OR operator `||`.
    LogicalOr,
    /// The bitwise OR operator `|`.
    BitwiseOr,
}

impl ComparisonOperator {
    /// Returns the precedence of the comparison operator.
    pub fn get_precedence(&self) -> u8 {
        match self {
            ComparisonOperator::Equal | ComparisonOperator::NotEqual => 3,
            ComparisonOperator::LessThanOrEqual
            | ComparisonOperator::LessThan
            | ComparisonOperator::GreaterThanOrEqual
            | ComparisonOperator::GreaterThan => 4,
            ComparisonOperator::NotNot | ComparisonOperator::Not => 8,
            ComparisonOperator::DoubleQuestion | ComparisonOperator::Question => 1,
            ComparisonOperator::LogicalAnd | ComparisonOperator::BitwiseAnd => 2,
            ComparisonOperator::LogicalOr | ComparisonOperator::BitwiseOr => 2,
        }
    }
}

impl From<&str> for ComparisonOperator {
    fn from(value: &str) -> Self {
        match value {
            "==" => ComparisonOperator::Equal,
            "<=" => ComparisonOperator::LessThanOrEqual,
            "<" => ComparisonOperator::LessThan,
            ">=" => ComparisonOperator::GreaterThanOrEqual,
            ">" => ComparisonOperator::GreaterThan,
            "!=" => ComparisonOperator::NotEqual,
            "!!" => ComparisonOperator::NotNot,

            // should never happen as we parse it another way in the expr parser
            "!" => ComparisonOperator::Not,

            "??" => ComparisonOperator::DoubleQuestion,
            "?" => ComparisonOperator::Question,
            "&&" => ComparisonOperator::LogicalAnd,
            "&" => ComparisonOperator::BitwiseAnd,
            "||" => ComparisonOperator::LogicalOr,
            "|" => ComparisonOperator::BitwiseOr,
            _ => unreachable!("Should not be reached! (in ComparisonOperator struct)"),
        }
    }
}

use std::fmt;

impl fmt::Display for ComparisonOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            ComparisonOperator::Equal => "==",
            ComparisonOperator::LessThanOrEqual => "<=",
            ComparisonOperator::LessThan => "<",
            ComparisonOperator::GreaterThanOrEqual => ">=",
            ComparisonOperator::GreaterThan => ">",
            ComparisonOperator::NotEqual => "!=",
            ComparisonOperator::NotNot => "!!",
            ComparisonOperator::Not => "!",
            ComparisonOperator::DoubleQuestion => "??",
            ComparisonOperator::Question => "?",
            ComparisonOperator::LogicalAnd => "&&",
            ComparisonOperator::BitwiseAnd => "&",
            ComparisonOperator::LogicalOr => "||",
            ComparisonOperator::BitwiseOr => "|",
        };
        write!(f, "{}", symbol)
    }
}
