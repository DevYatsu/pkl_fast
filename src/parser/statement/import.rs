use std::path::Path;

use crate::{
    parser::utils::parse_string_literal,
    prelude::{ParsingResult, PklLexer},
};

use super::Statement;

#[derive(Debug, PartialEq, Clone)]
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

    Ok(Statement::GlobbedImport {
        clause: import_clause(value),
        imported_as: None,
    })
}

fn parse_import_value<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<&'source str> {
    let value = parse_string_literal(lexer)?;

    Ok(value)
}

fn import_clause(value: &str) -> ImportClause {
    match value {
        value if value.starts_with("https://") => ImportClause::Https(value),
        value if value.starts_with("package://") => ImportClause::Package(value),
        value if value.starts_with("pkl:") => ImportClause::StandardLibrary(value),
        _ => ImportClause::LocalFile(&Path::new(value)),
    }
}
