use crate::{
    parser::{
        utils::{expect_token, list_while_not_token0, parse_identifier, retrieve_next_token},
        value::parse_value,
    },
    prelude::{ParsingResult, PklLexer, PklToken, PklValue},
};

use super::Statement;

#[derive(Debug, PartialEq, Clone)]
/// A struct representing an field of a @ModuleInfo annotation
pub struct InfoField<'a> {
    pub name: &'a str,
    pub value: PklValue<'a>,
}

/// Parsing @ModuleInfo and @Deprecated kind annotation
pub fn parse_info<'source>(
    lexer: &mut PklLexer<'source>,
    name: &'source str,
) -> ParsingResult<Statement<'source>> {
    let infos = parse_info_content(lexer)?;

    Ok(Statement::Info { name, infos })
}

fn parse_info_content<'source>(
    lexer: &mut PklLexer<'source>,
) -> ParsingResult<Vec<InfoField<'source>>> {
    expect_token(lexer, PklToken::OpenBracket)?;

    let predicate = |lexer: &mut PklLexer<'source>| -> ParsingResult<InfoField<'source>> {
        let name = parse_identifier(lexer)?;
        expect_token(lexer, PklToken::EqualSign)?;

        let next_token = retrieve_next_token(lexer)?;
        let value = parse_value(lexer, next_token)?;

        Ok(InfoField { name, value })
    };

    let infos = list_while_not_token0(lexer, PklToken::Comma, PklToken::CloseBracket, &predicate)?;

    Ok(infos)
}
