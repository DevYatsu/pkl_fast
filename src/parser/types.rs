use self::{errors::TypeError, union::parse_opt_union};

use super::{
    errors::ParsingError,
    expression::Expression,
    utils::{list_while_not_tokens, retrieve_next_token},
    value::{string::StringFragment, PklValue},
    ParsingResult, PklLexer,
};
use crate::{parser::expression::parse_expr, prelude::PklToken};

pub mod errors;
mod union;

#[derive(Debug, PartialEq, Clone)]
pub enum PklType<'a> {
    Any,
    NotNull,
    Unknown,
    Nothing,

    // For types that can be restrained (i.e String, Int, Float, collection-kind types),
    // I initially intended to manually add all the possible annotations,
    // but it turned out to be too troublesome.
    // It's better to parse the expression first
    // and then take care of it in the evaluation step.
    String {
        // example: s: "str" = "str", matches: Some("str")
        matches: Option<&'a str>,
        restriction: Option<Expression<'a>>,
    },
    Boolean,

    // SuperType: Number
    Int {
        restriction: Option<Expression<'a>>,
    },
    Float {
        restriction: Option<Expression<'a>>,
    },

    UInt,
    UInt8,
    UInt16,
    UInt32,
    Int8,
    Int16,
    Int32,

    Duration,
    DataSize,
    Null,

    Regex,

    Collection {
        _type: Box<PklType<'a>>,
        restriction: Option<Expression<'a>>,
    },
    Listing {
        _type: Box<PklType<'a>>,
        restriction: Option<Expression<'a>>,
    },
    List {
        _type: Box<PklType<'a>>,
        restriction: Option<Expression<'a>>,
    },

    Pair {
        key_type: Box<PklType<'a>>,
        value_type: Box<PklType<'a>>,
        restriction: Option<Expression<'a>>,
    },
    Map {
        key_type: Box<PklType<'a>>,
        value_type: Box<PklType<'a>>,
        restriction: Option<Expression<'a>>,
    },
    Mapping {
        key_type: Box<PklType<'a>>,
        value_type: Box<PklType<'a>>,
        restriction: Option<Expression<'a>>,
    },

    Set {
        _type: Box<PklType<'a>>,
        restriction: Option<Expression<'a>>,
    },

    Class {
        name: &'a str,
        generics_params: Option<Vec<PklType<'a>>>,
    },

    Union(Vec<PklType<'a>>),
    UnionDefault(Box<PklType<'a>>),
    PotentiallyNull(Box<PklType<'a>>),
}

pub fn parse_type<'source>(
    lexer: &mut PklLexer<'source>,
) -> ParsingResult<(PklType<'source>, Option<PklToken<'source>>)> {
    let base_type = parse_basic_type(lexer)?;

    parse_opt_union(lexer, base_type)
}

fn parse_basic_type<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<PklType<'source>> {
    let token = retrieve_next_token(lexer)?;

    let result_type = match token {
        PklToken::Identifier(value) => {
            let t: PklType = value.into();

            t
        }
        PklToken::GenericTypeAnnotationStart(str_type) => {
            let (types, end_token) = list_while_not_tokens(
                lexer,
                PklToken::Comma,
                &[
                    PklToken::RightAngleBracket(">"),
                    PklToken::GenericTypeAnnotationFunctionCall,
                ],
                &parse_type,
            )?;

            match end_token {
                PklToken::RightAngleBracket(_) => {
                    PklType::generate_from_generics(lexer, str_type, types)?
                }
                PklToken::GenericTypeAnnotationFunctionCall => {
                    let (expr, next_token) = parse_expr(lexer)?;

                    match next_token {
                        Some(PklToken::CloseParenthesis) => (),
                        None => return Err(ParsingError::eof(lexer)),
                        _ => return Err(ParsingError::unexpected(lexer, "')'".to_owned())),
                    }

                    let mut base_type: PklType<'_> = str_type.into();

                    // allowed types should be checked in generate_from_2_generic and generate_from_1_generic
                    match base_type {
                        PklType::Collection {
                            ref mut restriction,
                            ..
                        }
                        | PklType::Listing {
                            ref mut restriction,
                            ..
                        }
                        | PklType::List {
                            ref mut restriction,
                            ..
                        }
                        | PklType::Pair {
                            ref mut restriction,
                            ..
                        }
                        | PklType::Map {
                            ref mut restriction,
                            ..
                        }
                        | PklType::Mapping {
                            ref mut restriction,
                            ..
                        }
                        | PklType::Set {
                            ref mut restriction,
                            ..
                        } => {
                            *restriction = Some(expr);
                            base_type
                        }
                        _ => {
                            return Err(TypeError::no_restrictions_type(
                                lexer,
                                format!(
                                    "Remove the constraints annotation, try writing `{}`",
                                    base_type
                                ),
                            )
                            .into())
                        }
                    }
                }

                _ => unreachable!(),
            }
        }
        PklToken::FunctionCall(name) => {
            let mut base_type: PklType = name.into();

            let (expr, next_token) = parse_expr(lexer)?;

            match next_token {
                Some(PklToken::CloseParenthesis) => (),
                None => return Err(ParsingError::eof(lexer)),
                _ => return Err(ParsingError::unexpected(lexer, "')'".to_owned())),
            }

            // check which types can receive restriction
            match base_type {
                PklType::String {
                    ref mut restriction,
                    ..
                }
                | PklType::Int {
                    ref mut restriction,
                }
                | PklType::Float {
                    ref mut restriction,
                }
                | PklType::Collection {
                    ref mut restriction,
                    ..
                }
                | PklType::Listing {
                    ref mut restriction,
                    ..
                }
                | PklType::List {
                    ref mut restriction,
                    ..
                }
                | PklType::Pair {
                    ref mut restriction,
                    ..
                }
                | PklType::Map {
                    ref mut restriction,
                    ..
                }
                | PklType::Mapping {
                    ref mut restriction,
                    ..
                }
                | PklType::Set {
                    ref mut restriction,
                    ..
                } => {
                    *restriction = Some(expr);
                    base_type
                }

                PklType::PotentiallyNull(_) | PklType::UnionDefault(_) => unreachable!(),

                // I intentionnally separated types and did not considere them all as _
                // It will be clearer in case we need to change sth
                PklType::Duration
                | PklType::DataSize
                | PklType::Null
                | PklType::Class { .. }
                | PklType::Union(_)
                | PklType::Regex
                | _ => {
                    return Err((TypeError::no_restrictions_type(
                        lexer,
                        format!(
                            "Remove the constraints annotation, try writing `{}`",
                            base_type
                        ),
                    ))
                    .into())
                }
            }
        }
        PklToken::StringLiteral(value) => PklType::String {
            matches: Some(value),
            restriction: None,
        },
        PklToken::PotentiallyNullType(value) => PklType::PotentiallyNull(Box::new(value.into())),
        PklToken::DefaultUnionType(value) => PklType::UnionDefault(Box::new(value.into())),
        _ => {
            return Err(ParsingError::unexpected(
                lexer,
                "a valid type definition".to_owned(),
            ))
        }
    };

    Ok(result_type)
}

impl<'a> From<&'a str> for PklType<'a> {
    fn from(value: &'a str) -> Self {
        match value.trim() {
            "Any" => PklType::Any,
            "unknown" => PklType::Unknown,
            "nothing" => PklType::Nothing,
            "String" => PklType::String {
                matches: None,
                restriction: None,
            },
            "Boolean" => PklType::Boolean,
            "Int" => PklType::Int { restriction: None },
            "UInt" => PklType::UInt,
            "UInt8" => PklType::UInt8,
            "UInt16" => PklType::UInt16,
            "UInt32" => PklType::UInt32,
            "Int8" => PklType::Int8,
            "Int16" => PklType::Int16,
            "Int32" => PklType::Int32,
            "Float" => PklType::Float { restriction: None },
            "Duration" => PklType::Duration,
            "DataSize" => PklType::DataSize,
            "Null" => PklType::Null,
            "Regex" => PklType::Regex,
            "Collection" => PklType::Collection {
                _type: Box::new(PklType::Unknown),
                restriction: None,
            },
            "Listing" => PklType::Listing {
                _type: Box::new(PklType::Unknown),
                restriction: None,
            },
            "List" => PklType::List {
                _type: Box::new(PklType::Unknown),
                restriction: None,
            },
            "Pair" => PklType::Pair {
                key_type: Box::new(PklType::Unknown),
                value_type: Box::new(PklType::Unknown),
                restriction: None,
            },
            "Map" => PklType::Map {
                key_type: Box::new(PklType::Unknown),
                value_type: Box::new(PklType::Unknown),
                restriction: None,
            },
            "Mapping" => PklType::Mapping {
                key_type: Box::new(PklType::Unknown),
                value_type: Box::new(PklType::Unknown),
                restriction: None,
            },
            "Set" => PklType::Set {
                _type: Box::new(PklType::Unknown),
                restriction: None,
            },
            _ => PklType::Class {
                name: value,
                generics_params: None,
            },
        }
    }
}

impl<'a> PklType<'a> {
    pub fn default_value(&self, lexer: &mut PklLexer<'a>) -> ParsingResult<PklValue<'a>> {
        match self {
            PklType::String { matches, .. } => {
                if let Some(value) = matches {
                    return Ok(PklValue::String(StringFragment::from_raw_string(
                        lexer, value,
                    )?));
                }

                Err(ParsingError::no_default_value(lexer, &self.to_string()))
            }
            PklType::Null => Ok(PklValue::Null),
            PklType::Collection { .. } => Ok(PklValue::List(vec![])),
            PklType::Listing { .. } => Ok(PklValue::Listing(vec![])),
            PklType::List { .. } => Ok(PklValue::List(vec![])),
            PklType::Map { .. } => Ok(PklValue::Map(vec![])),
            PklType::Mapping { .. } => Ok(PklValue::Mapping(HashMap::new())),
            PklType::Set { .. } => Ok(PklValue::Set(vec![])),
            PklType::Class { name, .. } => Ok(PklValue::ClassInstance {
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

    pub fn generate_from_generics(
        lexer: &mut PklLexer<'a>,
        base_type: &'a str,
        generics: Vec<PklType<'a>>,
    ) -> Result<PklType<'a>, TypeError> {
        match base_type {
            "Collection" => {
                if generics.len() != 1 {
                    return Err(TypeError::expect_generics(lexer, 1, base_type));
                }

                Ok(PklType::Collection {
                    _type: Box::new(generics[0].to_owned()),
                    restriction: None,
                })
            }
            "Listing" => {
                if generics.len() != 1 {
                    return Err(TypeError::expect_generics(lexer, 1, base_type));
                }

                Ok(PklType::Listing {
                    _type: Box::new(generics[0].to_owned()),
                    restriction: None,
                })
            }
            "List" => {
                if generics.len() != 1 {
                    return Err(TypeError::expect_generics(lexer, 1, base_type));
                }

                Ok(PklType::List {
                    _type: Box::new(generics[0].to_owned()),
                    restriction: None,
                })
            }
            "Set" => {
                if generics.len() != 1 {
                    return Err(TypeError::expect_generics(lexer, 1, base_type));
                }

                Ok(PklType::Set {
                    _type: Box::new(generics[0].to_owned()),
                    restriction: None,
                })
            }
            "Pair" => {
                if generics.len() != 2 {
                    return Err(TypeError::expect_generics(lexer, 2, base_type));
                }

                Ok(PklType::Pair {
                    key_type: Box::new(generics[0].to_owned()),
                    value_type: Box::new(generics[1].to_owned()),
                    restriction: None,
                })
            }
            "Map" => {
                if generics.len() != 2 {
                    return Err(TypeError::expect_generics(lexer, 2, base_type));
                }

                Ok(PklType::Map {
                    key_type: Box::new(generics[0].to_owned()),
                    value_type: Box::new(generics[1].to_owned()),
                    restriction: None,
                })
            }
            "Mapping" => {
                if generics.len() != 2 {
                    return Err(TypeError::expect_generics(lexer, 2, base_type));
                }

                Ok(PklType::Mapping {
                    key_type: Box::new(generics[0].to_owned()),
                    value_type: Box::new(generics[1].to_owned()),
                    restriction: None,
                })
            }

            name => {
                let mut t: PklType = name.into();

                match t {
                    PklType::Class {
                        ref mut generics_params,
                        ..
                    } => {
                        *generics_params = Some(generics);
                        Ok(t)
                    }

                    _ => Err(TypeError::expect_generics(lexer, 0, name)),
                }
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
            PklType::String {
                matches,
                restriction,
            } => {
                if matches.is_some() {
                    write!(f, "{}", matches.unwrap(/* safe */))
                } else if restriction.is_some() {
                    write!(f, "String({})", restriction.clone().unwrap(/* safe */))
                } else {
                    write!(f, "String")
                }
            }
            PklType::Boolean => write!(f, "Boolean"),
            PklType::Int { restriction } => {
                if restriction.is_some() {
                    write!(f, "Int({})", restriction.clone().unwrap())
                } else {
                    write!(f, "Int")
                }
            }
            PklType::Float { restriction } => {
                if restriction.is_some() {
                    write!(f, "Float({})", restriction.clone().unwrap())
                } else {
                    write!(f, "Float")
                }
            }
            PklType::Duration => write!(f, "Duration"),
            PklType::DataSize => write!(f, "DataSize"),
            PklType::Null => write!(f, "Null"),
            PklType::Collection { _type, restriction } => {
                if restriction.is_some() {
                    write!(f, "Collection<{}>({})", _type, restriction.clone().unwrap())
                } else {
                    write!(f, "Collection<{}>", _type)
                }
            }
            PklType::Listing { _type, restriction } => {
                if restriction.is_some() {
                    write!(f, "Listing<{}>({})", _type, restriction.clone().unwrap())
                } else {
                    write!(f, "Listing<{}>", _type)
                }
            }
            PklType::List { _type, restriction } => {
                if restriction.is_some() {
                    write!(f, "List<{}>({})", _type, restriction.clone().unwrap())
                } else {
                    write!(f, "List<{}>", _type)
                }
            }
            PklType::Pair {
                key_type,
                value_type,
                restriction,
            } => {
                if restriction.is_some() {
                    write!(
                        f,
                        "Pair<{}, {}>({})",
                        key_type,
                        value_type,
                        restriction.clone().unwrap()
                    )
                } else {
                    write!(f, "Pair<{}, {}>", key_type, value_type)
                }
            }
            PklType::Map {
                key_type,
                value_type,
                restriction,
            } => {
                if restriction.is_some() {
                    write!(
                        f,
                        "Map<{}, {}>({})",
                        key_type,
                        value_type,
                        restriction.clone().unwrap()
                    )
                } else {
                    write!(f, "Map<{}, {}>", key_type, value_type)
                }
            }
            PklType::Mapping {
                key_type,
                value_type,
                restriction,
            } => {
                if restriction.is_some() {
                    write!(
                        f,
                        "Mapping<{}, {}>({})",
                        key_type,
                        value_type,
                        restriction.clone().unwrap()
                    )
                } else {
                    write!(f, "Mapping<{}, {}>", key_type, value_type)
                }
            }
            PklType::Set { _type, restriction } => {
                if restriction.is_some() {
                    write!(f, "Set<{}>({})", _type, restriction.clone().unwrap())
                } else {
                    write!(f, "Set<{}>", _type)
                }
            }
            PklType::Class {
                name,
                generics_params,
            } => {
                if generics_params.is_some() {
                    write!(f, "{}<", name)?;
                    for generic in generics_params.clone().unwrap() {
                        write!(f, "{},", generic)?;
                    }
                    write!(f, ">")
                } else {
                    write!(f, "{}", name)
                }
            }
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
            PklType::UInt => write!(f, "Uint"),
            PklType::UInt8 => write!(f, "Uint8"),
            PklType::UInt16 => write!(f, "Uint16"),
            PklType::UInt32 => write!(f, "Uint32"),
            PklType::Int8 => write!(f, "Int8"),
            PklType::Int16 => write!(f, "Int16"),
            PklType::Int32 => write!(f, "Int32"),
        }
    }
}
