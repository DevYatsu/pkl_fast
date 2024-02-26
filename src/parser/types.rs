use crate::prelude::PklToken;

use super::{ParsingError, ParsingResult, PklLexer};

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
    if let Some(token) = lexer.next() {
        if let Err(e) = token {
            return Err(ParsingError::lexing(lexer, e));
        }

        match token.unwrap() {
            PklToken::Identifier => {
                let value: &str = lexer.slice();
                Ok(value.into())
            }
            PklToken::GenericTypeAnnotation => {
                todo!()
            }
            _ => Err(ParsingError::unexpected(lexer)),
        }
    } else {
        return Err(ParsingError::eof(lexer));
    }
}

impl<'a> From<&'a str> for PklType<'a> {
    fn from(value: &'a str) -> Self {
        match value {
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
            "Collection" => PklType::Collection(Box::new(PklType::Unknown)), // For now we put unknown everywhere
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
