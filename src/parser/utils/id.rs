use winnow::{
    combinator::{alt, cut_err, preceded, terminated},
    token::{one_of, take_while},
    PResult, Parser,
};

use super::expected;

pub fn identifier<'source>(input: &mut &'source str) -> PResult<&'source str> {
    recognize_identifier
        .context(expected("identifier"))
        .parse_next(input)
}

pub fn cut_identifier<'source>(input: &mut &'source str) -> PResult<&'source str> {
    cut_err(identifier)
        .context(expected("identifier"))
        .parse_next(input)
}

pub fn recognize_identifier<'source>(input: &mut &'source str) -> PResult<&'source str> {
    alt((
        (
            one_of(('_', 'A'..='Z', 'a'..='z')),
            take_while(0.., ('0'..='9', 'A'..='Z', 'a'..='z', '_')),
        ),
        (
            preceded('`', one_of(('_', 'A'..='Z', 'a'..='z'))),
            terminated(take_while(0.., ('0'..='9', 'A'..='Z', 'a'..='z', '_')), '`'),
        ),
    ))
    .recognize()
    .parse_next(input)
}
