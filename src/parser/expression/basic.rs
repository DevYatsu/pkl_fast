use super::{fn_call::parse_fn_call_arguments, parse_expr, Expression};
use crate::{
    parser::utils::{assert_token_eq, retrieve_next_token, retrieve_opt_next_token},
    prelude::PklToken,
};

use super::super::{
    errors::ParsingError,
    utils::{expect_token, expect_token_with_opt_newlines, parse_opt_newlines},
    value::{parse_value, PklValue},
    ParsingResult, PklLexer,
};

pub fn parse_basic_expr<'source>(
    lexer: &mut PklLexer<'source>,
    opt_token: Option<PklToken<'source>>,
) -> ParsingResult<(Expression<'source>, Option<PklToken<'source>>)> {
    let token = if opt_token.is_some() {
        opt_token.unwrap()
    } else {
        retrieve_next_token(lexer)?
    };

    let expr = match token {
        PklToken::LogicalNotOperator => {
            let (expr, next) = parse_basic_expr(lexer, None)?;
            return Ok((Expression::LogicalNot(expr.into()), next));
        }
        PklToken::OpenParenthesis => {
            let (expr, next_token) = parse_expr(lexer, None)?;

            match next_token {
                Some(PklToken::CloseParenthesis) => (),
                _ => return Err(ParsingError::unexpected(lexer, "'('".to_owned())),
            };

            Expression::Parenthesised(expr.into())
        }
        PklToken::Identifier(ident) => Expression::Identifier(ident),
        PklToken::ListIndexing(indexed) => {
            let (expr, next_token) = parse_opt_newlines(lexer, &parse_expr)?;
            expect_token_with_opt_newlines(lexer, next_token, PklToken::CloseBrace)?;
            Expression::ListIndexing {
                indexed,
                indexer: expr.into(),
            }
        }
        PklToken::NonNullIdentifier(name) => {
            Expression::NonNull(Expression::Identifier(name).into())
        }
        PklToken::If => {
            expect_token(lexer, PklToken::OpenParenthesis)?;

            let (condition, next) = parse_expr(lexer, None)?;
            assert_token_eq(lexer, next, PklToken::CloseParenthesis)?;

            let (condition_true, next_token) = parse_opt_newlines(lexer, &parse_expr)?;

            expect_token_with_opt_newlines(lexer, next_token, PklToken::Else)?;

            let (_else, opt_token) = parse_opt_newlines(lexer, &parse_expr)?;

            return Ok((
                Expression::If {
                    condition: Box::new(condition),
                    condition_true: Box::new(condition_true),
                    _else: Box::new(_else),
                },
                opt_token,
            ));
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

    Ok((expr, retrieve_opt_next_token(lexer)?))
}
