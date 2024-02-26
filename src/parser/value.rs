use self::datasize::{DataSize, DataSizeValue};
use self::duration::{Duration, DurationUnit, DurationValue};
use super::PklLexer;
use crate::parser::value::datasize::DataSizeUnit;
use crate::parser::{errors::ParsingError, ParsingResult};
use crate::prelude::PklToken;
use std::collections::HashMap;

mod datasize;
mod duration;

#[derive(Debug, PartialEq, Clone)]
/// An enum representing any Pkl value
pub enum PklValue<'a> {
    String(&'a str),
    Boolean(bool),
    Int(i64),
    Float(f64),
    Object(HashMap<&'a str, PklValue<'a>>),

    List(Vec<PklValue<'a>>),
    Listing(Vec<PklValue<'a>>),

    Map(Vec<PklValue<'a>>),

    /// For now, only indexing with &str is supported.
    /// In the future we shall support other any data type as key!
    Mapping(HashMap<&'a str, PklValue<'a>>),

    Duration(Duration),
    DataSize(DataSize),
    Null,
}

pub fn parse_value<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<PklValue<'source>> {
    if let Some(token) = lexer.next() {
        if let Err(e) = token {
            return Err(ParsingError::lexing(lexer, e));
        }

        match token.unwrap() {
            PklToken::Boolean(b) => Ok(PklValue::Boolean(b)),
            PklToken::StringLiteral => {
                let raw_value = lexer.slice();
                Ok(PklValue::String(&raw_value[1..raw_value.len() - 1]))
            }
            PklToken::Integer => {
                let raw_value: &str = lexer.slice();

                // Remove underscores from the string
                let clean_value = raw_value.replace("_", "");

                // Check if the value starts with a radix specifier
                let parsed_value = if clean_value.starts_with("0x") {
                    // Parse hexadecimal value
                    i64::from_str_radix(&clean_value[2..], 16)
                } else if clean_value.starts_with("0b") {
                    // Parse binary value
                    i64::from_str_radix(&clean_value[2..], 2)
                } else if clean_value.starts_with("0o") {
                    // Parse octal value
                    i64::from_str_radix(&clean_value[2..], 8)
                } else {
                    // Parse decimal value
                    clean_value.parse::<i64>()
                };

                Ok(PklValue::Int(parsed_value?))
            }
            PklToken::Float => {
                let raw_value = lexer.slice();
                let clean_value = raw_value.parse::<f64>();
                Ok(PklValue::Float(clean_value?))
            }
            PklToken::Null => Ok(PklValue::Null),
            PklToken::OpenBracket => {
                todo!("Cannot proceed objects for now")
            }
            PklToken::DataSize => match lexer.slice().split('.').collect::<Vec<_>>().as_slice() {
                [value, unit] => {
                    let value: DataSizeValue = value.parse::<i64>()?.into();
                    let unit: DataSizeUnit = (*unit).into();
                    Ok(PklValue::DataSize(DataSize { value, unit }))
                }
                [value, frac, unit] => {
                    let value: DataSizeValue = format!("{}.{}", value, frac).parse::<f64>()?.into();
                    let unit: DataSizeUnit = (*unit).into();
                    Ok(PklValue::DataSize(DataSize { value, unit }))
                }
                _ => unreachable!("Cannot be reached!"),
            },
            PklToken::Duration => match lexer.slice().split('.').collect::<Vec<_>>().as_slice() {
                [value, unit] => {
                    let value: DurationValue = value.parse::<i64>()?.into();
                    let unit: DurationUnit = (*unit).into();
                    Ok(PklValue::Duration(Duration { value, unit }))
                }
                [value, frac, unit] => {
                    let value: DurationValue = format!("{}.{}", value, frac).parse::<f64>()?.into();
                    let unit: DurationUnit = (*unit).into();
                    Ok(PklValue::Duration(Duration { value, unit }))
                }
                _ => unreachable!("Cannot be reached!"),
            },
            _ => Err(ParsingError::unexpected(lexer)),
        }
    } else {
        return Err(ParsingError::eof(lexer));
    }
}
