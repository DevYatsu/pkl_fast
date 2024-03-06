use crate::{
    parser::{
        expression::{parse_expr, Expression},
        utils::{expect_token, hashmap_while_not_token1, retrieve_next_token},
    },
    prelude::{ParsingError, ParsingResult, PklLexer, PklToken, PklValue},
};

use super::parse_object;

/// Function called to parse a class instance, we assume that 'new' was already found
pub fn parse_class_instance<'source>(
    lexer: &mut PklLexer<'source>,
) -> ParsingResult<PklValue<'source>> {
    let next_token = retrieve_next_token(lexer)?;

    let name = match next_token {
        PklToken::Identifier(value) => {
            match value {
                "Listing" => unimplemented!(),
                "Mapping" => unimplemented!(),
                _ => (),
            }

            expect_token(lexer, PklToken::OpenBracket)?;
            Some(value)
        }
        PklToken::OpenBracket => None,
        _ => return Err(ParsingError::unexpected(lexer, "classname".to_owned())),
    };

    let arguments = hashmap_while_not_token1(
        lexer,
        PklToken::NewLine,
        PklToken::CloseBracket,
        &parse_class_instance_field,
    )?;

    Ok(PklValue::ClassInstance { name, arguments })
}

fn parse_class_instance_field<'source>(
    lexer: &mut PklLexer<'source>,
    token: PklToken<'source>,
) -> ParsingResult<(&'source str, Expression<'source>, Option<PklToken<'source>>)> {
    match token {
        PklToken::Identifier(name) | PklToken::IllegalIdentifier(name) => {
            let next_token = retrieve_next_token(lexer)?;

            match next_token {
                PklToken::EqualSign => {
                    let (value, next_token) = parse_expr(lexer, None)?;
                    Ok((name, value, next_token))
                }
                PklToken::OpenBracket => {
                    let value = parse_object(lexer, None)?;

                    Ok((name, Expression::Value(value), None))
                }
                _ => Err(ParsingError::unexpected(lexer, "'=' or '{'".to_owned())),
            }
        }
        _ => Err(ParsingError::expected_identifier(lexer)),
    }
}
