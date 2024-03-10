use std::fmt::Display;

use winnow::{combinator::todo, PResult};

use crate::{parser::expression::Expression, prelude::PklValue};

use super::mapping::MappingField;

#[derive(Debug, PartialEq, Clone)]
pub enum ClassField<'a> {
    Expression(Expression<'a>),
    VariableDeclaration {
        name: &'a str,
        value: Expression<'a>,
    },
    MappingField(MappingField<'a>),
}

/// Function called to parse a class instance, we assume that 'new' was already found
pub fn parse_class_instance<'source>(input: &mut &'source str) -> PResult<PklValue<'source>> {
    todo(input)
    // let next_token = retrieve_next_token(parser)?;

    // let name = match next_token {
    //     PklToken::Identifier(value) => {
    //         expect_token(parser, PklToken::OpenBracket)?;
    //         match value {
    //             "Listing" => {
    //                 let values = list_while_not_token3(
    //                     parser,
    //                     &[PklToken::NewLine, PklToken::SemiColon],
    //                     PklToken::CloseBracket,
    //                     &parse_listing_field,
    //                 )?;

    //                 return Ok(PklValue::Listing(values));
    //             }
    //             "Mapping" => {
    //                 let values = list_while_not_token3(
    //                     parser,
    //                     &[PklToken::NewLine, PklToken::SemiColon],
    //                     PklToken::CloseBracket,
    //                     &parse_mapping_field,
    //                 )?;

    //                 return Ok(PklValue::Mapping(values));
    //             }
    //             _ => (),
    //         }

    //         Some(value)
    //     }
    //     PklToken::OpenBracket => None,
    //     _ => {
    //         return Err(ParsingError::unexpected(
    //             parser,
    //             "class instance".to_owned(),
    //         ))
    //     }
    // };

    // let arguments = list_while_not_token3(
    //     parser,
    //     &[PklToken::NewLine, PklToken::SemiColon],
    //     PklToken::CloseBracket,
    //     &parse_class_instance_field,
    // )?;

    // Ok(PklValue::ClassInstance { name, arguments })
}

fn parse_class_instance_field<'source>(input: &mut &'source str) -> PResult<(ClassField<'source>)> {
    todo(input)
    // match token {
    //     PklToken::Identifier(name) | PklToken::IllegalIdentifier(name) => {
    //         let next_token = retrieve_next_token(parser)?;

    //         match next_token {
    //             PklToken::EqualSign => {
    //                 let (value, next_token) = parse_expr(parser, None)?;
    //                 Ok((ClassField::VariableDeclaration { name, value }, next_token))
    //             }
    //             PklToken::OpenBracket => {
    //                 let (value, token) = parse_object(parser, None)?;

    //                 Ok((
    //                     ClassField::VariableDeclaration {
    //                         name,
    //                         value: Expression::Value(value),
    //                     },
    //                     token,
    //                 ))
    //             }
    //             _ => Err(ParsingError::unexpected(parser, "'=' or '{'".to_owned())),
    //         }
    //     }

    //     PklToken::OpenBrace => {
    //         let (field, next) = parse_mapping_variable(parser)?;

    //         Ok((ClassField::MappingField(field), next))
    //     }
    //     token => {
    //         // try parsing an expression, the instance might be a listing
    //         let (expr, next) = parse_expr(parser, Some(token))?;

    //         Ok((ClassField::Expression(expr), next))
    //     }
    // }
}

impl<'a> Display for ClassField<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClassField::Expression(e) => write!(f, "{e}"),
            ClassField::VariableDeclaration { name, value } => write!(f, "{name} = {value}"),
            ClassField::MappingField(m) => write!(f, "{m}"),
        }
    }
}
