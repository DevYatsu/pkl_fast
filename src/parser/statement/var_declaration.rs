use crate::{
    parser::utils::{cut_multispace1, expected, var::variable},
    prelude::ParsingResult,
};
use winnow::{
    combinator::{cut_err, opt, terminated},
    Parser,
};

use super::Statement;

pub fn var_statement<'source>(input: &mut &'source str) -> ParsingResult<Statement<'source>> {
    let is_local = is_local.parse_next(input)?;

    if is_local {
        let (name, optional_type, value) = cut_err(variable)
            .context(expected("identifier"))
            .parse_next(input)?;

        return Ok(Statement::VariableDeclaration {
            name,
            optional_type,
            value,
            is_local,
        });
    }

    let (name, optional_type, value) = variable.parse_next(input)?;
    Ok(Statement::VariableDeclaration {
        name,
        optional_type,
        value,
        is_local,
    })
}

pub fn is_local<'source>(input: &mut &'source str) -> ParsingResult<bool> {
    opt(terminated("local", cut_multispace1))
        .map(|opt| opt.is_some())
        .parse_next(input)
}
