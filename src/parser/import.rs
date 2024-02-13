use super::{utils::jump_spaces_and_then, ParsingError, ParsingResult, PklLexer, Statement};
use pkl_fast::lexer::PklToken;

pub fn parse_import<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<Statement<'source>> {
    let value = parse_import_value(lexer)?;
    Ok(Statement::Import(value))
}

pub fn parse_globbed_import<'source>(
    lexer: &mut PklLexer<'source>,
) -> ParsingResult<Statement<'source>> {
    let value = parse_import_value(lexer)?;
    Ok(Statement::GlobbedImport(value))
}

fn parse_import_value<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<&'source str> {
    jump_spaces_and_then(lexer, &|token, lexer| {
        if let Ok(PklToken::StringLiteral) = token {
            let raw_value = lexer.slice(); // retrieve value with quotes: "value"
            println!("raw : {:?}", raw_value);

            let value = &raw_value[1..raw_value.len() - 1];
            Ok(value)
        } else {
            Err(ParsingError::Expected(format!(
                "Expected a valid `import` value at: {}",
                &(lexer.source()[lexer.span().start..lexer.span().start + 15])
            )))
        }
    })
}
