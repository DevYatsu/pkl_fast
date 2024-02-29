mod arithmetic;
mod comparison;

pub use arithmetic::ArithmeticOperator;
pub use comparison::ComparisonOperator;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Arithmetic(ArithmeticOperator),
    Comparison(ComparisonOperator),
}

use std::fmt;

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Arithmetic(x) => write!(f, "{x}"),
            Operator::Comparison(x) => write!(f, "{x}"),
        }
    }
}
