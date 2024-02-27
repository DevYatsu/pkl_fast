use crate::{
    parser::utils::{hashmap_while_not_token, retrieve_next_token},
    prelude::{ParsingError, ParsingResult, PklLexer, PklToken, PklValue},
};

use super::object::parse_object_body;

/// Function called to parse a class instance, we assume that 'new' was already found
pub fn parse_class_instance<'source>(
    lexer: &mut PklLexer<'source>,
) -> ParsingResult<PklValue<'source>> {
    let next_token = retrieve_next_token(lexer)?;

    let name = match next_token {
        PklToken::Identifier => Some(lexer.slice()),
        PklToken::OpenBracket => None,
        _ => return Err(ParsingError::unexpected(lexer)),
    };

    let arguments = hashmap_while_not_token(
        lexer,
        PklToken::NewLine,
        PklToken::CloseBracket,
        &parse_object_body,
    )?;

    Ok(PklValue::ClassInstance { name, arguments })
}
