use winnow::{token::take_while, PResult, Parser};

use super::expected;

pub fn identifier<'source>(input: &mut &'source str) -> PResult<&'source str> {
    take_while(0.., ('0'..='9', 'A'..='Z', 'a'..='z', '_'))
        .verify(|s: &str| s.len() >= 1 && !s.chars().next().unwrap().is_ascii_digit())
        .context(expected("identifier"))
        .parse_next(input)
}
