use crate::generate_method;
use crate::{PklResult, PklValue};
use base64::prelude::*;
use std::ops::Range;

/// Based on v0.26.0
pub fn match_string_props_api(s: &str, property: &str, range: Range<usize>) -> PklResult<PklValue> {
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
        "isBlank" => return Ok(PklValue::Bool(s.trim().len() == 0).into()),
        "isRegex" => {
            return Err((
                "isRegex String API method not yet supported".to_owned(),
                range,
            )
                .into())
        }
        "md5" => return Err(("md5 String API method not yet supported".to_owned(), range).into()),
        "sha1" => {
            return Err(("sha1 String API method not yet supported".to_owned(), range).into())
        }
        "sha256" => {
            return Err((
                "sha256 String API method not yet supported".to_owned(),
                range,
            )
                .into())
        }
        "sha256Int" => {
            return Err((
                "sha256Int String API method not yet supported".to_owned(),
                range,
            )
                .into())
        }
        "base64" => return Ok(PklValue::String(BASE64_STANDARD.encode(s))),
        "base64Decoded" => {
            let buf: Vec<u8> = BASE64_STANDARD
                .decode(s)
                .map_err(|e| (format!("Failed to decode base64: {}", e), range.to_owned()))?;

            let s = std::str::from_utf8(&buf)
                .map_err(|e| (format!("Invalid UTF-8 sequence: {}", e), range))?;

            return Ok(PklValue::String(String::from(s)));
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
            )
                .into())
        }
    }
}

/// Based on v0.26.0
pub fn match_string_methods_api(
    s: &str,
    fn_name: &str,
    args: Vec<PklValue>,
    range: Range<usize>,
) -> PklResult<PklValue> {
    match fn_name {
        "getOrNull" => {
            generate_method!(
                "getOrNull", &args;
                0: Int;
                |index: i64| {
                    if let Some(s) = s.get(index as usize..(index+1) as usize) {
                        return Ok(String::from(s).into())
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
                        return Ok(String::from(s).into())
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
                |(start, exclusive_end): (i64, i64)| {
                    if start < 0 || start as usize >= s.len() || exclusive_end < start || exclusive_end as usize >= s.len() {
                        return Ok(().into())
                    }

                    if let Some(s) = s.get(start as usize..exclusive_end as usize) {
                        return Ok(String::from(s).into())
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
        "lastIndexOf" => {
            generate_method!(
                "lastIndexOf", &args;
                0: String;
                |pattern: String| {
                    let result = s.rfind(&pattern).ok_or((format!("Cannot use lastIndexOf to index pattern '{pattern}', it is not present in the string"), range))?;
                    Ok((result as i64).into())
                };
                range
            )
        }
        "lastIndexOfOrNull" => {
            generate_method!(
                "lastIndexOfOrNull", &args;
                0: String;
                |pattern: String| {
                    Ok(s.rfind(&pattern).map(|x| x as i64).map(PklValue::Int).unwrap_or(PklValue::Null))
                };
                range
            )
        }
        "take" => {
            generate_method!(
                "take", &args;
                0: Int;
                |n: i64| {
                    if n.is_negative() {return Err(("Cannot use take method with a negative index".to_owned(), range))}
                    Ok(s[..=(n as usize).min(s.len())].to_owned().into())
                };
                range
            )
        }
        "takeWhile" => {
            generate_method!(
                "takeWhile", &args;
                0: String;
                |pattern: String| {
                    Ok(s[..s.len() - s.trim_start_matches(&pattern).len()].to_owned().into())
                };
                range
            )
        }
        "takeLast" => {
            generate_method!(
                "takeLast", &args;
                0: Int;
                |n: i64| {
                    if n.is_negative() {return Err(("Cannot use takeLast method with a negative index".to_owned(), range))}
                    if n as usize >= s.len() {return Ok(String::from(s).into())}
                    Ok(s[s.len() - n as usize..].to_owned().into())
                };
                range
            )
        }
        "takeLastWhile" => {
            generate_method!(
                "takeLastWhile", &args;
                0: String;
                |_pattern: String| {
                    // Argument function not yet supported
                    return Err(("Function arguments are not yet supported!".to_owned(), range));
                    // Ok(s[s.len() - s.trim_end_matches(&pattern).len()..].to_owned().into())
                };
                range
            )
        }
        "drop" => {
            generate_method!(
                "drop", &args;
                0: Int;
                |n: i64| {
                    if n.is_negative() {return Err(("Cannot use drop method with a negative index".to_owned(), range))}
                    if n as usize >= s.len() {return Ok(String::new().into())}
                    Ok(s[n as usize..].to_owned().into())
                };
                range
            )
        }
        "dropWhile" => {
            generate_method!(
                "dropWhile", &args;
                0: Int;
                |_n: i64| {
                    // Argument function not yet supported
                    return Err(("Function arguments are not yet supported!".to_owned(), range));

                    // if n.is_negative() {return Err(("Cannot use dropWhile method with a negative index".to_owned(), range))}
                    // if n as usize >= s.len() {return Ok(String::new().into())}
                    // Ok(s[n as usize..].to_owned().into())
                };
                range
            )
        }
        "dropLast" => {
            generate_method!(
                "dropLast", &args;
                0: Int;
                |n: i64| {
                    if n.is_negative() {return Err(("Cannot use dropLast method with a negative index".to_owned(), range))}
                    if n as usize >= s.len() {return Ok(String::new().into())}
                    Ok(s[..s.len() - n as usize].to_owned().into())
                };
                range
            )
        }
        "dropLastWhile" => {
            generate_method!(
                "dropLastWhile", &args;
                0: Int;
                |_n: i64| {
                    // Argument function not yet supported
                    return Err(("Function arguments are not yet supported!".to_owned(), range));

                    // if n.is_negative() {return Err(("Cannot use dropWhile method with a negative index".to_owned(), range))}
                    // if n as usize >= s.len() {return Ok(String::new().into())}
                    // Ok(s[n as usize..].to_owned().into())
                };
                range
            )
        }
        "replaceFirst" => {
            generate_method!(
                "replaceFirst", &args;
                0: String, 1: String;
                |(pattern, replacement): (String, String)| {
                    Ok(s.replacen(&pattern, &replacement, 1).into())
                };
                range
            )
        }
        "replaceLast" => {
            generate_method!(
                "replaceLast", &args;
                0: String, 1: String;
                |(pattern, replacement): (String, String)| {
                    // fck this implementation is maybe wrong
                    if let Some(i) = s.rfind(&pattern) {
                        Ok((String::new() + &s[0..i] + &replacement + &s[i+pattern.len()..s.len()]).into())
                    }else {
                        Ok(String::from(s).into())
                    }
                };
                range
            )
        }
        "replaceAll" => {
            generate_method!(
                "replaceAll", &args;
                0: String, 1: String;
                |(pattern, replacement): (String, String)| {
                    Ok(s.replace(&pattern, &replacement).into())
                };
                range
            )
        }
        "replaceFirstMapped" => {
            generate_method!(
                "replaceFirstMapped", &args;
                0: String;
                |_pattern: String| {
                    // Argument function not yet supported
                    return Err(("Function arguments are not yet supported!".to_owned(), range));
                };
                range
            )
        }
        "replaceLastMapped" => {
            generate_method!(
                "replaceLastMapped", &args;
                0: String;
                |_pattern: String| {
                    // Argument function not yet supported
                    return Err(("Function arguments are not yet supported!".to_owned(), range));
                };
                range
            )
        }
        "replaceAllMapped" => {
            generate_method!(
                "replaceAllMapped", &args;
                0: String;
                |_pattern: String| {
                    // Argument function not yet supported
                     Err(("Function arguments are not yet supported!".to_owned(), range))
                };
                range
            )
        }
        "replaceRange" => {
            generate_method!(
                "replaceRange", &args;
                0: Int, 1: Int, 2:String;
                |(start, exclusive_end, replacement): (i64,i64, String)| {
                    if start.is_negative() {return Err(("Cannot use replaceRange method with a negative index (start)".to_owned(), range))}
                    if exclusive_end.is_negative() {return Err(("Cannot use replaceRange method with a negative index (exclusiveEnd)".to_owned(), range))}

                    if start as usize >= s.len() || exclusive_end as  usize > s.len() || start > exclusive_end {
                        return Ok(String::from(s).into()); // Invalid range, return the original string
                    }
                    let mut result = String::new();
                    result.push_str(&s[0..start as usize]);
                    result.push_str(&replacement);
                    result.push_str(&s[exclusive_end as usize..]);

                    Ok(result.into())
                };
                range
            )
        }
        "toUpperCase" => {
            generate_method!(
                "toUpperCase", &args;
                Ok(s.to_uppercase().into());
                range
            )
        }
        "toLowerCase" => {
            generate_method!(
                "toLowerCase", &args;
                Ok(s.to_lowercase().into());
                range
            )
        }
        "reverse" => {
            generate_method!(
                "reverse", &args;
                Ok(s.chars().rev().collect::<String>().into());
                range
            )
        }
        "trim" => {
            generate_method!(
                "trim", &args;
                Ok(s.trim().to_owned().into());
                range
            )
        }
        "trimStart" => {
            generate_method!(
                "trimStart", &args;
                Ok(s.trim_start().to_owned().into());
                range
            )
        }
        "trimEnd" => {
            generate_method!(
                "trimEnd", &args;
                Ok(s.trim_end().to_owned().into());
                range
            )
        }
        "padStart" => {
            generate_method!(
                "padStart", &args;
                0: Int, 1: String;
                |(width, character): (i64, String)| {
                    if character.len() != 1 {return Err(("padStart expects a Char (String(length = 1)), found String".to_owned(), range))}
                    if s.len() as i64 >= width {return Ok(String::from(s).into())}
                    let mut string = String::with_capacity(width as usize);
                    while (string.len() as i64) + (s.len() as i64) < width {
                        string.push_str(&character);
                    }
                    string.push_str(s);
                    Ok(string.into())
                };
                range
            )
        }
        "padEnd" => {
            generate_method!(
                "padEnd", &args;
                0: Int, 1: String;
                |(width, character): (i64, String)| {
                    if character.len() != 1 {return Err(("padEnd expects a Char (String(length = 1)), found String".to_owned(), range))}
                    if s.len() as i64 >= width {return Ok(String::from(s).into())}
                    let mut string = String::with_capacity(width as usize);
                    string.push_str(s);
                    while (string.len() as i64) < width {
                        string.push_str(&character);
                    }
                    Ok(string.into())
                };
                range
            )
        }
        "split" => {
            generate_method!(
                "split", &args;
                0: String;
                |pattern: String| {
                    let split_strings: Vec<String> = s.split(&pattern).map(String::from).collect();
                    let pkl_values: Vec<PklValue> = split_strings.into_iter().map(PklValue::String).collect();
                    Ok(PklValue::List(pkl_values))                };
                range
            )
        }
        "capitalize" => {
            generate_method!(
                "capitalize", &args;
                {
                    let mut chars = s.chars();
                    let new_s = match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    };
                    Ok(new_s.into())
                };
                range
            )
        }
        "decapitalize" => {
            generate_method!(
                "decapitalize", &args;
                {
                    let mut chars = s.chars();
                    let new_s = match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
                    };
                    Ok(new_s.into())
                };
                range
            )
        }
        "toInt" => {
            generate_method!(
                "toInt", &args;
                {
                    match s.parse::<i64>() {
                        Ok(result) => Ok(PklValue::Int(result)),
                        Err(e) => Err((format!("Failed to convert string to Int: {}", e), range).into())
                    }
                };
                range
            )
        }
        "toIntOrNull" => {
            generate_method!(
                "toIntOrNull", &args;
                {
                    match s.parse::<i64>() {
                        Ok(result) => Ok(PklValue::Int(result)),
                        Err(_) => Ok(PklValue::Null)
                    }
                };
                range
            )
        }
        "toFloat" => {
            generate_method!(
                "toFloat", &args;
                {
                    match s.parse::<f64>() {
                        Ok(result) => Ok(PklValue::Float(result)),
                        Err(e) => Err((format!("Failed to convert string to Float: {}", e), range).into())
                    }
                };
                range
            )
        }
        "toFloatOrNull" => {
            generate_method!(
                "toFloatOrNull", &args;
                {
                    match s.parse::<f64>() {
                        Ok(result) => Ok(PklValue::Float(result)),
                        Err(_) => Ok(PklValue::Null)
                    }
                };
                range
            )
        }
        "toBoolean" => {
            generate_method!(
                "toBoolean", &args;
                {
                    match s {
                        "true" => Ok(true.into()),
                        "false" => Ok(false.into()),
                        x => Err((format!("Failed to convert string to Boolean: '{x}' is neither equal to true nor false"), range).into())
                    }
                };
                range
            )
        }
        "toBooleanOrNull" => {
            generate_method!(
                "toBooleanOrNull", &args;
                {
                    match s {
                        "true" => Ok(true.into()),
                        "false" => Ok(false.into()),
                        _ => Ok(PklValue::Null)
                    }
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
            )
                .into())
        }
    }
}
