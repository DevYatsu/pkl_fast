use super::Expression;
use crate::{
    parser::{operator::parse_operation, utils::retrieve_opt_next_token, value::parse_object},
    prelude::PklToken,
};

use super::super::{errors::ParsingError, ParsingResult, PklLexer};

pub fn parse_complex_expr<'source>(
    lexer: &mut PklLexer<'source>,
    expr: Expression<'source>,
    opt_token: Option<PklToken<'source>>,
) -> ParsingResult<(Expression<'source>, Option<PklToken<'source>>)> {
    let token = if opt_token.is_some() {
        opt_token
    } else {
        retrieve_opt_next_token(lexer)?
    };

    match token {
        Some(PklToken::Operator(op)) | Some(PklToken::RightAngleBracket(op)) => {
            parse_operation(lexer, expr, op)
        }
        Some(PklToken::Is) => parse_operation(lexer, expr, "is"),
        Some(PklToken::As) => parse_operation(lexer, expr, "as"),
        Some(PklToken::OpenBracket) => {
            if let Expression::Parenthesised(inner_expr) = expr {
                if let Expression::Identifier(name) = *inner_expr {
                    let (value, next) = parse_object(lexer, Some(name))?;
                    return Ok((Expression::Value(value), next));
                }
            }

            Err(ParsingError::unexpected(lexer, "line ending".to_owned()))
        }
        _ => Ok((expr, token)),
    }
}
