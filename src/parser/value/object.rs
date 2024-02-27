use crate::{
    parser::{
        utils::{hashmap_while_not_token, retrieve_next_token},
        value::parse_value,
    },
    prelude::{ParsingError, ParsingResult, PklLexer, PklToken, PklValue},
};

/// Function called to parse an object, we assume that '{' was already found
pub fn parse_object<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<PklValue<'source>> {
    let predicate = |lexer: &mut logos::Lexer<'source, PklToken>,
                     token: PklToken|
     -> ParsingResult<(&'source str, PklValue<'source>)> {
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
                        let value = parse_object(lexer)?;

                        value
                    }
                    _ => return Err(ParsingError::unexpected(lexer)),
                };

                Ok((name, value))
            }
            _ => Err(ParsingError::unexpected(lexer)),
        }
    };

    let object =
        hashmap_while_not_token(lexer, PklToken::NewLine, PklToken::CloseBracket, &predicate)?;

    Ok(PklValue::Object(object))
}
