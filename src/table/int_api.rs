use super::{
    data_size,
    duration::{self, Duration},
};
use crate::{generate_method, values::Byte, PklResult, PklValue};
use std::ops::Range;

/// Based on v0.26.0
pub fn match_int_props_api<'a, 'b>(
    int: i64,
    property: &'a str,
    range: Range<usize>,
) -> PklResult<PklValue<'b>> {
    if let Some(unit) = duration::Unit::from_str(property) {
        return Ok(PklValue::Duration(Duration::from_int_and_unit(int, unit)));
    }

    if let Some(unit) = data_size::Unit::from_str(property) {
        return Ok(PklValue::DataSize(Byte::from_int_and_unit(int, unit)));
    }

    match property {
        "sign" => {
            if int > 0 {
                return Ok(PklValue::Int(1));
            }
            if int < 0 {
                return Ok(PklValue::Int(-1));
            }

            return Ok(PklValue::Int(0));
        }
        "abs" => {
            return Ok(PklValue::Int(int.abs()));
        }
        "ceil" => return Ok(PklValue::Int(int)),
        "floor" => return Ok(PklValue::Int(int)),
        "isPositive" => return Ok(PklValue::Bool(int.is_positive())),
        "isFinite" => return Ok(PklValue::Bool(true)),
        "isNaN" => return Ok(PklValue::Bool(false)),
        "isNonZero" => return Ok(PklValue::Bool(int != 0)),
        "inv" => return Ok(PklValue::Int(!int)),
        "isEven" => return Ok(PklValue::Bool(int % 2 == 0)),
        "isOdd" => return Ok(PklValue::Bool(int % 2 == 1)),
        _ => return Err((format!("Int does not possess {} property", property), range)),
    }
}

/// Based on v0.26.0
pub fn match_int_methods_api<'a, 'b>(
    int: i64,
    fn_name: &'a str,
    args: Vec<PklValue<'b>>,
    range: Range<usize>,
) -> PklResult<PklValue<'b>> {
    match fn_name {
        "toString" => {
            generate_method!(
                "toString", &args;
                Ok(int.to_string().into());
                range
            )
        }
        "round" => {
            generate_method!(
                "round", &args;
                Ok(int.into());
                range
            )
        }
        "truncate" => {
            generate_method!(
                "truncate", &args;
                Ok(int.into());
                range
            )
        }
        "toInt" => {
            generate_method!(
                "toInt", &args;
                Ok(int.into());
                range
            )
        }
        "toFixed" => {
            generate_method!(
                "toFixed", &args;
                0: Int;
                |fraction_digits: i64|
                    {
                        if fraction_digits < 0 || fraction_digits > 20 {
                            return Err((format!("fractionDigits must be in range 0..20, here it is '{}'", fraction_digits), range))
                        }
                        Ok(format!("{:.1$}", int, fraction_digits as usize).into())
                    }
                ;
                range
            )
        }
        "toDuration" => {
            generate_method!(
                "toDuration", &args;
                0: String;
                |duration_unit: String|
                    {
                        if let Some(unit) = duration::Unit::from_str(&duration_unit) {
                            return Ok(Duration::from_int_and_unit(int, unit).into())
                        }

                        return Err((format!("Cannot convert {} to Duration, durationUnit '{}' is not valid", int, duration_unit), range))
                    }
                ;
                range
            )
        }
        "toDataSize" => {
            generate_method!(
                "toDataSize", &args;
                0: String;
                |datasize_unit: String|
                    {
                        if let Some(unit) = data_size::Unit::from_str(&datasize_unit) {
                            return Ok(Byte::from_int_and_unit(int, unit).into())
                        }

                        return Err((format!("Cannot convert {} to DataSize, datasizeUnit '{}' is not valid", int, datasize_unit), range))
                    }
                ;
                range
            )
        }
        "isBetween" => {
            generate_method!(
                "isBetween", &args;
                Numbers: 2;
                |[start, inclusive_end]: [f64; 2]|
                    {
                        Ok((int as f64 >= start && int as f64 <= inclusive_end).into())
                    }
                ;
                range
            )
        }
        "toRadixString" => {
            generate_method!(
                "toRadixString", &args;
                0: Int;
                |radix: i64|
                    {
                        if radix < 2 || radix > 36 {
                            return Err((format!("Radix must be in range 2..36, here it is '{}'", radix), range))
                        }

                        if int == 0 {
                            return Ok("0".to_owned().into());
                        }

                        let mut result = Vec::new();
                        let digits = "0123456789abcdefghijklmnopqrstuvwxyz".as_bytes();

                        let int_ref = &mut int.clone();

                        while *int_ref > 0 {
                            let remainder = (*int_ref % radix) as usize;
                            result.push(digits[remainder] as char);
                            *int_ref /= radix;
                        }

                        result.reverse();
                        let s = result.iter().collect::<String>();

                        Ok(s.into())
                    }
                ;
                range
            )
        }
        "shl" => {
            generate_method!(
                "shl", &args;
                0: Int;
                |n: i64|
                Ok((int << (8 * n)).into());
                range
            )
        }
        "shr" => {
            generate_method!(
                "shr", &args;
                0: Int;
                |n: i64|
                Ok((int >> n).into());
                range
            )
        }
        // not sure 'bout this one
        "ushr" => {
            generate_method!(
                "ushr", &args;
                0: Int;
                |n: i64|
                    Ok(((int as u64 >> n as u64) as i64).into());
                range
            )
        }
        "and" => {
            generate_method!(
                "and", &args;
                0: Int;
                |n: i64|
                    Ok((int & n).into());
                range
            )
        }
        "or" => {
            generate_method!(
                "and", &args;
                0: Int;
                |n: i64|
                    Ok((int | n).into());
                range
            )
        }
        "xor" => {
            generate_method!(
                "and", &args;
                0: Int;
                |n: i64|
                    Ok((int ^ n).into());
                range
            )
        }
        "toChar" => {
            generate_method!(
                "toChar", &args;
                {
                    if int > 0x10FFFF &&  int < 0 {
                        return Err((format!("Cannot convert {int} to char, it is not a valid unicode code point"), range))
                    }

                    Ok((std::char::from_u32(int as u32).unwrap() as i64).into())
                };
                range
            )
        }
        _ => {
            return Err((
                format!(
                    "String does not possess {} method (or method not yet implemented)",
                    fn_name
                ),
                range,
            ))
        }
    }
}
