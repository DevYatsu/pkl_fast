use super::Expression;
use crate::parser::PklParser;

use super::super::ParsingResult;

pub fn parse_basic_expr<'source>(
    parser: &mut PklParser<'source>,
) -> ParsingResult<Expression<'source>> {
    todo!()

    // let token = if opt_token.is_some() {
    //     opt_token.unwrap()
    // } else {
    //     retrieve_next_token(parser)?
    // };

    // match token {
    //     PklToken::LogicalNotOperator => {
    //         let (expr, next) = parse_basic_expr(parser, None)?;
    //         Ok((Expression::LogicalNot(expr.into()), next))
    //     }
    //     PklToken::OpenParenthesis => {
    //         let (expr, next_token) = parse_expr(parser, None)?;

    //         match next_token {
    //             Some(PklToken::CloseParenthesis) => (),
    //             _ => return Err(ParsingError::unexpected(parser, "')'".to_owned())),
    //         };

    //         let expr = Expression::Parenthesised(expr.into());
    //         Ok(parse_opt_member_expr(parser, expr)?)
    //     }
    //     PklToken::Identifier(ident) => {
    //         let expr = Expression::Identifier(ident);

    //         Ok(parse_opt_member_expr(parser, expr)?)
    //     }
    //     PklToken::ListIndexing(indexed) => {
    //         let (expr, next_token) = parse_opt_newlines(parser, &parse_expr)?;
    //         expect_token_with_opt_newlines(parser, next_token, PklToken::CloseBrace)?;
    //         let expr = Expression::ListIndexing {
    //             indexed,
    //             indexer: expr.into(),
    //         };

    //         Ok(parse_opt_member_expr(parser, expr)?)
    //     }
    //     PklToken::NonNullIdentifier(name) => {
    //         let expr = Expression::NonNull(Expression::Identifier(name).into());

    //         Ok(parse_opt_member_expr(parser, expr)?)
    //     }
    //     PklToken::FunctionCall(func_name) => {
    //         let args = parse_fn_call_arguments(parser)?;

    //         let expr = match func_name {
    //             "List" => Expression::Value(PklValue::List(args)),
    //             "Map" => Expression::Value(PklValue::Map(args)),
    //             "Set" => Expression::Value(PklValue::Set(args)),
    //             _ => Expression::FunctionCall { func_name, args },
    //         };

    //         Ok(parse_opt_member_expr(parser, expr)?)
    //     }
    //     PklToken::If => {
    //         expect_token(parser, PklToken::OpenParenthesis)?;

    //         let (condition, next) = parse_expr(parser, None)?;
    //         assert_token_eq(parser, next, PklToken::CloseParenthesis)?;

    //         let (condition_true, next_token) = parse_opt_newlines(parser, &parse_expr)?;

    //         expect_token_with_opt_newlines(parser, next_token, PklToken::Else)?;

    //         let (_else, opt_token) = parse_opt_newlines(parser, &parse_expr)?;

    //         Ok((
    //             Expression::If {
    //                 condition: Box::new(condition),
    //                 condition_true: Box::new(condition_true),
    //                 _else: Box::new(_else),
    //             },
    //             opt_token,
    //         ))
    //     }
    //     PklToken::Let => {
    //         expect_token_with_opt_newlines(parser, None, PklToken::OpenParenthesis)?;
    //         let name = parse_identifier(parser)?;

    //         let opt_type = match retrieve_next_token(parser)? {
    //             PklToken::EqualSign => None,
    //             PklToken::Colon => {
    //                 let (t, next) = parse_type(parser, None)?;
    //                 assert_token_eq(parser, next, PklToken::EqualSign)?;
    //                 Some(t.into())
    //             }
    //             _ => return Err(ParsingError::unexpected(parser, "= or :".to_owned())),
    //         };

    //         let (value, next) = parse_expr(parser, None)?;

    //         expect_token_with_opt_newlines(parser, next, PklToken::CloseParenthesis)?;

    //         let (expr, next_token) = parse_opt_newlines(parser, &parse_expr)?;

    //         Ok((
    //             Expression::Let {
    //                 name,
    //                 opt_type,
    //                 value: value.into(),
    //                 expr: expr.into(),
    //             },
    //             next_token,
    //         ))
    //     }
    //     current_token => {
    //         let expr = Expression::Value(parse_value(parser, current_token)?);
    //         Ok(parse_opt_member_expr(parser, expr)?)
    //     }
    // }
}

fn parse_opt_member_expr<'source>(
    parser: &mut PklParser<'source>,
    expr: Expression<'source>,
) -> ParsingResult<Expression<'source>> {
    todo!()
    // let next_token = retrieve_opt_next_token(parser)?;

    // match next_token {
    //     Some(PklToken::Dot) => {
    //         // function call or identifier
    //         let (second_expr, next) = parse_basic_expr(parser, None)?;

    //         return Ok((
    //             Expression::MemberExpression {
    //                 object: expr.into(),
    //                 property: second_expr.into(),
    //             },
    //             next,
    //         ));
    //     }
    //     _ => (),
    // };

    // Ok((expr, next_token))
}
