use std::path::Path;

use winnow::{ascii::multispace1, stream::Stream, PResult, Parser};

use crate::parser::utils::string_literal;

use super::Statement;

#[derive(Debug, PartialEq, Clone)]
pub enum ImportClause<'a> {
    LocalFile(&'a Path),
    StandardLibrary(&'a str), // example: `pkl:math` with the pkl: stripped thus only leaving `math`
    Package(&'a str),
    Https(&'a str),
}

pub fn import_statement<'source>(input: &mut &'source str) -> PResult<Statement<'source>> {
    "import".parse_next(input)?;

    let start = input.checkpoint();

    if let Ok((_, _, value)) = ('*', multispace1, import_clause).parse_next(input) {
        return Ok(Statement::Import {
            clause: value,
            imported_as: None,
            is_globbed: true,
        });
    }

    input.reset(&start);
    let (_, value) = (multispace1, import_clause).parse_next(input)?;

    Ok(Statement::Import {
        clause: value,
        imported_as: None,
        is_globbed: false,
    })
}

pub fn import_clause<'source>(input: &mut &'source str) -> PResult<ImportClause<'source>> {
    let value = string_literal.parse_next(input)?;

    let result = match value {
        value if value.starts_with("https://") => ImportClause::Https(value),
        value if value.starts_with("package://") => ImportClause::Package(value),
        value if value.starts_with("pkl:") => ImportClause::StandardLibrary(value),
        _ => ImportClause::LocalFile(&Path::new(value)),
    };

    Ok(result)
}
