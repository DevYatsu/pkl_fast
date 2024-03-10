use winnow::{
    ascii::{digit1, hex_digit1, oct_digit1},
    combinator::{alt, cut_err, fail, opt, todo},
    dispatch,
    stream::ParseSlice,
    token::{one_of, take, take_while},
    PResult, Parser,
};

use super::PklValue;
use crate::parser::utils::expected;

pub fn float<'source>(input: &mut &'source str) -> PResult<PklValue<'source>> {
    let number = recognize_float(input)?;

    Ok(number.into())
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
        alt(((digit1, ('.', opt(digit1))).void(), ('.', digit1).void())),
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
