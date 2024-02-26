use crate::lexer::LexingError;
use crate::parser::{errors::UnexpectedEndOfInputError, ParsingError, ParsingResult};
use crate::prelude::PklToken;
use miette::NamedSource;
use std::collections::HashMap;

use self::datasize::DataSize;
use self::duration::Duration;

use super::errors::locating::get_error_location;
use super::PklLexer;

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
        if let Err(_e) = token {
            // temporary mesure, LexingError may differ
            return Err(ParsingError::LexingError(LexingError::NonAsciiCharacter));
        }

        match token.unwrap() {
            PklToken::Boolean(b) => Ok(PklValue::Boolean(b)),
            PklToken::StringLiteral => {
                let raw_value = lexer.slice();
                Ok(PklValue::String(&raw_value[1..raw_value.len() - 1]))
            }
            PklToken::Integer(int) => Ok(PklValue::Int(int)),
            PklToken::Float(f) => Ok(PklValue::Float(f)),
            PklToken::Null => Ok(PklValue::Null),
            _ => todo!(),
        }
    } else {
        return Err(ParsingError::UnexpectedEndOfInput(
            UnexpectedEndOfInputError {
                src: NamedSource::new("main.pkl", lexer.source().to_string()),
                at: get_error_location(lexer).into(),
            },
        ));
    }
}
