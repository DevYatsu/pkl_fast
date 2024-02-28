use self::{
    errors::{Expected1GenericError, Expected2GenericError, TypeError},
    generics::extract_generics,
};

use super::{
    errors::{
        locating::{generate_source, get_error_location},
        ParsingError,
    },
    utils::retrieve_next_token,
    ParsingResult, PklLexer,
};
use crate::prelude::PklToken;

pub mod errors;
mod generics;

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub enum PklType<'a> {
    Any,
    Unknown,
    Nothing,

    String,
    Boolean,

    Int,
    UInt16,
    Float,
    Number,

    Duration,
    DataSize,
    Null,

    Collection(Box<PklType<'a>>),
    Listing(Box<PklType<'a>>),
    List(Box<PklType<'a>>),

    Pair(Box<PklType<'a>>, Box<PklType<'a>>),
    Map(Box<PklType<'a>>, Box<PklType<'a>>),
    Mapping(Box<PklType<'a>>, Box<PklType<'a>>),

    Set(Box<PklType<'a>>),

    Class(&'a str),
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
        _ => Err(ParsingError::unexpected(lexer)),
    }
}

impl<'a> From<&'a str> for PklType<'a> {
    fn from(value: &'a str) -> Self {
        match value.trim() {
            "Any" => PklType::Any,
            "unknown" => PklType::Unknown,
            "nothing" => PklType::Nothing,
            "String" => PklType::String,
            "Boolean" => PklType::Boolean,
            "Int" => PklType::Int,
            "UInt16" => PklType::UInt16,
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
