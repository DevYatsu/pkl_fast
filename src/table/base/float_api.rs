use super::{
    data_size,
    duration::{self, Duration},
};
use crate::{generate_method, values::Byte, PklResult, PklValue};
use std::ops::Range;

/// Based on v0.26.0
pub fn match_float_props_api(
    float: f64,
    property: &str,
    range: Range<usize>,
) -> PklResult<PklValue> {
    if let Some(unit) = duration::Unit::from_str(property) {
        return Ok(PklValue::Duration(Duration::from_float_and_unit(
            float, unit,
        )));
    }

    if let Some(unit) = data_size::Unit::from_str(property) {
        return Ok(PklValue::DataSize(Byte::from_float_and_unit(float, unit)));
    }

    match property {
        "sign" => {
            if float > 0.0 {
                return Ok(PklValue::Int(1));
            }
            if float < 0.0 {
                return Ok(PklValue::Int(-1));
            }

            return Ok(PklValue::Int(0));
        }
        "abs" => {
            return Ok(PklValue::Float(float.abs()));
        }
        "ceil" => return Ok(PklValue::Float(float.ceil())),
        "floor" => return Ok(PklValue::Float(float.floor())),
        "isPositive" => return Ok(PklValue::Bool(float >= 0.0)),
        "isFinite" => return Ok(PklValue::Bool(float.is_finite())),
        "isInfinite" => return Ok(PklValue::Bool(float.is_infinite())),
        "isNaN" => return Ok(PklValue::Bool(float.is_nan())),
        "isNonZero" => return Ok(PklValue::Bool(float != 0.0)),

        "isEven" => return Err(("Float does not possess isEven property".to_owned(), range)),
        "isOdd" => return Err(("Float does not possess isOdd property".to_owned(), range)),
        "inv" => {
            return Err((
                "Cannot apply bitwise NOT operator on floats".to_owned(),
                range,
            ))
        }
        _ => {
            return Err((
                format!("Float does not possess {} property", property),
                range,
            ))
        }
    }
}

/// Based on v0.26.0
pub fn match_float_methods_api(
    float: f64,
    fn_name: &str,
    args: Vec<PklValue>,
    range: Range<usize>,
) -> PklResult<PklValue> {
    match fn_name {
        "toString" => {
            generate_method!(
                "toString", &args;
                Ok(float.to_string().into());
                range
            )
        }
        "round" => {
            generate_method!(
                "round", &args;
                Ok(float.round_ties_even().into());
                range
            )
        }
        "truncate" => {
            generate_method!(
                "truncate", &args;
                Ok(float.trunc().into());
                range
            )
        }
        "toInt" => {
            generate_method!(
                            "toInt", &args;
                            {
                                let value = float.trunc();
                                if value.is_infinite() {
                                    return Err(("Cannot convert Float to Int, float represents infinity".to_owned(), range))
                                }else if value.is_nan() {
                                    return Err(("Cannot convert Float to Int, float is NaN".to_owned(), range))
                                }
            else
                               if value > i64::MAX as f64 {
                                   return Err(("Cannot convert Float to Int, float is too large".to_owned(), range))
                               }else if value < i64::MIN as f64 {
                                   return Err(("Cannot convert Float to Int, float is too large".to_owned(), range))
                               }
                               Ok((value as i64).into())}
                            ;
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
                        Ok(format!("{:.1$}", float, fraction_digits as usize).into())
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
                            return Ok(Duration::from_float_and_unit(float, unit).into())
                        }

                        return Err((format!("Cannot convert {} to Duration, durationUnit '{}' is not valid", float, duration_unit), range))
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
                            return Ok(Byte::from_float_and_unit(float, unit).into())
                        }

                        return Err((format!("Cannot convert {} to DataSize, datasizeUnit '{}' is not valid", float, datasize_unit), range))
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
                        Ok((float >= start && float <= inclusive_end).into())
                    }
                ;
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
