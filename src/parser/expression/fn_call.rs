use crate::{
    parser::{expression::parse_expr, utils::list_while_not_token0},
    prelude::{ParsingResult, PklLexer, PklToken},
};

use super::Expression;

pub fn parse_fn_call_arguments<'source>(
    lexer: &mut PklLexer<'source>,
) -> ParsingResult<Vec<Expression<'source>>> {
    list_while_not_token0(
        lexer,
        PklToken::Comma,
        PklToken::CloseParenthesis,
        &parse_expr,
    )
}
