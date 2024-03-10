use winnow::{
    combinator::{fail, opt},
    dispatch,
    token::{one_of, take, take_while},
    PResult, Parser,
};

use super::PklValue;

pub fn int<'source>(input: &mut &'source str) -> PResult<i64> {
    let is_negative = opt(one_of(['+', '-'])).parse_next(input)?.is_some();

    let number = opt(dispatch!(take(2usize);
        "0b" => bin_digit1,
        "0o" => oct_digit1,
        "0x" => hex_digit1,
        _ => fail,
    ))
    .parse_next(input)?;

    if let Some(num) = number {
        if is_negative {
            return Ok(-num);
        } else {
            return Ok(num);
        }
    }

    let number = take_while(1.., ('0'..='9', '_'))
        .try_map(|int: &str| int.replace("_", "").parse::<i64>())
        .parse_next(input)?;

    if is_negative {
        return Ok(-number);
    } else {
        return Ok(number);
    }
}

fn bin_digit1<'source>(input: &mut &'source str) -> PResult<i64> {
    take_while(1.., ('0', '1', '_'))
        .try_map(|int: &str| i64::from_str_radix(&int.replace("_", ""), 2))
        .parse_next(input)
}
fn oct_digit1<'source>(input: &mut &'source str) -> PResult<i64> {
    take_while(1.., ('0'..='7', '_'))
        .try_map(|int: &str| i64::from_str_radix(&int.replace("_", ""), 8))
        .parse_next(input)
}
fn hex_digit1<'source>(input: &mut &'source str) -> PResult<i64> {
    take_while(1.., ('0'..='9', 'a'..='f', 'A'..='F', '_'))
        .try_map(|int: &str| i64::from_str_radix(&int.replace("_", ""), 16))
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
