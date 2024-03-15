use self::class::ClassField;
use self::datasize::{datasize_unit, DataSize};
use self::duration::{duration_unit, Duration};
use self::float::float;
use self::int::int;
use self::listing::ListingField;
use self::mapping::MappingField;
use self::object::ObjectField;
use self::string::{multiline_string_value, string_value, StringFragment};
use super::expression::Expression;
use super::utils::expected;
use super::ParsingResult;
use std::fmt;

mod amending;
mod class;
mod datasize;
mod duration;
mod float;
mod int;
mod listing;
mod mapping;
mod object;
pub mod string;
mod utils;

pub use class::class_instance;
pub use object::{object, object_values};
use winnow::combinator::{alt, cut_err, opt};
use winnow::{Parser};

#[derive(Debug, Clone, PartialEq)]
/// An enum representing any Pkl value
pub enum PklValue<'a> {
    String(Vec<StringFragment<'a>>),
    Boolean(bool),
    Int(i64),
    Float(f64),

    /// An object!
    ///  
    /// chained_body and amended_by properties: ([see note](https://pkl-lang.org/main/current/language-reference/index.html#amending-objects))
    Object {
        values: Vec<ObjectField<'a>>,
        amended_by: Option<&'a str>,
        chained_body: Option<Vec<ObjectField<'a>>>,
    },

    List(Vec<Expression<'a>>),
    Listing(Vec<ListingField<'a>>),

    Map(Vec<Expression<'a>>),

    Set(Vec<Expression<'a>>),

    /// For now, only indexing with &str is supported.
    /// In the future we shall support other any data type as key!
    Mapping(Vec<MappingField<'a>>),

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
        arguments: Vec<ClassField<'a>>,
    },
}

pub fn parse_value<'source>(input: &mut &'source str) -> ParsingResult<PklValue<'source>> {
    alt((
        "true".map(|_| PklValue::Boolean(true)),
        "false".map(|_| PklValue::Boolean(false)),
        "null".map(|_| PklValue::Null),
        class_instance,
        multiline_string_value, // need to return an error whenever the end """ is not preceded by a newline
        string_value,
        float.map(PklValue::Float),
        int.map(PklValue::Int),
    ))
    .parse_next(input)
}

/// We keep this function in case we decide to parse duration and datasize directly here
///
/// But I believe it's better to parse them with other methods and properties indexing
fn _float_or_derived<'source>(input: &mut &'source str) -> ParsingResult<PklValue<'source>> {
    let f = float.parse_next(input)?;

    let dot_exists = opt('.').parse_next(input)?.is_some();

    if dot_exists {
        let unit = opt(datasize_unit).parse_next(input)?;

        if let Some(datasize_unit) = unit {
            return Ok(PklValue::DataSize(DataSize {
                value: f.into(),
                unit: datasize_unit,
            }));
        }

        let unit = cut_err(duration_unit)
            .context(expected("datasize/duration unit"))
            .parse_next(input)?;
        Ok(PklValue::Duration(Duration {
            value: f.into(),
            unit,
        }))
    } else {
        Ok(PklValue::Float(f))
    }
}
/// We keep this function in case we decide to parse duration and datasize directly here
///
/// But I believe it's better to parse them with other methods and properties indexing
fn _int_or_derived<'source>(input: &mut &'source str) -> ParsingResult<PklValue<'source>> {
    let i = int.parse_next(input)?;

    let dot_exists = opt('.').parse_next(input)?.is_some();

    if dot_exists {
        let unit = opt(datasize_unit).parse_next(input)?;

        if let Some(datasize_unit) = unit {
            return Ok(PklValue::DataSize(DataSize {
                value: i.into(),
                unit: datasize_unit,
            }));
        }

        let unit = cut_err(duration_unit)
            .context(expected("datasize/duration unit"))
            .parse_next(input)?;
        Ok(PklValue::Duration(Duration {
            value: i.into(),
            unit,
        }))
    } else {
        Ok(PklValue::Int(i))
    }
}

// pub fn parse_value<'source>(
//     parser: &mut PklParser<'source>,
//     current_token: PklToken<'source>,
// ) -> ParsingResult<PklValue<'source>> {
//     match current_token {
//         PklToken::Boolean(b) => Ok(PklValue::Boolean(b)),
//         PklToken::StringLiteral(value) => Ok(PklValue::String(StringFragment::from_raw_string(
//             parser, value,
//         )?)),
//         PklToken::MultipleLinesString(value) => Ok(PklValue::String(
//             StringFragment::from_raw_string(parser, value)?,
//         )),
//         PklToken::Integer(i) => Ok(PklValue::Int(i)),
//         PklToken::Float(f) => Ok(PklValue::Float(f)),
//         PklToken::Null => Ok(PklValue::Null),
//         PklToken::DataSize => match parser
//             .lexer
//             .slice()
//             .split('.')
//             .collect::<Vec<_>>()
//             .as_slice()
//         {
//             [value, unit] => {
//                 let value: DataSizeValue = value.parse::<i64>()?.into();
//                 let unit: DataSizeUnit = (*unit).into();
//                 Ok(PklValue::DataSize(DataSize { value, unit }))
//             }
//             [value, frac, unit] => {
//                 let value: DataSizeValue = format!("{}.{}", value, frac).parse::<f64>()?.into();
//                 let unit: DataSizeUnit = (*unit).into();
//                 Ok(PklValue::DataSize(DataSize { value, unit }))
//             }
//             _ => unreachable!(),
//         },
//         PklToken::Duration => match parser
//             .lexer
//             .slice()
//             .split('.')
//             .collect::<Vec<_>>()
//             .as_slice()
//         {
//             [value, unit] => {
//                 let value: DurationValue = value.parse::<i64>()?.into();
//                 let unit: DurationUnit = (*unit).into();
//                 Ok(PklValue::Duration(Duration { value, unit }))
//             }
//             [value, frac, unit] => {
//                 let value: DurationValue = format!("{}.{}", value, frac).parse::<f64>()?.into();
//                 let unit: DurationUnit = (*unit).into();
//                 Ok(PklValue::Duration(Duration { value, unit }))
//             }
//             _ => unreachable!(),
//         },
//         PklToken::New => parse_class_instance(parser),
//         _ => Err(ParsingError::expected_expression(parser)),
//     }
// }

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
            PklValue::Object {
                values,
                amended_by,
                chained_body,
            } => {
                if let Some(amended_by) = amended_by {
                    write!(f, "({}) ", amended_by)?;
                }
                write!(f, "{{")?;
                for value in values {
                    write!(f, "{}", value)?;
                }

                match chained_body {
                    Some(amended_values) => {
                        write!(f, "{{")?;
                        for value in amended_values {
                            write!(f, "{}", value)?;
                        }
                        write!(f, "}}")
                    }
                    None => write!(f, "}}"),
                }
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
                for x in map {
                    write!(f, "\t{x}\n",)?;
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
                    for value in arguments {
                        write!(f, "\t{}\n", value)?;
                    }
                }
                write!(f, "}}")?;
                Ok(())
            }
            PklValue::Set(values) => {
                write!(f, "Set(")?;
                for val in values {
                    write!(f, "{}, ", val)?;
                }
                write!(f, ")")
            }
            PklValue::Nullable(value) => {
                write!(f, "Null({})", *value)
            }
        }
    }
}

impl<'a> Into<PklValue<'a>> for (f64, &'a str) {
    fn into(self) -> PklValue<'a> {
        todo!()
    }
}

impl<'a> Into<PklValue<'a>> for (i64, &'a str) {
    fn into(self) -> PklValue<'a> {
        todo!()
    }
}
