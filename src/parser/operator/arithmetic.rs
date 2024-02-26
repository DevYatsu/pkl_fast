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
