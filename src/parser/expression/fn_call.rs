use crate::{parser::PklParser, prelude::ParsingResult};

use super::Expression;

pub fn parse_fn_call_arguments<'source>(
    parser: &mut PklParser<'source>,
) -> ParsingResult<Vec<Expression<'source>>> {
    todo!()
    // list_while_not_token2(
    //     parser,
    //     PklToken::Comma,
    //     PklToken::CloseParenthesis,
    //     &parse_expr,
    // )
}
