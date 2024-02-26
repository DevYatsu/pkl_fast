use crate::prelude::{ParsingResult, PklLexer};

use super::Statement;
pub fn parse_module<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<Statement<'source>> {
    lexer.next();
    let value = lexer.slice();
    Ok(Statement::Module(value))
}
