use crate::{
    parser::utils::parse_string_literal,
    prelude::{ParsingResult, PklLexer},
};

use super::Statement;

pub fn parse_amends<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<Statement<'source>> {
    let value = parse_string_literal(lexer)?;

    Ok(Statement::Amends(value))
}
