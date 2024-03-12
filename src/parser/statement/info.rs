use super::Statement;
use crate::parser::utils::{expected, id::identifier, string::string_literal};
use winnow::{
    ascii::multispace0,
    combinator::{cut_err, delimited, preceded, separated},
    token::take_while,
    PResult, Parser,
};

#[derive(Debug, PartialEq, Clone)]
/// A struct representing an field of a @ModuleInfo annotation
pub struct InfoField<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

/// Parsing informationnal annotation, for instance `@ModuleInfo`
pub fn info_statement<'source>(input: &mut &'source str) -> PResult<Statement<'source>> {
    // '@' already parsed

    //todo! prevent names with several dots one after another
    let name = take_while(1.., ('a'..='z', 'A'..='Z', '.')).parse_next(input)?;

    multispace0.parse_next(input)?;
    cut_err('{')
        .context(expected("open bracket"))
        .parse_next(input)?;
    multispace0.parse_next(input)?;

    let infos =
        separated(0.., info_field, delimited(multispace0, ',', multispace0)).parse_next(input)?;

    preceded(multispace0, '}').parse_next(input)?;
    Ok(Statement::Info { name, infos })
}

fn info_field<'source>(input: &mut &'source str) -> PResult<InfoField<'source>> {
    let name = identifier.parse_next(input)?;
    multispace0.parse_next(input)?;

    cut_err('=')
        .context(expected("equal sign"))
        .parse_next(input)?;
    multispace0.parse_next(input)?;

    let value = string_literal(input)?;
    //todo! need to support values not only string_literal

    Ok(InfoField { name, value })
}
