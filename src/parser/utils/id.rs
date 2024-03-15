use winnow::{
    combinator::{alt, cut_err, terminated},
    token::{one_of, take_until, take_while},
    Parser,
};

use crate::prelude::ParsingResult;

use super::{expected, GLOBAL_KEYWORDS};

/// A valid Pkl identifier ([see](https://pkl-lang.org/main/current/language-reference/index.html#quoted-identifiers)).
pub fn identifier<'source>(input: &mut &'source str) -> ParsingResult<&'source str> {
    recognize_identifier.parse_next(input)
}

/// A valid identifier that is not a Pkl's keyword, this parser is bound to only be used when declaring variables.
pub fn identifier_not_keyword<'source>(input: &mut &'source str) -> ParsingResult<&'source str> {
    recognize_identifier
        .verify(|id: &str| !GLOBAL_KEYWORDS.contains(&id))
        .parse_next(input)
}

pub fn cut_identifier<'source>(input: &mut &'source str) -> ParsingResult<&'source str> {
    cut_err(identifier)
        .context(expected("identifier"))
        .parse_next(input)
}

pub fn recognize_identifier<'source>(input: &mut &'source str) -> ParsingResult<&'source str> {
    alt((
        // an identifier conforming to Unicode’s UAX31-R1-1 syntax
        (
            one_of(('_', '$', 'A'..='Z', 'a'..='z')),
            take_while(0.., ('0'..='9', 'A'..='Z', 'a'..='z')),
        ),
        // an illegal identifier
        ('`', terminated(take_until(0.., '`'), '`')),
    ))
    .recognize()
    .parse_next(input)
}
