use winnow::{
    combinator::{alt, cut_err, opt, repeat, terminated},
    token::one_of,
    PResult, Parser,
};

use super::PklValue;
use crate::parser::utils::expected;

/// Parsing a 64-bit double-precision floating point number, using decimal notation.
/// They consist of an integer part, decimal point, fractional part, and exponent part. The integer and exponent part are optional.
///
/// **WE URGENTLY NEED TO ADD SUPPORT FOR _ IN FLOATS!!**
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
        alt((
            (decimal, ('.', (opt(decimal)))).void(),
            ('.', decimal).void(),
        )),
        opt((
            one_of(['e', 'E']),
            opt(one_of(['+', '-'])),
            cut_err(decimal).context(expected("exponential integer")),
        )),
    )
        .recognize()
        .parse_next(input)
}

fn decimal<'s>(input: &mut &'s str) -> PResult<&'s str> {
    repeat(
        1..,
        terminated(one_of('0'..='9'), repeat(0.., '_').map(|()| ())),
    )
    .map(|()| ())
    .recognize()
    .parse_next(input)
}

impl<'a> Into<PklValue<'a>> for f64 {
    fn into(self) -> PklValue<'a> {
        PklValue::Float(self)
    }
}
