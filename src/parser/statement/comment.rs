use crate::prelude::ParsingResult;

use super::Statement;
use winnow::{
    ascii::{newline, space0},
    combinator::separated,
    token::take_until,
    Parser,
};

pub fn line_comment<'source>(input: &mut &'source str) -> ParsingResult<Statement<'source>> {
    // '//' keyword already parsed
    let value = take_until(0.., '\n').parse_next(input)?;
    newline.parse_next(input)?;

    Ok(Statement::LineComment(value))
}

pub fn doc_comment<'source>(input: &mut &'source str) -> ParsingResult<Statement<'source>> {
    // '///' keyword already parsed
    let lines: Vec<&'source str> =
        separated(1.., take_until(0.., '\n'), (newline, space0, "///")).parse_next(input)?;
    newline.parse_next(input)?;

    Ok(Statement::DocComment { lines })
}

pub fn multiline_comment<'source>(input: &mut &'source str) -> ParsingResult<Statement<'source>> {
    // '/*' keyword already parsed
    let value = take_until(0.., "*/").parse_next(input)?;
    "*/".parse_next(input)?;

    Ok(Statement::MultiLineComment(value))
}
