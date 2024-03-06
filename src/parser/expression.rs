use self::{basic::parse_basic_expr, complex::parse_complex_expr};
use crate::prelude::{ParsingResult, PklToken};
use std::fmt;

use super::{operator::Operator, value::PklValue, PklLexer};

pub mod basic;
mod complex;

mod fn_call;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    Value(PklValue<'a>),
    Identifier(&'a str),
    FunctionCall {
        func_name: &'a str,
        args: Vec<Expression<'a>>,
    },

    ListIndexing {
        indexed: &'a str,
        indexer: Box<Expression<'a>>,
    },

    Operation {
        operator: Operator,
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },

    If {
        condition: Box<Expression<'a>>,
        condition_true: Box<Expression<'a>>,
        _else: Box<Expression<'a>>,
    },

    LogicalNot(Box<Expression<'a>>),
    NonNull(Box<Expression<'a>>),
    Parenthesised(Box<Expression<'a>>),
}

pub fn parse_expr<'source>(
    lexer: &mut PklLexer<'source>,
    opt_token: Option<PklToken<'source>>,
) -> ParsingResult<(Expression<'source>, Option<PklToken<'source>>)> {
    let (expr, opt_token) = parse_basic_expr(lexer, opt_token)?;

    parse_complex_expr(lexer, expr, opt_token)
}

impl<'a> fmt::Display for Expression<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Value(value) => write!(f, "{}", value),
            Expression::Identifier(identifier) => write!(f, "{}", identifier),
            Expression::FunctionCall { func_name, args } => {
                write!(f, "{}(", func_name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Expression::Operation { operator, lhs, rhs } => {
                write!(f, "{} {} {}", lhs, operator, rhs)
            }
            Expression::LogicalNot(x) => write!(f, "!{x}"),
            Expression::NonNull(x) => write!(f, "{x}!!"),
            Expression::Parenthesised(x) => write!(f, "({x})"),
            Expression::If {
                condition,
                condition_true,
                _else,
            } => write!(f, "if ({}) {} else {}", condition, condition_true, _else),
            Expression::ListIndexing { indexed, indexer } => write!(f, "{}[{}]", indexed, indexer),
        }
    }
}
