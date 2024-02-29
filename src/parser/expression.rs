use crate::{parser::utils::retrieve_next_token, prelude::PklToken};

use self::fn_call::parse_fn_call_arguments;

use super::{
    operator::Operator,
    value::{parse_value, PklValue},
    ParsingResult, PklLexer,
};

mod fn_call;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    Value(PklValue<'a>),
    Identifier(&'a str),
    FunctionCall {
        func_name: &'a str,
        args: Vec<Expression<'a>>,
    },

    Operation {
        operator: Operator,
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },
}

pub fn parse_expr<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<Expression<'source>> {
    let token = retrieve_next_token(lexer)?;

    match token {
        PklToken::Identifier(ident) => Ok(Expression::Identifier(ident)),
        PklToken::FunctionCall(func_name) => {
            let args = parse_fn_call_arguments(lexer)?;
            Ok(Expression::FunctionCall { func_name, args })
        }

        current_token => Ok(Expression::Value(parse_value(lexer, current_token)?)),
    }
}

use std::fmt;

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
        }
    }
}
