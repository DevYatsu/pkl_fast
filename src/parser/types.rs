use winnow::{
    ascii::multispace0,
    combinator::{alt, delimited, opt, preceded, separated},
    Parser,
};

use super::{
    expression::Expression,
    utils::{id::identifier, string::string_literal},
    value::{string::StringFragment, PklValue},
    ParsingResult,
};
use std::fmt;

pub mod errors;
mod union;

#[derive(Debug, PartialEq, Clone)]
/// Represents a PklType.
pub struct PklType<'a> {
    pub name: &'a str,
    pub args: Option<Vec<PklType<'a>>>,
    pub restriction: Option<Expression<'a>>,
}

pub fn parse_type<'source>(input: &mut &'source str) -> ParsingResult<PklType<'source>> {
    alt((
        (identifier, opt(generics)).map(|(name, generics)| PklType::new(name, generics, None)),
        (string_literal, opt(generics)).map(|(s, generics)| {
            PklType::new(
                "String",
                generics,
                Some(Expression::Value(PklValue::String(vec![
                    StringFragment::RawText(s),
                ]))),
            )
        }),
    ))
    .parse_next(input)
}

fn generics<'source>(input: &mut &'source str) -> ParsingResult<Vec<PklType<'source>>> {
    delimited(
        '<',
        separated(1.., parse_type, delimited(multispace0, ',', multispace0)),
        preceded(multispace0, '>'),
    )
    .parse_next(input)
}

impl<'a> PklType<'a> {
    pub fn new(
        name: &'a str,
        args: Option<Vec<PklType<'a>>>,
        restriction: Option<Expression<'a>>,
    ) -> Self {
        Self {
            name,
            args,
            restriction,
        }
    }
}

impl<'a> fmt::Display for PklType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
