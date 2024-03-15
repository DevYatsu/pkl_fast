use crate::prelude::ParsingResult;

use super::PklType;

pub fn parse_opt_union<'source>(input: &mut &'source str) -> ParsingResult<PklType<'source>> {
    todo!()
    // let token = if opt_token.is_some() {
    //     opt_token
    // } else {
    //     retrieve_opt_next_token(parser)?
    // };

    // let result = match token {
    //     Some(PklToken::UnionSerarator) => {
    //         let (t, next_token) = parse_type(parser, None)?;

    //         if let PklType::Union(ref mut values) = base_type {
    //             match t {
    //                 PklType::Union(second_values) => {
    //                     values.extend(second_values);
    //                 }
    //                 _ => values.push(t),
    //             };

    //             (base_type, next_token)
    //         } else {
    //             let mut values = vec![base_type];

    //             match t {
    //                 PklType::Union(second_values) => {
    //                     values.extend(second_values);
    //                 }
    //                 _ => values.push(t),
    //             };

    //             (PklType::Union(values), next_token)
    //         }
    //     }
    //     _ => (base_type, token),
    // };

    // Ok(result)
}
