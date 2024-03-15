use super::Statement;
use crate::{
    parser::utils::{expected, id::identifier, string::string_literal, ws},
    prelude::ParsingResult,
};
use winnow::{
    ascii::multispace0,
    combinator::{cut_err, preceded, separated},
    token::take_while,
    Parser,
};

#[derive(Debug, PartialEq, Clone)]
/// A struct representing an field of a @ModuleInfo annotation
pub struct InfoField<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

/// Parsing informationnal annotation, for instance `@ModuleInfo`
pub fn info_statement<'source>(input: &mut &'source str) -> ParsingResult<Statement<'source>> {
    // '@' already parsed

    //todo! prevent names with several dots one after another
    let name = take_while(1.., ('a'..='z', 'A'..='Z', '.')).parse_next(input)?;

    cut_err(ws('{'))
        .context(expected("open bracket"))
        .parse_next(input)?;

    let infos = separated(0.., info_field, ws(',')).parse_next(input)?;

    preceded(multispace0, '}').parse_next(input)?;
    Ok(Statement::Info { name, infos })
}

fn info_field<'source>(input: &mut &'source str) -> ParsingResult<InfoField<'source>> {
    let name = identifier.parse_next(input)?;

    cut_err(ws('='))
        .context(expected("equal sign"))
        .parse_next(input)?;

    let value = string_literal(input)?;
    //todo! need to support values not only string_literal

    Ok(InfoField { name, value })
}
