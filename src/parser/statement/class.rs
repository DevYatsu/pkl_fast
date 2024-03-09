use winnow::{combinator::todo, PResult};

use crate::{
    parser::types::PklType,
    prelude::{PklToken, PklValue},
};

#[derive(Debug, PartialEq, Clone)]
/// A struct representing the type of a `ClassDeclaration`.
pub enum ClassType {
    Abstract,
    Open,
    None,
}

#[derive(Debug, PartialEq, Clone)]
/// A struct representing the type of a `ClassField`.
pub enum FieldType {
    Fixed,
    Hidden,
    Local,
    None,
}

#[derive(Debug, PartialEq, Clone)]
/// A struct representing an argument, that is a field or a method, of a `ClassDeclaration`.
pub enum ClassArgument<'a> {
    Field {
        value: PklType<'a>,
        _type: FieldType,
    },

    Method {
        args: Vec<(&'a str, PklType<'a>)>,
        return_type: PklType<'a>,
        return_value: PklValue<'a>,
    },
}

pub fn parse_class_field<'source>(
    input: &mut &'source str,
) -> PResult<(
    (&'source str, ClassArgument<'source>),
    Option<PklToken<'source>>,
)> {
    todo(input)
    // match token {
    //     PklToken::Hidden => {
    //         let name = parse_identifier(parser)?;
    //         expect_token(parser, PklToken::Colon)?;
    //         let (value, next_token) = parse_type(parser, None)?;

    //         Ok((
    //             (
    //                 name,
    //                 ClassArgument::Field {
    //                     value,
    //                     _type: FieldType::Hidden,
    //                 },
    //             ),
    //             next_token,
    //         ))
    //     }
    //     PklToken::Fixed => {
    //         let name = parse_identifier(parser)?;
    //         expect_token(parser, PklToken::Colon)?;
    //         let (value, next_token) = parse_type(parser, None)?;

    //         Ok((
    //             (
    //                 name,
    //                 ClassArgument::Field {
    //                     value,
    //                     _type: FieldType::Hidden,
    //                 },
    //             ),
    //             next_token,
    //         ))
    //     }
    //     PklToken::Local => {
    //         let name = parse_identifier(parser)?;
    //         expect_token(parser, PklToken::Colon)?;
    //         let (value, next_token) = parse_type(parser, None)?;

    //         Ok((
    //             (
    //                 name,
    //                 ClassArgument::Field {
    //                     value,
    //                     _type: FieldType::Local,
    //                 },
    //             ),
    //             next_token,
    //         ))
    //     }
    //     PklToken::Identifier(name) | PklToken::IllegalIdentifier(name) => {
    //         expect_token(parser, PklToken::Colon)?;
    //         let (value, next_token) = parse_type(parser, None)?;

    //         Ok((
    //             (
    //                 name,
    //                 ClassArgument::Field {
    //                     value,
    //                     _type: FieldType::None,
    //                 },
    //             ),
    //             next_token,
    //         ))
    //     }
    //     PklToken::Function => {
    //         let name = match retrieve_next_token(parser)? {
    //             PklToken::FunctionCall(name) => name,
    //             _ => return Err(ParsingError::unexpected(parser, "function name".to_owned())),
    //         };

    //         let args = list_while_not_token2(
    //             parser,
    //             PklToken::Comma,
    //             PklToken::CloseParenthesis,
    //             &parse_fn_arg,
    //         )?;
    //         expect_token(parser, PklToken::Colon)?;

    //         let (return_type, next_token) = parse_type(parser, None)?;

    //         assert_token_eq(parser, next_token, PklToken::EqualSign)?;

    //         let next_token = retrieve_next_token(parser)?;
    //         let return_value = parse_value(parser, next_token)?;

    //         Ok((
    //             (
    //                 name,
    //                 ClassArgument::Method {
    //                     args,
    //                     return_type,
    //                     return_value,
    //                 },
    //             ),
    //             None,
    //         ))
    //     }
    //     _ => Err(ParsingError::unexpected(
    //         parser,
    //         "field or method definition".to_string(),
    //     )),
    // }
}

fn parse_fn_arg<'a>(
    input: &mut &'a str,
) -> PResult<((&'a str, PklType<'a>), Option<PklToken<'a>>)> {
    todo(input)
    // let name = if opt_token.is_some() {
    //     match opt_token.unwrap() {
    //         PklToken::Identifier(id) => id,
    //         _ => return Err(ParsingError::expected_identifier(parser)),
    //     }
    // } else {
    //     parse_identifier(parser)?
    // };
    // expect_token(parser, PklToken::Colon)?;
    // let (value, next_token) = parse_type(parser, None)?;

    // Ok(((name, value), next_token))
}
