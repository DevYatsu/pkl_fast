use super::{ParsingResult, PklLexer, Statement};

pub fn parse_module<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<Statement<'source>> {
    lexer.next();
    let value = lexer.slice();
    Ok(Statement::Module(value))
}
