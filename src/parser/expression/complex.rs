use super::Expression;
use crate::parser::PklParser;

use super::super::ParsingResult;

pub fn parse_complex_expr<'source>(
    parser: &mut PklParser<'source>,
    expr: Expression<'source>,
) -> ParsingResult<Expression<'source>> {
    todo!()
    // let token = if opt_token.is_some() {
    //     opt_token
    // } else {
    //     retrieve_opt_next_token(parser)?
    // };

    // match token {
    //     Some(PklToken::Operator(op)) | Some(PklToken::RightAngleBracket(op)) => {
    //         parse_operation(parser, expr, op)
    //     }
    //     Some(PklToken::Is) => parse_operation(parser, expr, "is"),
    //     Some(PklToken::As) => parse_operation(parser, expr, "as"),
    //     Some(PklToken::OpenBracket) => {
    //         if let Expression::Parenthesised(inner_expr) = expr {
    //             if let Expression::Identifier(name) = *inner_expr {
    //                 let (value, next) = parse_object(parser, Some(name))?;
    //                 return Ok((Expression::Value(value), next));
    //             }
    //         }

    //         Err(ParsingError::unexpected(parser, "line ending".to_owned()))
    //     }
    //     _ => Ok((expr, token)),
    // }
}
