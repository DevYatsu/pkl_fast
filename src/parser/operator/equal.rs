use super::super::utils::parse_token;
use crate::prelude::{ParsingResult, PklLexer, PklToken};

pub fn parse_equal<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<()> {
    parse_token(lexer, PklToken::EqualSign)
}
