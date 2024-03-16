use winnow::{
    ascii::{multispace0, space0},
    combinator::{cut_err, delimited, separated},
    token::one_of,
    PResult, Parser,
};

use crate::parser::utils::expected;

pub fn object_kind_list<'source, O, F>(
    field_parser: F,
) -> impl Fn(&mut &'source str) -> PResult<Vec<O>>
where
    F: Fn(&mut &'source str) -> PResult<O>,
{
    move |input: &mut &str| {
        '{'
            .context(expected("opening bracket"))
            .parse_next(input)?;
        multispace0.parse_next(input)?;

        let values = separated(
            0..,
            &field_parser,
            delimited(space0, one_of([';', '\n']), multispace0),
        )
        .parse_next(input)?;

        multispace0.parse_next(input)?;
        cut_err('}')
            .context(expected("closing bracket"))
            .parse_next(input)?;

        Ok(values)
    }
}
