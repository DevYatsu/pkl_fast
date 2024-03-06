use self::fn_call::parse_fn_call_arguments;
use crate::{
    parser::utils::{assert_token_eq, retrieve_next_token},
    prelude::PklToken,
};
use std::fmt;

use super::{
    errors::ParsingError,
    operator::{parse_opt_operation, Operator},
    utils::{expect_token, expect_token_with_opt_newlines, parse_opt_newlines},
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
) -> ParsingResult<(Expression<'source>, Option<PklToken<'source>>)> {
    let expr = parse_basic_expr(lexer, None)?;

    parse_opt_operation(lexer, expr)
}

pub fn parse_basic_expr<'source>(
    lexer: &mut PklLexer<'source>,
    opt_token: Option<PklToken<'source>>,
) -> ParsingResult<Expression<'source>> {
    let token = if opt_token.is_some() {
        opt_token.unwrap()
    } else {
        retrieve_next_token(lexer)?
    };

    let expr = match token {
        PklToken::LogicalNotOperator => {
            Expression::LogicalNot(parse_basic_expr(lexer, None)?.into())
        }
        PklToken::OpenParenthesis => {
            let (expr, next_token) = parse_expr(lexer)?;

            match next_token {
                Some(PklToken::CloseParenthesis) => (),
                _ => return Err(ParsingError::unexpected(lexer, "'('".to_owned())),
            };

            Expression::Parenthesised(expr.into())
        }
        PklToken::Identifier(ident) => Expression::Identifier(ident),
        PklToken::NonNullIdentifier(name) => {
            Expression::NonNull(Expression::Identifier(name).into())
        }
        PklToken::If => {
            expect_token(lexer, PklToken::OpenParenthesis)?;

            let (condition, next) = parse_expr(lexer)?;
            assert_token_eq(lexer, next, PklToken::CloseParenthesis)?;

            let condition_true = parse_opt_newlines(lexer, &parse_basic_expr)?;
            expect_token_with_opt_newlines(lexer, PklToken::Else)?;

            let _else = parse_opt_newlines(lexer, &parse_basic_expr)?;

            Expression::If {
                condition: Box::new(condition),
                condition_true: Box::new(condition_true),
                _else: Box::new(_else),
            }
        }
        PklToken::FunctionCall(func_name) => {
            let args = parse_fn_call_arguments(lexer)?;

            match func_name {
                "List" => Expression::Value(PklValue::List(args)),
                "Map" => Expression::Value(PklValue::Map(args)),
                "Set" => Expression::Value(PklValue::Set(args)),
                _ => Expression::FunctionCall { func_name, args },
            }
        }
        current_token => Expression::Value(parse_value(lexer, current_token)?),
    };

    Ok(expr)
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
        }
    }
}
