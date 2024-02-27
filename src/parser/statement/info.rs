use crate::{
    parser::{
        operator::parse_equal,
        utils::{list_while_not_token, parse_token},
        value::parse_value,
    },
    prelude::{ParsingError, ParsingResult, PklLexer, PklToken, PklValue},
};

use super::Statement;

#[derive(Debug, PartialEq, Clone)]
/// A struct representing an field of a @ModuleInfo annotation
pub struct InfoField<'a> {
    pub name: &'a str,
    pub value: PklValue<'a>,
}

/// Parsing @ModuleInfo annotation
pub fn parse_module_info<'source>(
    lexer: &mut PklLexer<'source>,
) -> ParsingResult<Statement<'source>> {
    let infos = parse_info(lexer)?;

    Ok(Statement::ModuleInfo { infos })
}

/// Parsing @Deprecated annotation
pub fn parse_deprecated<'source>(
    lexer: &mut PklLexer<'source>,
) -> ParsingResult<Statement<'source>> {
    let infos = parse_info(lexer)?;

    Ok(Statement::DeprecatedInfo { infos })
}

fn parse_info<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<Vec<InfoField<'source>>> {
    parse_token(lexer, PklToken::OpenBracket)?;

    let predicate =
        |lexer: &mut PklLexer<'source>, token: PklToken| -> ParsingResult<InfoField<'source>> {
            match token {
                PklToken::Identifier => {
                    let name: &str = lexer.slice();
                    parse_equal(lexer)?;

                    let value = parse_value(lexer)?;

                    Ok(InfoField { name, value })
                }
                _ => Err(ParsingError::unexpected(lexer)),
            }
        };

    let infos = list_while_not_token(lexer, PklToken::Comma, PklToken::CloseBracket, &predicate)?;

    Ok(infos)
}
