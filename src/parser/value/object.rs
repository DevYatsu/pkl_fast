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
        &parse_object_body,
    )?;

    Ok(PklValue::Object {
        value: object,
        amended_by: opt_amended_object,
    })
}

pub fn parse_object_body<'source>(
    lexer: &mut logos::Lexer<'source, PklToken>,
    token: PklToken,
) -> ParsingResult<(&'source str, PklValue<'source>)> {
    match token {
        PklToken::Identifier | PklToken::IllegalIdentifier => {
            let name: &str = lexer.slice();
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

pub fn extract_amended_object_name<'a>(raw_token: &'a str) -> &'a str {
    // we can unwrap as we are sure there is a parenthesis thanks to the initial regex match
    let end_paren_index = raw_token.find(')').unwrap();

    let result = &raw_token[1..end_paren_index];

    return result;
}
