use winnow::{
    token::{any, take_until},
    PResult, Parser,
};

use super::expected;

pub fn string_literal<'source>(input: &mut &'source str) -> PResult<&'source str> {
    any.verify(|c| *c == '"').parse_next(input)?;

    let str_content = take_until(0.., '"')
        .context(expected("string literal"))
        .verify(|s: &str| !s.contains('\\'))
        .parse_next(input)?;

    '"'.parse_next(input)?;

    Ok(str_content)
}
