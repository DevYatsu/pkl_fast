use super::{
    data_size,
    duration::{self, Duration},
};
use crate::{values::Byte, PklResult, PklValue};
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
