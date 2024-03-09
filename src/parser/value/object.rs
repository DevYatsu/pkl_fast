use std::fmt;

use winnow::{combinator::todo, PResult};

use crate::{
    parser::expression::{parse_expr, Expression},
    prelude::{ParsingError, ParsingResult, PklParser, PklToken, PklValue},
};

#[derive(Debug, PartialEq, Clone)]
pub enum ObjectField<'a> {
    Variable {
        name: &'a str,
        value: Expression<'a>,
    },

    WhenGenerator {
        condition: Expression<'a>,
        members: Vec<ObjectField<'a>>,
        _else: Option<Vec<ObjectField<'a>>>,
    },

    ForGenerator {
        key: Option<&'a str>,
        value: &'a str,
        iterable: Expression<'a>,
        members: Vec<ObjectField<'a>>,
    },

    Spread(&'a str),
    NullableSpread(&'a str),

    // AmendedValue for either Mapping or Listing
    AmendedValue {
        key: Expression<'a>,
        value: Expression<'a>,
    },
    DefaultObject(Vec<ObjectField<'a>>),

    MemberPredicate {
        match_expr: Expression<'a>,
        values: Vec<ObjectField<'a>>,
    },

    // represents an expression when a Listing is amended
    Expression(Expression<'a>),
}

/// Function called to parse an object, we assume that '{' was already found
pub fn parse_object<'source>(
    input: &mut &'source str,
) -> PResult<(PklValue<'source>, Option<PklToken<'source>>)> {
    todo(input)
    // let values = parse_object_values(parser)?;

    // match retrieve_opt_next_token(parser)? {
    //     Some(PklToken::OpenBracket) => {
    //         let chained_body = list_while_not_token3(
    //             parser,
    //             &[PklToken::NewLine, PklToken::SemiColon],
    //             PklToken::CloseBracket,
    //             &parse_block_field,
    //         )?;

    //         Ok((
    //             PklValue::Object {
    //                 values,
    //                 amended_by,
    //                 chained_body: Some(chained_body),
    //             },
    //             None,
    //         ))
    //     }

    //     token => Ok((
    //         PklValue::Object {
    //             values,
    //             amended_by,
    //             chained_body: None,
    //         },
    //         token,
    //     )),
    // }
}

pub fn parse_object_values<'source>(
    input: &mut &'source str,
) -> PResult<Vec<ObjectField<'source>>> {
    todo(input)
    // list_while_not_token3(
    //     parser,
    //     &[PklToken::NewLine, PklToken::SemiColon],
    //     PklToken::CloseBracket,
    //     &parse_block_field,
    // )
}

pub fn parse_block_field<'source>(
    input: &mut &'source str,
) -> PResult<(ObjectField<'source>, Option<PklToken<'source>>)> {
    todo(input)

    // match token {
    //     PklToken::Identifier(name) | PklToken::IllegalIdentifier(name) => {
    //         let next_token = retrieve_next_token(parser)?;

    //         let (value, next_token) = match next_token {
    //             PklToken::EqualSign => {
    //                 let (value, next_token) = parse_expr(parser, None)?;
    //                 (value, next_token)
    //             }
    //             PklToken::OpenBracket => {
    //                 // we sould see whether or not we add to this object that its parent object is amended
    //                 let (value, token) = parse_object(parser, None)?;

    //                 (Expression::Value(value), token)
    //             }
    //             _ => return Err(ParsingError::unexpected(parser, "'='".to_owned())),
    //         };

    //         Ok((ObjectField::Variable { name, value }, next_token))
    //     }
    //     PklToken::OpenBrace => {
    //         let token = retrieve_next_token(parser)?;

    //         match token {
    //             PklToken::OpenBrace => {
    //                 let (match_expr, next_token) = parse_expr(parser, None)?;
    //                 assert_token_eq(parser, next_token, PklToken::CloseBrace)?;
    //                 expect_token(parser, PklToken::CloseBrace)?;
    //                 expect_token(parser, PklToken::OpenBracket)?;

    //                 let values = parse_object_values(parser)?;
    //                 return Ok((ObjectField::MemberPredicate { match_expr, values }, None));
    //             }
    //             _ => (),
    //         };

    //         let (expr, next_token) = parse_expr(parser, Some(token))?;
    //         assert_token_eq(parser, next_token, PklToken::CloseBrace)?;
    //         match retrieve_next_token(parser)? {
    //             PklToken::OpenBracket => {
    //                 let (value, token) = parse_object(parser, None)?;
    //                 Ok((
    //                     ObjectField::AmendedValue {
    //                         value: Expression::Value(value),
    //                         key: expr,
    //                     },
    //                     token,
    //                 ))
    //             }
    //             PklToken::EqualSign => {
    //                 let (value, next) = parse_expr(parser, None)?;
    //                 Ok((ObjectField::AmendedValue { value, key: expr }, next))
    //             }
    //             _ => Err(ParsingError::unexpected(parser, "'=' or '{'".to_owned())),
    //         }
    //     }
    //     PklToken::Default => {
    //         expect_token(parser, PklToken::OpenBracket)?;
    //         let values = parse_object_values(parser)?;

    //         Ok((ObjectField::DefaultObject(values), None))
    //     }
    //     PklToken::SpreadSyntax => {
    //         let ident = parse_identifier(parser)?;
    //         Ok((ObjectField::Spread(ident), None))
    //     }
    //     PklToken::NullableSpreadSyntax => {
    //         let ident = parse_identifier(parser)?;
    //         Ok((ObjectField::NullableSpread(ident), None))
    //     }
    //     PklToken::When => {
    //         expect_token(parser, PklToken::OpenParenthesis)?;
    //         let (condition, next) = parse_opt_newlines(parser, &parse_expr)?;
    //         assert_token_eq(parser, next, PklToken::CloseParenthesis)?;
    //         expect_token(parser, PklToken::OpenBracket)?;

    //         let members = list_while_not_token3(
    //             parser,
    //             &[PklToken::NewLine],
    //             PklToken::CloseBracket,
    //             &parse_block_field,
    //         )?;

    //         let next_token = retrieve_next_token(parser)?;

    //         match next_token {
    //             PklToken::Else => {
    //                 expect_token(parser, PklToken::OpenBracket)?;
    //                 let _else = list_while_not_token3(
    //                     parser,
    //                     &[PklToken::NewLine],
    //                     PklToken::CloseBracket,
    //                     &parse_block_field,
    //                 )?;

    //                 Ok((
    //                     ObjectField::WhenGenerator {
    //                         condition,
    //                         members,
    //                         _else: Some(_else),
    //                     },
    //                     None,
    //                 ))
    //             }
    //             _ => Ok((
    //                 ObjectField::WhenGenerator {
    //                     condition,
    //                     members,
    //                     _else: None,
    //                 },
    //                 Some(next_token),
    //             )),
    //         }
    //     }
    //     PklToken::For => {
    //         expect_token(parser, PklToken::OpenParenthesis)?;
    //         let ident1 = parse_identifier(parser)?;
    //         let next = retrieve_next_token(parser)?;

    //         match next {
    //             PklToken::Comma => {
    //                 let value = parse_identifier(parser)?;
    //                 expect_token(parser, PklToken::In)?;
    //                 let (iterable, next) = parse_expr(parser, None)?;
    //                 assert_token_eq(parser, next, PklToken::CloseParenthesis)?;
    //                 expect_token(parser, PklToken::OpenBracket)?;

    //                 let members = list_while_not_token3(
    //                     parser,
    //                     &[PklToken::NewLine],
    //                     PklToken::CloseBracket,
    //                     &parse_block_field,
    //                 )?;

    //                 Ok((
    //                     ObjectField::ForGenerator {
    //                         key: Some(ident1),
    //                         value,
    //                         iterable,
    //                         members,
    //                     },
    //                     None,
    //                 ))
    //             }
    //             PklToken::In => {
    //                 let (iterable, next) = parse_expr(parser, None)?;
    //                 assert_token_eq(parser, next, PklToken::CloseParenthesis)?;
    //                 expect_token(parser, PklToken::OpenBracket)?;

    //                 let members = list_while_not_token3(
    //                     parser,
    //                     &[PklToken::NewLine],
    //                     PklToken::CloseBracket,
    //                     &parse_block_field,
    //                 )?;

    //                 Ok((
    //                     ObjectField::ForGenerator {
    //                         key: None,
    //                         value: ident1,
    //                         iterable,
    //                         members,
    //                     },
    //                     None,
    //                 ))
    //             }
    //             _ => Err(ParsingError::unexpected(parser, "'in' or ','".to_owned())),
    //         }
    //     }
    //     token => {
    //         let (expr, next) = parse_expr(parser, Some(token))?;

    //         Ok((ObjectField::Expression(expr), next))
    //     }
    // }
}

impl<'a> fmt::Display for ObjectField<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjectField::Variable { name, value } => write!(f, "{name} = {value}"),
            ObjectField::WhenGenerator {
                condition,
                members,
                _else,
            } => {
                write!(f, "when ({condition}) {{\n")?;

                for member in members {
                    write!(f, "{member}\n")?;
                }

                match _else {
                    Some(else_members) => {
                        write!(f, "}} else {{\n")?;

                        for member in else_members {
                            write!(f, "{member}\n")?;
                        }
                        write!(f, "}}")
                    }
                    None => write!(f, "}}"),
                }
            }
            ObjectField::ForGenerator {
                key,
                value,
                iterable,
                members,
            } => {
                write!(f, "for (")?;
                match key {
                    Some(key) => write!(f, "{key}, {value}")?,
                    None => write!(f, "{value}")?,
                };

                write!(f, " in {iterable}) {{\n")?;

                for member in members {
                    write!(f, "{member}\n")?;
                }

                write!(f, "}}")
            }
            ObjectField::Spread(value) => write!(f, "...{value}"),
            ObjectField::NullableSpread(value) => write!(f, "...?{value}"),
            ObjectField::AmendedValue { value, key } => write!(f, "[{key}] {value}"),
            ObjectField::DefaultObject(x) => {
                write!(f, "default {{\n")?;

                for y in x {
                    write!(f, "\t{y}\n")?;
                }

                write!(f, "}}")
            }
            ObjectField::MemberPredicate { match_expr, values } => {
                write!(f, "[[{match_expr}]] {{\n")?;

                for y in values {
                    write!(f, "\t{y}\n")?;
                }

                write!(f, "}}")
            }
            ObjectField::Expression(x) => write!(f, "{x}"),
        }
    }
}
