use crate::parser::utils::string_literal;

use super::Statement;
use winnow::{ascii::multispace1, PResult, Parser};

pub fn amends_statement<'source>(input: &mut &'source str) -> PResult<Statement<'source>> {
    let (_, value) = (multispace1, string_literal).parse_next(input)?;

    Ok(Statement::Amends(value))
}
