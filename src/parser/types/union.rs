use crate::{
    parser::utils::retrieve_opt_next_token,
    prelude::{ParsingResult, PklLexer, PklToken},
};

use super::{parse_type, PklType};

pub fn parse_opt_union<'source>(
    lexer: &mut PklLexer<'source>,
    mut base_type: PklType<'source>,
) -> ParsingResult<(PklType<'source>, Option<PklToken<'source>>)> {
    let token = retrieve_opt_next_token(lexer)?;

    let result = match token {
        Some(PklToken::UnionSerarator) => {
            let (t, next_token) = parse_type(lexer, None)?;

            if let PklType::Union(ref mut values) = base_type {
                match t {
                    PklType::Union(second_values) => {
                        values.extend(second_values);
                    }
                    _ => values.push(t),
                };

                (base_type, next_token)
            } else {
                let mut values = vec![base_type];

                match t {
                    PklType::Union(second_values) => {
                        values.extend(second_values);
                    }
                    _ => values.push(t),
                };

                (PklType::Union(values), next_token)
            }
        }
        _ => (base_type, token),
    };

    Ok(result)
}
