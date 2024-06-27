use crate::generate_method;
use crate::{PklResult, PklValue};
use base64::prelude::*;
use std::ops::Range;

/// Based on v0.26.0
pub fn match_string_props_api<'a, 'b>(
    s: &'a str,
    property: &'a str,
    range: Range<usize>,
) -> PklResult<PklValue<'b>> {
    match property {
        "length" => return Ok(PklValue::Int(s.len() as i64)),
        "lastIndex" => {
            return Ok(PklValue::Int({
                if s.len() == 0 {
                    -1
                } else {
                    (s.len() - 1) as i64
                }
            }))
        }
        "isEmpty" => return Ok(PklValue::Bool(s.len() == 0)),
        "isBlank" => return Ok(PklValue::Bool(s.trim().len() == 0)),
        "isRegex" => {
            return Err((
                "isRegex String API method not yet supported".to_owned(),
                range,
            ))
        }
        "md5" => return Err(("md5 String API method not yet supported".to_owned(), range)),
        "sha1" => return Err(("sha1 String API method not yet supported".to_owned(), range)),
        "sha256" => {
            return Err((
                "sha256 String API method not yet supported".to_owned(),
                range,
            ))
        }
        "sha256Int" => {
            return Err((
                "sha256Int String API method not yet supported".to_owned(),
                range,
            ))
        }
        "base64" => return Ok(PklValue::String(BASE64_STANDARD.encode(s))),
        "base64Decoded" => {
            let buf: Vec<u8> = BASE64_STANDARD.decode(s).map_err(|e| {
                (
                    format!("Failed to decode base64: {}", e.to_string()),
                    range.to_owned(),
                )
            })?;

            let s = std::str::from_utf8(&buf)
                .map_err(|e| (format!("Invalid UTF-8 sequence: {}", e.to_string()), range))?;

            return Ok(PklValue::String(s.to_owned()));
        }
        "chars" => {
            let chars = s
                .chars()
                .into_iter()
                .map(|c| PklValue::String(c.to_string()))
                .collect::<Vec<_>>();

            // typealias Char = String(length == 1)
            return Ok(PklValue::List(chars));
        }
        "codePoints" => {
            // would be better to have the Int as an u32
            let codepoints = s
                .chars()
                .into_iter()
                .map(|c| PklValue::Int(c as i64))
                .collect::<Vec<_>>();

            return Ok(PklValue::List(codepoints));
        }
        _ => {
            return Err((
                format!("String does not possess {} property", property),
                range,
            ))
        }
    }
}

/// Based on v0.26.0
pub fn match_string_methods_api<'a, 'b>(
    s: &'a str,
    fn_name: &'a str,
    args: Vec<PklValue<'b>>,
    range: Range<usize>,
) -> PklResult<PklValue<'b>> {
    match fn_name {
        "getOrNull" => {
            generate_method!(
                "getOrNull", &args;
                0: Int;
                |index: i64| {
                    if let Some(s) = s.get(index as usize..(index+1) as usize) {
                        return Ok(s.to_owned().into())
                    }

                    Ok(().into())
                };
                range
            )
        }
        "substring" => {
            generate_method!(
                "substring", &args;
                0: Int, 1: Int;
                |(start, exclusive_end): (i64, i64)| {
                    if start < 0 || start as usize >= s.len() {
                        return Err(("start index is out of bound".to_owned(), range))
                    }
                    if exclusive_end < start || exclusive_end as usize >= s.len() {
                        return Err(("exclusiveEnd index is out of bound".to_owned(), range))
                    }

                    if let Some(s) = s.get(start as usize..exclusive_end as usize) {
                        return Ok(s.to_owned().into())
                    }

                    Ok(().into())
                };
                range
            )
        }
        "substringOrNull" => {
            generate_method!(
                "substringOrNull", &args;
                0: Int, 1: Int;
                |(start, exclusiveEnd): (i64, i64)| {
                    if start < 0 || start as usize >= s.len() || exclusiveEnd < start || exclusiveEnd as usize >= s.len() {
                        return Ok(().into())
                    }

                    if let Some(s) = s.get(start as usize..exclusiveEnd as usize) {
                        return Ok(s.to_owned().into())
                    }

                    Ok(().into())
                };
                range
            )
        }
        "repeat" => {
            generate_method!(
                "repeat", &args;
                0: Int;
                |index: i64| {
                    Ok(s.repeat(index as usize).into())
                };
                range
            )
        }
        "contains" => {
            generate_method!(
                "contains", &args;
                0: String;
                |pattern: String| {
                    Ok(s.contains(&pattern).into())
                };
                range
            )
        }
        "matches" => {
            generate_method!(
                "matches", &args;
                0: String;
                |pattern: String| {
                     Ok((s.matches(&pattern).count() != 0).into())
                };
                range
            )
        }
        "startsWith" => {
            generate_method!(
                "startsWith", &args;
                0: String;
                |pattern: String| {
                    Ok(s.starts_with(&pattern).into())
                };
                range
            )
        }
        "endsWith" => {
            generate_method!(
                "endsWith", &args;
                0: String;
                |pattern: String| {
                    Ok(s.ends_with(&pattern).into())
                };
                range
            )
        }
        "indexOf" => {
            generate_method!(
                "indexOf", &args;
                0: String;
                |pattern: String| {
                    let result = s.find(&pattern).ok_or((format!("Cannot use indexOf to index pattern '{pattern}', it is not present in the string"), range))?;
                    Ok((result as i64).into())
                };
                range
            )
        }
        "indexOfOrNull" => {
            generate_method!(
                "indexOfOrNull", &args;
                0: String;
                |pattern: String| {
                    Ok(s.find(&pattern).map(|x| x as i64).map(PklValue::Int).unwrap_or(PklValue::Null))
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
