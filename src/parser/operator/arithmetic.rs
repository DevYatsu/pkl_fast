#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
/// An enum representing arithmetic operators used in mathematical expressions.
pub enum ArithmeticOperator {
    /// The addition operator `+`.
    Addition,
    /// The subtraction operator `-`.
    Subtraction,
    /// The multiplication operator `*`.
    Multiplication,
    /// The exponentiation operator `**`.
    Exponentiation,
    /// The division operator `/`.
    Division,
    /// The modulo operator `%`.
    Modulo,
    /// The bitwise OR operator `|`.
    BitwiseOr,
    /// The bitwise NOT operator `~|`.
    BitwiseNot,
}

impl ArithmeticOperator {
    /// Returns the precedence of the arithmetic operator.
    pub fn get_precedence(&self) -> u8 {
        match self {
            ArithmeticOperator::Addition | ArithmeticOperator::Subtraction => 1,
            ArithmeticOperator::Multiplication
            | ArithmeticOperator::Division
            | ArithmeticOperator::Modulo => 2,
            ArithmeticOperator::Exponentiation => 3,
            ArithmeticOperator::BitwiseOr => 4,
            ArithmeticOperator::BitwiseNot => 5,
        }
    }
}

impl From<&str> for ArithmeticOperator {
    fn from(value: &str) -> Self {
        match value {
            "+" => ArithmeticOperator::Addition,
            "-" => ArithmeticOperator::Subtraction,
            "*" => ArithmeticOperator::Multiplication,
            "**" => ArithmeticOperator::Exponentiation,
            "/" => ArithmeticOperator::Division,
            "%" => ArithmeticOperator::Modulo,
            "|" => ArithmeticOperator::BitwiseOr,
            "~/" => ArithmeticOperator::BitwiseNot,
            _ => unreachable!("Should not be reached! (in ArithmeticOperator struct)"),
        }
    }
}

use std::fmt;

impl fmt::Display for ArithmeticOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            ArithmeticOperator::Addition => "+",
            ArithmeticOperator::Subtraction => "-",
            ArithmeticOperator::Multiplication => "*",
            ArithmeticOperator::Exponentiation => "**",
            ArithmeticOperator::Division => "/",
            ArithmeticOperator::Modulo => "%",
            ArithmeticOperator::BitwiseOr => "|",
            ArithmeticOperator::BitwiseNot => "~/",
        };
        write!(f, "{}", symbol)
    }
}
