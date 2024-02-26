use crate::prelude::{ParsingError, ParsingResult, PklLexer, PklToken};

use super::Statement;

pub fn parse_var_declaration<'source>(
    lexer: &mut PklLexer<'source>,
    identifier_value: &'source str,
) -> ParsingResult<Statement<'source>> {
    todo!()
}
