use crate::prelude::{ParsingError, ParsingResult, PklLexer, PklToken};

use super::Statement;
pub fn parse_module<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<Statement<'source>> {
    let token = lexer.next();

    if let Some(Ok(PklToken::Identifier)) = token {
        let value = lexer.slice();
        Ok(Statement::Module(value))
    } else {
        if token.is_some() {
            Err(ParsingError::invalid_id(lexer))
        } else {
            Err(ParsingError::eof(lexer))
        }
    }
}
