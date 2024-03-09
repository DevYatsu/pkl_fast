use std::path::Path;

use crate::{
    parser::{utils::parse_string_literal, PklParser},
    prelude::ParsingResult,
};

#[derive(Debug, PartialEq, Clone)]
pub enum ImportClause<'a> {
    LocalFile(&'a Path),
    StandardLibrary(&'a str), // example: `pkl:math` with the pkl: stripped thus only leaving `math`
    Package(&'a str),
    Https(&'a str),
}

pub fn parse_import_value<'source>(parser: &mut PklParser<'source>) -> ParsingResult<&'source str> {
    let value = parse_string_literal(parser)?;

    Ok(value)
}

pub fn import_clause(value: &str) -> ImportClause {
    match value {
        value if value.starts_with("https://") => ImportClause::Https(value),
        value if value.starts_with("package://") => ImportClause::Package(value),
        value if value.starts_with("pkl:") => ImportClause::StandardLibrary(value),
        _ => ImportClause::LocalFile(&Path::new(value)),
    }
}
