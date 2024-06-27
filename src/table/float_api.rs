use super::{
    data_size,
    duration::{self, Duration},
};
use crate::{values::Byte, PklResult, PklValue};
use std::ops::Range;

/// Based on v0.26.0
pub fn match_float_props_api<'a, 'b>(
    float: f64,
    property: &'a str,
    range: Range<usize>,
) -> PklResult<PklValue<'b>> {
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
