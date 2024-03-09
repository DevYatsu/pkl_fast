use std::path::Path;

use winnow::{
    ascii::{multispace1, space1},
    combinator::{opt, preceded},
    PResult, Parser,
};

use crate::parser::utils::{expected, identifier, string_literal};

use super::Statement;

#[derive(Debug, PartialEq, Clone)]
pub enum ImportClause<'a> {
    LocalFile(&'a Path),
    StandardLibrary(&'a str), // example: `pkl:math` with the pkl: stripped thus only leaving `math`
    Package(&'a str),
    Https(&'a str),
}

pub fn import_statement<'source>(input: &mut &'source str) -> PResult<Statement<'source>> {
    let is_globbed = opt('*').parse_next(input)?.is_some();
    let value = preceded(multispace1, import_clause).parse_next(input)?;
    let imported_as = opt(parse_as).parse_next(input)?;

    Ok(Statement::Import {
        clause: value,
        imported_as,
        is_globbed,
    })
}

fn parse_as<'source>(input: &mut &'source str) -> PResult<&'source str> {
    multispace1.parse_next(input)?;
    "as".parse_next(input)?;
    multispace1.parse_next(input)?;
    identifier.parse_next(input)
}

pub fn import_clause<'source>(input: &mut &'source str) -> PResult<ImportClause<'source>> {
    let value = string_literal
        .context(expected("import clause"))
        .parse_next(input)?;

    let result = match value {
        value if value.starts_with("https://") => ImportClause::Https(value),
        value if value.starts_with("package://") => ImportClause::Package(value),
        value if value.starts_with("pkl:") => ImportClause::StandardLibrary(value),
        _ => ImportClause::LocalFile(&Path::new(value)),
    };

    Ok(result)
}
