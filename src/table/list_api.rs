use crate::{PklResult, PklValue};
use std::ops::Range;

/// Based on v0.26.0
pub fn match_list_props_api<'a, 'b>(
    mut list: Vec<PklValue<'b>>,
    property: &'a str,
    range: Range<usize>,
) -> PklResult<PklValue<'b>> {
    match property {
        "length" => {
            return Ok(PklValue::Int(list.len() as i64));
        }
        "isEmpty" => {
            return Ok(PklValue::Bool(list.is_empty()));
        }
        "first" => {
            if list.is_empty() {
                return Err((
                    "Cannot get first element of an empty list!".to_owned(),
                    range,
                ));
            }

            return Ok(list.remove(0));
        }
        "firstOrNull" => {
            if list.is_empty() {
                return Ok(PklValue::Null);
            }

            return Ok(list.remove(0));
        }
        "rest" => {
            if list.is_empty() {
                return Err(("Cannot get the rest of an empty list!".to_owned(), range));
            }

            return Ok(PklValue::List(list.split_at(1).1.to_vec()));
        }
        "restOrNull" => {
            if list.is_empty() || list.len() == 1 {
                return Ok(PklValue::Null);
            }

            return Ok(PklValue::List(list.split_at(1).1.to_vec()));
        }
        "last" => {
            if list.is_empty() {
                return Err(("Cannot get last element of empty list!".to_owned(), range));
            }

            return Ok(list.last().unwrap().to_owned());
        }
        "lastOrNull" => {
            if list.is_empty() {
                return Ok(PklValue::Null);
            }

            return Ok(list.remove(list.len() - 1));
        }
        "single" => {
            if list.is_empty() || list.len() != 1 {
                return Err((
                    "Cannot get single element of a list with length != 1!".to_owned(),
                    range,
                ));
            }

            return Ok(list.remove(0));
        }
        "singleOrNull" => {
            if list.is_empty() || list.len() != 1 {
                return Ok(PklValue::Null);
            }

            return Ok(list.remove(0));
        }

        "lastIndex" => {
            if list.is_empty() {
                return Ok(PklValue::Int(-1));
            }

            return Ok(PklValue::Int((list.len() - 1) as i64));
        }

        "min" => return Err((format!("min property is not yet implemented"), range)),
        "minOrNull" => return Err((format!("minOrNull property is not yet implemented"), range)),
        "max" => return Err((format!("max property is not yet implemented"), range)),
        "maxOrNull" => return Err((format!("maxOrNull property is not yet implemented"), range)),

        "isDistinct" => return Err((format!("isDistinct property is not yet implemented"), range)),
        "distinct" => return Err((format!("distinct property is not yet implemented"), range)),

        _ => {
            return Err((
                format!("List does not possess {} property", property),
                range,
            ))
        }
    }
}
