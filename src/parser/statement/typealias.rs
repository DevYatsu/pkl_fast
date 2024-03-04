use crate::{
    parser::{
        types::{generics::extract_generics, parse_type, PklType},
        utils::{expect_token, retrieve_next_token},
    },
    prelude::{ParsingError, ParsingResult, PklLexer, PklToken},
};

use super::Statement;
pub fn parse_typealias<'source>(
    lexer: &mut PklLexer<'source>,
) -> ParsingResult<Statement<'source>> {
    let token = retrieve_next_token(lexer)?;

    let (alias, generics_params) = match token {
        PklToken::Identifier(v) => (v, None),
        PklToken::GenericTypeAnnotation => {
            let (name, generics) = extract_generics(lexer.slice());

            let generics_vec = generics
                .into_iter()
                .map(|s| s.trim().into())
                .collect::<Vec<PklType<'source>>>();

            (name, Some(generics_vec))
        }
        _ => return Err(ParsingError::expected_identifier(lexer)),
    };

    expect_token(lexer, PklToken::EqualSign)?;

    let equivalent_type = parse_type(lexer)?;

    Ok(Statement::TypeAlias {
        alias,
        equivalent_type,
        generics_params,
    })
}
