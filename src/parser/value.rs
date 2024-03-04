use self::datasize::{DataSize, DataSizeValue};
use self::duration::{Duration, DurationUnit, DurationValue};
use self::string::StringFragment;
use super::expression::Expression;
use super::PklLexer;
use crate::parser::value::datasize::DataSizeUnit;
use crate::parser::{errors::ParsingError, ParsingResult};
use crate::prelude::PklToken;
use std::collections::HashMap;
use std::fmt;

mod class;
mod datasize;
mod duration;
pub mod object;
pub mod string;

pub use class::parse_class_instance;
pub use object::parse_object;

#[derive(Debug, Clone, PartialEq)]
/// An enum representing any Pkl value
pub enum PklValue<'a> {
    String(Vec<StringFragment<'a>>),
    Boolean(bool),
    Int(i64),
    Float(f64),
    Object {
        value: HashMap<&'a str, Expression<'a>>,
        amended_by: Option<&'a str>,
    },

    List(Vec<Expression<'a>>),
    Listing(Vec<Expression<'a>>),

    Map(Vec<Expression<'a>>),

    Set(Vec<Expression<'a>>),

    /// For now, only indexing with &str is supported.
    /// In the future we shall support other any data type as key!
    Mapping(HashMap<&'a str, Expression<'a>>),

    Duration(Duration),
    DataSize(DataSize),
    Null,

    Nullable(Box<Expression<'a>>),

    /// You may ask why `name` is optional ?
    /// pkl allows to specify the class in only the type and not in the instantiation
    /// Thus when parsing a value we cannot know the name if the type is specified and not the name after the `new`
    /// For instance, we can write:
    /// parrot: Bird = new {
    ///     name = "James"
    /// }
    /// or:
    /// parrot = new Bird {
    ///     name = "James"
    /// }
    ClassInstance {
        name: Option<&'a str>,
        arguments: HashMap<&'a str, Expression<'a>>,
    },
}

pub fn parse_value<'source>(
    lexer: &mut PklLexer<'source>,
    current_token: PklToken<'source>,
) -> ParsingResult<PklValue<'source>> {
    match current_token {
        PklToken::Boolean(b) => Ok(PklValue::Boolean(b)),
        PklToken::StringLiteral(value) => Ok(PklValue::String(StringFragment::from_raw_string(
            lexer, value,
        )?)),
        PklToken::MultipleLinesString(value) => Ok(PklValue::String(
            StringFragment::from_raw_string(lexer, value)?,
        )),
        PklToken::Integer(i) => Ok(PklValue::Int(i)),
        PklToken::Float(f) => Ok(PklValue::Float(f)),
        PklToken::Null => Ok(PklValue::Null),
        PklToken::AmendedObjectBracket(amended_by) => {
            let value = parse_object(lexer, Some(amended_by))?;

            Ok(value)
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
        PklToken::New => parse_class_instance(lexer),
        _ => Err(ParsingError::expected_expression(lexer)),
    }
}

impl<'a> fmt::Display for PklValue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PklValue::String(fragments) => {
                for frag in fragments {
                    write!(f, "{}", frag)?;
                }

                Ok(())
            }
            PklValue::Boolean(b) => write!(f, "{}", b),
            PklValue::Int(i) => write!(f, "{}", i),
            PklValue::Float(fl) => write!(f, "{}", fl),
            PklValue::Object { value, amended_by } => {
                if let Some(amended_by) = amended_by {
                    write!(f, "({}) ", amended_by)?;
                }
                write!(f, "{{")?;
                for (key, val) in value {
                    write!(f, "{}: {}, ", key, val)?;
                }

                write!(f, "}}")
            }
            PklValue::List(list) => {
                write!(f, "List(")?;
                for val in list {
                    write!(f, "{}, ", val)?;
                }
                write!(f, ")")
            }
            PklValue::Listing(list) => {
                write!(f, "new Listing {{\n")?;
                for val in list {
                    write!(f, "\t{}\n", val)?;
                }
                write!(f, "}}")
            }
            PklValue::Map(vec) => {
                write!(f, "Map(")?;
                for val in vec {
                    write!(f, "{}, ", val)?;
                }
                write!(f, ")")
            }
            PklValue::Mapping(map) => {
                write!(f, "new Mappin {{\n")?;
                for (key, val) in map {
                    write!(f, "\t{}: {}\n", key, val)?;
                }
                write!(f, "}}")
            }
            PklValue::Duration(duration) => write!(f, "{}", duration),
            PklValue::DataSize(data_size) => write!(f, "{}", data_size),
            PklValue::Null => write!(f, "null"),
            PklValue::ClassInstance { name, arguments } => {
                write!(f, "new {} {{", name.unwrap_or(""))?;
                if !arguments.is_empty() {
                    write!(f, "\n")?;
                    for (key, val) in arguments {
                        write!(f, "\t{}: {}\n", key, val)?;
                    }
                }
                write!(f, "}}")?;
                Ok(())
            }
            PklValue::Set(_) => todo!(),
            PklValue::Nullable(value) => {
                write!(f, "Null({})", *value)
            }
        }
    }
}
