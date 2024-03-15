use winnow::{ascii::multispace1, Parser};

use crate::{
    parser::utils::{line_ending_or_end, string::string_literal},
    prelude::ParsingResult,
};

use super::Statement;
pub fn extends_statement<'source>(input: &mut &'source str) -> ParsingResult<Statement<'source>> {
    // extends keyword already parsed
    let (_, value) = (multispace1, string_literal).parse_next(input)?;
    line_ending_or_end.parse_next(input)?;

    Ok(Statement::Extends(value))
}
