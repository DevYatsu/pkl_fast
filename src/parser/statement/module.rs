use crate::{
    parser::utils::parse_identifier,
    prelude::{ParsingResult, PklLexer},
};

use super::Statement;
pub fn parse_module<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<Statement<'source>> {
    let value = parse_identifier(lexer)?;

    Ok(Statement::Module(value))
}
