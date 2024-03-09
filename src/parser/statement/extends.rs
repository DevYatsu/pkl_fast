use crate::{
    parser::{utils::parse_string_literal, PklParser},
    prelude::ParsingResult,
};

use super::Statement;
pub fn parse_extends<'source>(
    parser: &mut PklParser<'source>,
) -> ParsingResult<Statement<'source>> {
    let value = parse_string_literal(parser)?;

    Ok(Statement::Extends(value))
}
