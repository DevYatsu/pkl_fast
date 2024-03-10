use winnow::{
    token::{one_of, take_while},
    PResult, Parser,
};

use super::expected;

pub fn identifier<'source>(input: &mut &'source str) -> PResult<&'source str> {
    recognize_identifier
        .context(expected("identifier"))
        .parse_next(input)
}

pub fn recognize_identifier<'source>(input: &mut &'source str) -> PResult<&'source str> {
    (
        one_of(('_', 'A'..='Z', 'a'..='z')),
        take_while(0.., ('0'..='9', 'A'..='Z', 'a'..='z', '_')),
    )
        .recognize()
        .parse_next(input)
}
