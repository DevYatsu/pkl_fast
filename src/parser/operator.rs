use super::{
    expression::{basic::parse_basic_expr, Expression},
    types::parse_type,
    utils::parse_opt_newlines,
    ParsingResult, PklLexer,
};
use crate::prelude::PklToken;
use std::fmt;

mod arithmetic;
mod comparison;

pub use arithmetic::ArithmeticOperator;
pub use comparison::ComparisonOperator;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Arithmetic(ArithmeticOperator),
    Comparison(ComparisonOperator),
    TypeTest,
    TypeCast,
}

/// Parses the next token to determine if it's an operator. If an operator is found,
/// it constructs a `Expression::BinaryOperation` with the given expression as the left-hand side,
/// and the expression obtained from parsing the next token as the right-hand side.
/// If no operator is found, it returns the given expression and the next token.
/// If there are other operators following with higher precedence, it recursively generates
/// `Expression::BinaryOperation` nodes to ensure correct precedence order.
/// Returns a tuple containing the resulting expression and the next token encountered (or None if there's none).
///
/// **Simply put, this fn parses a mathematical expression and returns the next token.**
pub fn parse_operation<'source>(
    lexer: &mut PklLexer<'source>,
    expr: Expression<'source>,
    operator: &'source str,
) -> ParsingResult<(Expression<'source>, Option<PklToken<'source>>)> {
    let mut output_queue = Vec::new();
    let mut operator_stack: Vec<Operator> = Vec::new();
    let mut return_token = None;
    output_queue.push(expr);

    // we take care of the initial expr and operator given as parameter
    let first_operator = Operator::from(operator);
    let (first_expr, next) = parse_expr_following_op(lexer, &first_operator)?;
    output_queue.push(first_expr);
    operator_stack.push(first_operator);

    let mut next_token = next;

    loop {
        match next_token {
            Some(PklToken::Operator(op)) | Some(PklToken::RightAngleBracket(op)) => {
                let new_operator = Operator::from(op);

                let (new_expr, next) = parse_expr_following_op(lexer, &new_operator)?;

                next_token = next;

                output_queue.push(new_expr);
                // If the operator stack is not empty and the precedence of the new operator
                // is less than or equal to the precedence of the operator on top of the stack,
                // pop operators from the stack and add them to the output queue.
                while let Some(top_operator) = operator_stack.last() {
                    if new_operator.get_precedence() <= top_operator.get_precedence() {
                        if let Some(top_expr) = output_queue.pop() {
                            let new_expr = Expression::Operation {
                                operator: operator_stack.pop().unwrap(),
                                lhs: Box::new(output_queue.pop().unwrap()), // Safe unwrap because we're checking it inside the loop
                                rhs: Box::new(top_expr),
                            };
                            output_queue.push(new_expr);
                        }
                    } else {
                        break;
                    }
                }

                operator_stack.push(new_operator);
            }
            Some(_) => {
                return_token = next_token;
                break;
            }
            None => break,
        };
    }

    // Pop remaining operators from the stack and add them to the output queue
    while let Some(operator) = operator_stack.pop() {
        // we can unwrap safely as we are sure there is enough operands
        let rhs = output_queue.pop().unwrap();
        let lhs = output_queue.pop().unwrap();
        let new_expr = Expression::Operation {
            operator,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
        output_queue.push(new_expr);
    }

    Ok((output_queue.pop().unwrap(), return_token))
}

fn parse_expr_following_op<'source>(
    lexer: &mut PklLexer<'source>,
    op: &Operator,
) -> ParsingResult<(Expression<'source>, Option<PklToken<'source>>)> {
    match op {
        Operator::TypeCast | Operator::TypeTest => {
            let (t, next) = parse_opt_newlines(lexer, &parse_type)?;
            Ok((Expression::ExpressionType(Box::new(t)), next))
        }
        _ => parse_opt_newlines(lexer, &parse_basic_expr),
    }
}

impl Operator {
    pub fn get_precedence(&self) -> u8 {
        match self {
            Operator::Arithmetic(op) => op.get_precedence(),
            Operator::Comparison(op) => op.get_precedence(),
            Operator::TypeCast => 0,
            Operator::TypeTest => 0,
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Arithmetic(x) => write!(f, "{x}"),
            Operator::Comparison(x) => write!(f, "{x}"),
            Operator::TypeCast => write!(f, "as"),
            Operator::TypeTest => write!(f, "is"),
        }
    }
}

impl From<ArithmeticOperator> for Operator {
    fn from(value: ArithmeticOperator) -> Self {
        Operator::Arithmetic(value)
    }
}
impl From<ComparisonOperator> for Operator {
    fn from(value: ComparisonOperator) -> Self {
        Operator::Comparison(value)
    }
}

impl From<&str> for Operator {
    fn from(value: &str) -> Self {
        match value {
            "+" => ArithmeticOperator::Addition.into(),
            "-" => ArithmeticOperator::Subtraction.into(),
            "*" => ArithmeticOperator::Multiplication.into(),
            "**" => ArithmeticOperator::Exponentiation.into(),
            "/" => ArithmeticOperator::Division.into(),
            "%" => ArithmeticOperator::Modulo.into(),
            "~/" => ArithmeticOperator::IntegerDivision.into(),
            "==" => ComparisonOperator::Equal.into(),
            "<=" => ComparisonOperator::LessThanOrEqual.into(),
            "<" => ComparisonOperator::LessThan.into(),
            ">=" => ComparisonOperator::GreaterThanOrEqual.into(),
            ">" => ComparisonOperator::GreaterThan.into(),
            "!=" => ComparisonOperator::NotEqual.into(),
            "!!" => ComparisonOperator::NotNot.into(),
            "!" => ComparisonOperator::Not.into(),
            "?" => ComparisonOperator::Question.into(),
            "??" => ComparisonOperator::DoubleQuestion.into(),
            "&&" => ComparisonOperator::LogicalAnd.into(),
            "&" => ComparisonOperator::BitwiseAnd.into(),
            "|" => ComparisonOperator::BitwiseOr.into(),
            "||" => ComparisonOperator::LogicalOr.into(),
            "is" => Operator::TypeTest,
            "as" => Operator::TypeTest,
            _ => unreachable!("Should not be reached! (in Operator)"),
        }
    }
}
