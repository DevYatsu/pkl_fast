use logos::Lexer;

use pkl_fast::lexer::PklToken;

use super::{ParsingResult, Statement};

pub fn parse_import<'source>(
    lexer: &mut Lexer<'source, PklToken>,
) -> ParsingResult<Statement<'source>> {
    let value = parse_import_value(lexer)?;
    Ok(Statement::Import(value))
}

pub fn parse_globbed_import<'source>(
    lexer: &mut Lexer<'source, PklToken>,
) -> ParsingResult<Statement<'source>> {
    let value = parse_import_value(lexer)?;
    Ok(Statement::GlobbedImport(value))
}

fn parse_import_value<'source>(
    lexer: &mut Lexer<'source, PklToken>,
) -> ParsingResult<&'source str> {
    lexer.next();
    // skip whitespace
    // after an import there is necessarily a whitespace otherwise the lexer would not have created an Import Token but an Indentifier Token

    lexer.next(); // go to next token which is the value
    let raw_value = lexer.slice(); // retrieve value with quotes: "value"
    let value = &raw_value[1..raw_value.len() - 1];

    Ok(value)
}
