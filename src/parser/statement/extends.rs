use winnow::{ascii::multispace1, PResult, Parser};

use crate::parser::utils::string_literal;

use super::Statement;
pub fn extends_statement<'source>(input: &mut &'source str) -> PResult<Statement<'source>> {
    // extends keyword already parsed
    let (_, value) = (multispace1, string_literal).parse_next(input)?;

    Ok(Statement::Extends(value))
}
