use super::{
    utils::{jump_spaces_and_then, parse_object_name},
    ParsingResult, PklLexer, Statement,
};

pub fn parse_module<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<Statement<'source>> {
    jump_spaces_and_then(lexer, &|_token, lexer| {
        // todo!("modify parse object name fn to suppose that token is coming in");
        let value: &str = parse_object_name(lexer)?;
        Ok(Statement::Module(value))
    })
}
