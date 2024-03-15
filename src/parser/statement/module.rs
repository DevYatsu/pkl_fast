use winnow::{
    ascii::{alphanumeric1, multispace1},
    combinator::{cut_err, repeat, terminated},
     Parser,
};

use crate::{parser::utils::{cut_multispace1, expected, line_ending_or_end}, prelude::ParsingResult};

use super::Statement;
pub fn module_statement<'source>(input: &mut &'source str) -> ParsingResult<Statement<'source>> {
    // 'module' keyword already parsed

    cut_multispace1.parse_next(input)?;
    let value = cut_module_value.parse_next(input)?;
    line_ending_or_end.parse_next(input)?;

    Ok(Statement::Module { value, open: false })
}

pub fn open_module_statement<'source>(input: &mut &'source str) -> ParsingResult<Statement<'source>> {
    // open keyword already parsed

    // do not throw a cut error as the `open` keyword is not necessarily followed by a module
    multispace1.parse_next(input)?;
    "module".parse_next(input)?;
    cut_multispace1.parse_next(input)?;
    let value = cut_module_value.parse_next(input)?;

    Ok(Statement::Module { value, open: true })
}

fn cut_module_value<'source>(input: &mut &'source str) -> ParsingResult<&'source str> {
    cut_err(module_value)
        .context(expected("module name"))
        .parse_next(input)
}

/// Parse dot-separated identifier segments recursively
fn module_value<'source>(input: &mut &'source str) -> ParsingResult<&'source str> {
    repeat(
        1..,
        terminated(alphanumeric1, repeat(0.., '.').map(|()| ())),
    )
    .map(|()| ())
    .recognize()
    .parse_next(input)
}
