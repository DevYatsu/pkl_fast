use winnow::{
    combinator::{alt, fail, opt, preceded, repeat, terminated},
    dispatch,
    token::{one_of, take, take_while},
    PResult, Parser,
};

use super::PklValue;

/// Parsing a pkl integer, including 64-bit signed integer, written in decimal, hexadecimal, binary, or octal notations.
/// They can potentially include underscores as separators.
pub fn int<'source>(input: &mut &'source str) -> PResult<i64> {
    let operator = opt(one_of(['+', '-'])).parse_next(input)?;

    let number = opt(dispatch!(take(2usize);
        "0b" | "0B" => binary,
        "0o" | "0O" => octal,
        "0x" | "0X" => hexadecimal_value,
        _ => fail,
    ))
    .parse_next(input)?;

    if let Some(num) = number {
        match operator {
            Some('-') => return Ok(-num),
            _ => return Ok(num),
        }
    }

    let number = decimal.parse_next(input)?;

    match operator {
        Some('-') => Ok(-number),
        _ => Ok(number),
    }
}

fn decimal<'s>(input: &mut &'s str) -> PResult<i64> {
    repeat(
        1..,
        terminated(one_of('0'..='9'), repeat(0.., '_').map(|()| ())),
    )
    .map(|()| ())
    .recognize()
    .try_map(|out: &str| str::replace(&out, "_", "").parse::<i64>())
    .parse_next(input)
}

fn binary<'s>(input: &mut &'s str) -> PResult<i64> {
    repeat(
        1..,
        terminated(one_of('0'..='1'), repeat(0.., '_').map(|()| ())),
    )
    .map(|()| ())
    .recognize()
    .try_map(|out: &str| i64::from_str_radix(&str::replace(&out, "_", ""), 2))
    .parse_next(input)
}
fn octal<'s>(input: &mut &'s str) -> PResult<i64> {
    repeat(
        1..,
        terminated(one_of('0'..='7'), repeat(0.., '_').map(|()| ())),
    )
    .map(|()| ())
    .recognize()
    .try_map(|out: &str| i64::from_str_radix(&str::replace(&out, "_", ""), 8))
    .parse_next(input)
}

fn hexadecimal_value(input: &mut &str) -> PResult<i64> {
    repeat(
        1..,
        terminated(
            one_of(('0'..='9', 'a'..='f', 'A'..='F')),
            repeat(0.., '_').map(|()| ()),
        ),
    )
    .map(|()| ())
    .recognize()
    .try_map(|out: &str| i64::from_str_radix(&str::replace(&out, "_", ""), 16))
    .parse_next(input)
}

impl<'a> Into<PklValue<'a>> for i64 {
    fn into(self) -> PklValue<'a> {
        PklValue::Int(self)
    }
}

//**Another implementation of the function, it has potential but is not as fast**

// use winnow::{
//     combinator::{alt, opt, preceded},
//     token::{one_of, take_while},
//     PResult, Parser,
// };

// use super::PklValue;

// pub fn int<'source>(input: &mut &'source str) -> PResult<PklValue<'source>> {
//     let number = recognize_int.parse_next(input);
//     println!("{:?}", number);
//     Ok(number?.into())
// }

// fn recognize_int<'source>(input: &mut &'source str) -> PResult<i64> {
//     alt((
//         recognize_bin_number,
//         recognize_octal_number,
//         recognize_hex_number,
//         recognize_int_number,
//     ))
//     .parse_next(input)
// }

// fn recognize_int_number<'source>(input: &mut &'source str) -> PResult<i64> {
//     (opt(one_of(['+', '-'])), take_while(1.., ('0'..='9', '_')))
//         .recognize()
//         .try_map(|int: &str| int.replace("_", "").parse::<i64>())
//         .parse_next(input)
// }

// fn recognize_bin_number<'source>(input: &mut &'source str) -> PResult<i64> {
//     (
//         opt(one_of(['+', '-'])),
//         preceded("0b", take_while(1.., ('0', '1', '_'))),
//     )
//         .recognize()
//         .try_map(|int: &str| i64::from_str_radix(&int.replace("_", "").replace("0b", ""), 2))
//         .parse_next(input)
// }

// fn recognize_octal_number<'source>(input: &mut &'source str) -> PResult<i64> {
//     (
//         opt(one_of(['+', '-'])),
//         preceded("0o", take_while(1.., ('0'..='7', '_'))),
//     )
//         .recognize()
//         .try_map(|int: &str| i64::from_str_radix(&int.replace("_", "").replace("0o", ""), 8))
//         .parse_next(input)
// }
// fn recognize_hex_number<'source>(input: &mut &'source str) -> PResult<i64> {
//     (
//         opt(one_of(['+', '-'])),
//         preceded(
//             "0x",
//             take_while(1.., ('0'..='9', 'a'..='f', 'A'..='F', '_')),
//         ),
//     )
//         .recognize()
//         .try_map(|int: &str| i64::from_str_radix(&int.replace("_", "").replace("0x", ""), 16))
//         .parse_next(input)
// }
