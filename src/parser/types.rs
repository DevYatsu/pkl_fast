use self::{
    errors::{Expected1GenericError, Expected2GenericError, TypeError},
    generics::extract_generics,
};

use super::{
    errors::{
        locating::{generate_source, get_error_location},
        ParsingError,
    },
    expression::Expression,
    utils::retrieve_next_token,
    value::PklValue,
    ParsingResult, PklLexer,
};
use crate::{parser::expression::parse_expr, prelude::PklToken};

pub mod errors;
mod generics;

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub enum PklType<'a> {
    Any,
    NotNull,
    Unknown,
    Nothing,

    String {
        matches: Option<&'a str>,
        contains: Option<&'a str>,
        allowed_empty: bool,
    },
    Boolean,

    Int {
        between: Option<(i64, i64)>,
    },
    Float,
    Number,

    Duration,
    DataSize,
    Null,

    Regex,

    Collection(Box<PklType<'a>>),
    Listing(Box<PklType<'a>>),
    List(Box<PklType<'a>>),

    Pair(Box<PklType<'a>>, Box<PklType<'a>>),
    Map(Box<PklType<'a>>, Box<PklType<'a>>),
    Mapping(Box<PklType<'a>>, Box<PklType<'a>>),

    Set(Box<PklType<'a>>),

    Class(&'a str),

    Union(Vec<PklType<'a>>),
    UnionDefault(Box<PklType<'a>>),
    PotentiallyNull(Box<PklType<'a>>),
}

pub fn parse_type<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<PklType<'source>> {
    let token = retrieve_next_token(lexer)?;

    match token {
        PklToken::Identifier(value) => Ok(value.into()),
        PklToken::GenericTypeAnnotation => {
            let raw_value: &str = lexer.slice();

            let (base_type, mut generics) = extract_generics(raw_value);

            // there is necessarily one generic otherwise the lexer would have produced an Error
            // we do not need to call trim method on our strings as it's done in the impl From<&str>
            let first_generic: PklType<'_> = generics.next().unwrap().into();
            let second_generic = generics.next().map(|s| s.into());

            if second_generic.is_some() {
                Ok(PklType::generate_from_2_generic(
                    lexer,
                    base_type,
                    first_generic,
                    second_generic.unwrap(),
                )?)
            } else {
                Ok(PklType::generate_from_1_generic(
                    lexer,
                    base_type,
                    first_generic,
                )?)
            }
        }

        PklToken::FunctionCall(name) => {
            let base_type: PklType = name.into();

            let value = parse_expr(lexer)?;

            todo!()
        }
        PklToken::StringLiteral(value) => Ok(PklType::String {
            matches: Some(value),
            contains: None,
            allowed_empty: true,
        }),
        PklToken::PotentiallyNullType(value) => {
            Ok(PklType::PotentiallyNull(Box::new(value.into())))
        }
        _ => Err(ParsingError::unexpected(lexer)),
    }
}

impl<'a> From<&'a str> for PklType<'a> {
    fn from(value: &'a str) -> Self {
        match value.trim() {
            "Any" => PklType::Any,
            "unknown" => PklType::Unknown,
            "nothing" => PklType::Nothing,
            "String" => PklType::String {
                matches: None,
                contains: None,
                allowed_empty: true,
            },
            "Boolean" => PklType::Boolean,
            "Int" => PklType::Int { between: None },
            "UInt" => PklType::Int {
                between: Some((0, i64::MAX)),
            },
            "UInt8" => PklType::Int {
                between: Some((0, 255)),
            },
            "UInt16" => PklType::Int {
                between: Some((0, 65_535)),
            },
            "UInt32" => PklType::Int {
                between: Some((0, 4_294_967_295)),
            },
            "Int8" => PklType::Int {
                between: Some((-128, 127)),
            },
            "Int16" => PklType::Int {
                between: Some((-32_768, 32_767)),
            },
            "Int32" => PklType::Int {
                between: Some((-2_147_483_648, 2_147_483_647)),
            },
            "Float" => PklType::Float,
            "Number" => PklType::Number,
            "Duration" => PklType::Duration,
            "DataSize" => PklType::DataSize,
            "Null" => PklType::Null,
            "Collection" => PklType::Collection(Box::new(PklType::Unknown)),
            "Listing" => PklType::Listing(Box::new(PklType::Unknown)),
            "List" => PklType::List(Box::new(PklType::Unknown)),
            "Pair" => PklType::Pair(Box::new(PklType::Unknown), Box::new(PklType::Unknown)),
            "Map" => PklType::Map(Box::new(PklType::Unknown), Box::new(PklType::Unknown)),
            "Mapping" => PklType::Mapping(Box::new(PklType::Unknown), Box::new(PklType::Unknown)),
            "Set" => PklType::Set(Box::new(PklType::Unknown)),
            _ => PklType::Class(value),
        }
    }
}

impl<'a> PklType<'a> {
    pub fn default_value(&self, lexer: &mut PklLexer<'a>) -> ParsingResult<PklValue<'a>> {
        match self {
            PklType::String { matches, .. } => {
                if let Some(value) = matches {
                    return Ok(PklValue::String(*value));
                }

                Err(ParsingError::no_default_value(lexer, &self.to_string()))
            }
            PklType::Null => Ok(PklValue::Null),
            PklType::Collection(_) => Ok(PklValue::List(vec![])),
            PklType::Listing(_) => Ok(PklValue::List(vec![])),
            PklType::List(_) => Ok(PklValue::List(vec![])),
            PklType::Map(_, _) => Ok(PklValue::Map(vec![])),
            PklType::Mapping(_, _) => todo!(),
            PklType::Set(_) => Ok(PklValue::Set(vec![])),
            PklType::Class(name) => Ok(PklValue::ClassInstance {
                name: Some(*name),
                arguments: HashMap::new(),
            }),
            PklType::PotentiallyNull(t) => Ok(PklValue::Nullable(Box::new(Expression::Value(
                t.default_value(lexer)?,
            )))),
            PklType::Union(values) => {
                let result = values
                    .iter()
                    .filter(|value| match value {
                        PklType::UnionDefault(_) => true,
                        _ => false,
                    })
                    .collect::<Vec<_>>();

                if result.len() != 1 {
                    return Err(ParsingError::no_default_value(lexer, &self.to_string()));
                }

                result[0].default_value(lexer)
            }
            _ => Err(ParsingError::no_default_value(lexer, &self.to_string())),
        }
    }

    pub fn generate_from_1_generic(
        lexer: &mut PklLexer<'a>,
        base_type: &'a str,
        first_type: PklType<'a>,
    ) -> Result<PklType<'a>, TypeError> {
        match base_type {
            "Collection" => Ok(PklType::Collection(Box::new(first_type))),
            "Listing" => Ok(PklType::Listing(Box::new(first_type))),
            "List" => Ok(PklType::List(Box::new(first_type))),
            "Set" => Ok(PklType::Set(Box::new(first_type))),
            _ => {
                return Err(TypeError::Expected1Generic(Expected1GenericError {
                    src: generate_source("main.pkl", lexer.source()),
                    at: get_error_location(lexer).into(),
                }))
            }
        }
    }
    pub fn generate_from_2_generic(
        lexer: &mut PklLexer<'a>,
        base_type: &'a str,
        first_type: PklType<'a>,
        second_type: PklType<'a>,
    ) -> Result<PklType<'a>, TypeError> {
        match base_type {
            "Pair" => Ok(PklType::Pair(Box::new(first_type), Box::new(second_type))),
            "Map" => Ok(PklType::Map(Box::new(first_type), Box::new(second_type))),
            "Mapping" => Ok(PklType::Mapping(
                Box::new(first_type),
                Box::new(second_type),
            )),
            _ => {
                return Err(TypeError::Expected2Generic(Expected2GenericError {
                    src: generate_source("main.pkl", lexer.source()),
                    at: get_error_location(lexer).into(),
                }))
            }
        }
    }
}

use std::{collections::HashMap, fmt};

impl<'a> fmt::Display for PklType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PklType::Any => write!(f, "Any"),
            PklType::Unknown => write!(f, "unknown"),
            PklType::Nothing => write!(f, "nothing"),
            PklType::String { .. } => {
                write!(f, "String")
            }
            PklType::Boolean => write!(f, "Boolean"),
            PklType::Int { .. } => write!(f, "Int"),
            PklType::Float => write!(f, "Float"),
            PklType::Number => write!(f, "Number"),
            PklType::Duration => write!(f, "Duration"),
            PklType::DataSize => write!(f, "DataSize"),
            PklType::Null => write!(f, "Null"),
            PklType::Collection(x) => write!(f, "Collection<{}>", x),
            PklType::Listing(x) => write!(f, "Listing<{}>", x),
            PklType::List(x) => write!(f, "List<{}>", x),
            PklType::Pair(x, y) => write!(f, "Pair<{}, {}>", x, y),
            PklType::Map(x, y) => write!(f, "Map<{}, {}>", x, y),
            PklType::Mapping(x, y) => write!(f, "Mapping<{}, {}>", x, y),
            PklType::Set(x) => write!(f, "Set<{}>", x),
            PklType::Class(name) => write!(f, "{name}"),
            PklType::Union(values) => {
                write!(f, "{}", values[0])?;
                for value in &values[1..] {
                    write!(f, "|{}", value)?;
                }
                Ok(())
            }
            PklType::PotentiallyNull(t) => write!(f, "{t}?"),
            PklType::NotNull => write!(f, "NotNull"),
            PklType::Regex => write!(f, "Regex"),
            PklType::UnionDefault(t) => write!(f, "*{t}"),
        }
    }
}
