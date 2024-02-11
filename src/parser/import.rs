use logos::Lexer;

use crate::lexer::Token;

use super::{ParsingResult, Statement};

pub fn parse_import<'source>(
    lexer: &mut Lexer<'source, Token>,
) -> ParsingResult<Statement<'source>> {
    let value = parse_import_value(lexer)?;
    Ok(Statement::Import(value))
}

pub fn parse_globbed_import<'source>(
    lexer: &mut Lexer<'source, Token>,
) -> ParsingResult<Statement<'source>> {
    let value = parse_import_value(lexer)?;
    Ok(Statement::GlobbedImport(value))
}

fn parse_import_value<'source>(lexer: &mut Lexer<'source, Token>) -> ParsingResult<&'source str> {
    lexer.next(); // skip import
    lexer.next(); // skip whitespace
    let raw_value = lexer.slice(); // value with quotes: "value"
    let value = &raw_value[1..raw_value.len() - 1];

    Ok(value)
}
