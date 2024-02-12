use logos::Lexer;

use pkl_fast::lexer::PklToken;

use super::{utils::jump_spaces_and_then, ParsingResult, Statement};

pub fn parse_amends<'source>(
    lexer: &mut Lexer<'source, PklToken>,
) -> ParsingResult<Statement<'source>> {
    let predicate = |lexer: &mut Lexer<'source, PklToken>| -> ParsingResult<Statement<'source>> {
        let raw_value = lexer.slice(); // retrieve value with quotes: "value"

        let value = &raw_value[1..raw_value.len() - 1];
        Ok(Statement::Amends(value))
    };

    let value = jump_spaces_and_then(lexer, &predicate);
    value
}
