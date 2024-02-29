mod arithmetic;
mod comparison;

pub use arithmetic::ArithmeticOperator;
pub use comparison::ComparisonOperator;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Arithmetic(ArithmeticOperator),
    Comparison(ComparisonOperator),
}
