use winnow::{
    combinator::cut_err,
    token::{any, take_until},
    PResult, Parser,
};

use super::expected;

pub fn string_literal<'source>(input: &mut &'source str) -> PResult<&'source str> {
    any.verify(|c| *c == '"').parse_next(input)?;

    let str_content = cut_err(take_until(0.., '"'))
        .verify(|s: &str| !s.contains('\\'))
        .context(expected("string literal"))
        .parse_next(input)?;

    '"'.parse_next(input)?;

    Ok(str_content)
}
