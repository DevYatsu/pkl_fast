use super::super::utils::expect_token;
use crate::prelude::{ParsingResult, PklLexer, PklToken};

pub fn parse_equal<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<()> {
    expect_token(lexer, PklToken::EqualSign)
}
