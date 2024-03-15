use crate::{
    parser::utils::{line_ending_or_end, string::string_literal},
    prelude::ParsingResult,
};

use super::Statement;
use winnow::{ascii::multispace1, Parser};

pub fn amends_statement<'source>(input: &mut &'source str) -> ParsingResult<Statement<'source>> {
    // amends keyword already parsed
    let (_, value) = (multispace1, string_literal).parse_next(input)?;
    line_ending_or_end.parse_next(input)?;

    Ok(Statement::Amends(value))
}
