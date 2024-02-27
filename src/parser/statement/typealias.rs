use crate::{
    parser::{operator::parse_equal, types::parse_type, utils::parse_identifier},
    prelude::{ParsingResult, PklLexer},
};

use super::Statement;
pub fn parse_typealias<'source>(
    lexer: &mut PklLexer<'source>,
) -> ParsingResult<Statement<'source>> {
    let alias = parse_identifier(lexer)?;
    parse_equal(lexer)?;

    let equivalent_type = parse_type(lexer)?;

    Ok(Statement::TypeAlias {
        alias,
        equivalent_type,
    })
}
