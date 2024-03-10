use winnow::{
    ascii::digit1,
    combinator::{alt, cut_err, opt},
    token::{ one_of},
    PResult, Parser,
};

use super::PklValue;
use crate::parser::utils::expected;

pub fn float<'source>(input: &mut &'source str) -> PResult<f64> {
    let number = recognize_float(input)?;

    Ok(number)
}

fn recognize_float<'source>(input: &mut &'source str) -> PResult<f64> {
    alt((
        recognize_float_number,
        "NaN",
        (opt(one_of(['+', '-'])), "Infinity").recognize(),
    ))
    .parse_to()
    .parse_next(input)
}

fn recognize_float_number<'source>(input: &mut &'source str) -> PResult<&'source str> {
    (
        opt(one_of(['+', '-'])),
        alt(((digit1, ('.', (opt(digit1)))).void(), ('.', digit1).void())),
        opt((
            one_of(['e', 'E']),
            opt(one_of(['+', '-'])),
            cut_err(digit1).context(expected("exponential integer")),
        )),
    )
        .recognize()
        .parse_next(input)
}

impl<'a> Into<PklValue<'a>> for f64 {
    fn into(self) -> PklValue<'a> {
        PklValue::Float(self)
    }
}
