use super::Statement;
use crate::prelude::PklValue;
use winnow::{combinator::todo, PResult};

#[derive(Debug, PartialEq, Clone)]
/// A struct representing an field of a @ModuleInfo annotation
pub struct InfoField<'a> {
    pub name: &'a str,
    pub value: PklValue<'a>,
}

/// Parsing @ModuleInfo annotation
pub fn parse_module_info<'source>(input: &mut &'source str) -> PResult<Statement<'source>> {
    todo(input)
    // let infos = parse_info(parser)?;

    // Ok(Statement::ModuleInfo { infos })
}

/// Parsing @Deprecated annotation
pub fn parse_deprecated<'source>(input: &mut &'source str) -> PResult<Statement<'source>> {
    todo(input)
    // let infos = parse_info(parser)?;

    // Ok(Statement::DeprecatedInfo { infos })
}

fn parse_info<'source>(input: &mut &'source str) -> PResult<Vec<InfoField<'source>>> {
    todo(input)
    // expect_token(parser, PklToken::OpenBracket)?;

    // let predicate = |parser: &mut PklParser<'source>| -> ParsingResult<InfoField<'source>> {
    //     let name = parse_identifier(parser)?;
    //     expect_token(parser, PklToken::EqualSign)?;

    //     let next_token = retrieve_next_token(parser)?;
    //     let value = parse_value(parser, next_token)?;

    //     Ok(InfoField { name, value })
    // };

    // let infos = list_while_not_token0(parser, PklToken::Comma, PklToken::CloseBracket, &predicate)?;

    // Ok(infos)
}
