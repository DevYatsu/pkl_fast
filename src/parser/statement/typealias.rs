use crate::{
    parser::{
        types::parse_type,
        utils::{expect_token, parse_identifier},
    },
    prelude::{ParsingResult, PklLexer, PklToken},
};

use super::Statement;
pub fn parse_typealias<'source>(
    lexer: &mut PklLexer<'source>,
) -> ParsingResult<Statement<'source>> {
    let alias = parse_identifier(lexer)?;
    expect_token(lexer, PklToken::EqualSign)?;

    let equivalent_type = parse_type(lexer)?;

    Ok(Statement::TypeAlias {
        alias,
        equivalent_type,
    })
}
