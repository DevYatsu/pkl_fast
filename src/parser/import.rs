use std::path::Path;

use crate::parser::errors::locating::generate_source;

use super::{
    errors::{locating::get_error_location, InvalidStringError},
    utils::jump_spaces_and_then,
    ParsingError, ParsingResult, PklLexer, Statement,
};
use pkl_fast::lexer::PklToken;

#[derive(Debug)]
pub enum ImportClause<'a> {
    LocalFile(&'a Path),
    StandardLibrary(&'a str), // example: `pkl:math` with the pkl: stripped thus only leaving `math`
    Package(&'a str),
    Https(&'a str),
}

pub fn parse_import<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<Statement<'source>> {
    let value = parse_import_value(lexer)?;

    Ok(Statement::Import {
        clause: import_clause(value),
        imported_as: None,
    })
}

pub fn parse_globbed_import<'source>(
    lexer: &mut PklLexer<'source>,
) -> ParsingResult<Statement<'source>> {
    let value = parse_import_value(lexer)?;
    Ok(Statement::GlobbedImport(value))
}

fn parse_import_value<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<&'source str> {
    jump_spaces_and_then(lexer, |token, lexer| {
        if let Some(Ok(PklToken::StringLiteral)) = token {
            let raw_value = lexer.slice(); // retrieve value with quotes: "value"
            println!("raw : {:?}", raw_value);

            let value = &raw_value[1..raw_value.len() - 1];
            Ok(value)
        } else {
            Err(ParsingError::InvalidString(InvalidStringError {
                src: generate_source("main.pkl", lexer.source()),
                at: get_error_location(lexer).into(),
            }))
        }
    })
}

fn import_clause(value: &str) -> ImportClause {
    match value {
        value if value.starts_with("https://") => ImportClause::Https(value),
        value if value.starts_with("package://") => ImportClause::Package(value),
        value if value.starts_with("pkl:") => ImportClause::StandardLibrary(value),
        _ => ImportClause::LocalFile(&Path::new(value)),
    }
}
