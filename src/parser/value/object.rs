use crate::{
    parser::{
        utils::{hashmap_while_not_token, retrieve_next_token},
        value::parse_value,
    },
    prelude::{ParsingError, ParsingResult, PklLexer, PklToken, PklValue},
};

/// Function called to parse an object, we assume that '{' was already found
pub fn parse_object<'source>(
    lexer: &mut PklLexer<'source>,
    opt_amended_object: Option<&'source str>,
) -> ParsingResult<PklValue<'source>> {
    let object = hashmap_while_not_token(
        lexer,
        PklToken::NewLine,
        PklToken::CloseBracket,
        &parse_block,
    )?;

    Ok(PklValue::Object {
        value: object,
        amended_by: opt_amended_object,
    })
}

pub fn parse_block<'source>(
    lexer: &mut PklLexer<'source>,
    token: PklToken<'source>,
) -> ParsingResult<(&'source str, PklValue<'source>)> {
    match token {
        PklToken::Identifier(name) | PklToken::IllegalIdentifier(name) => {
            let next_token = retrieve_next_token(lexer)?;

            let value = match next_token {
                PklToken::EqualSign => {
                    let value = parse_value(lexer)?;

                    value
                }
                PklToken::OpenBracket => {
                    // we sould see whether or not we should put if the initial parent object is amended
                    let value = parse_object(lexer, None)?;

                    value
                }
                _ => return Err(ParsingError::unexpected(lexer)),
            };

            Ok((name, value))
        }
        _ => Err(ParsingError::unexpected(lexer)),
    }
}
