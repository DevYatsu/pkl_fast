use crate::{
    parser::{expression::parse_expr, utils::list_while_not_token2, PklParser},
    prelude::{ParsingResult, PklToken},
};

use super::Expression;

pub fn parse_fn_call_arguments<'source>(
    parser: &mut PklParser<'source>,
) -> ParsingResult<Vec<Expression<'source>>> {
    list_while_not_token2(
        parser,
        PklToken::Comma,
        PklToken::CloseParenthesis,
        &parse_expr,
    )
}
