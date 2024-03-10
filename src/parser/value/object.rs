use std::fmt;

use winnow::{
    ascii::{multispace0, multispace1, space0},
    combinator::{alt, cut_err, delimited, opt, preceded, separated},
    token::one_of,
    PResult, Parser,
};

use crate::{
    parser::{
        expression::{parse_expr, Expression},
        utils::{expected, identifier},
    },
    prelude::PklValue,
};

use super::parse_value;

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
pub fn parse_object<'source>(input: &mut &'source str) -> PResult<PklValue<'source>> {
    // '{' already parsed

    let values = parse_object_values.parse_next(input)?;        
    multispace0.parse_next(input)?;

    cut_err('}')
        .context(expected("closing bracket"))
        .parse_next(input)?;

    if opt(preceded(multispace0, '{')).parse_next(input)?.is_some() {
        let chained_body = parse_object_values.map(|v| Some(v)).parse_next(input)?;

        cut_err('}')
            .context(expected("closing bracket"))
            .parse_next(input)?;

        return Ok(PklValue::Object {
            values,
            amended_by: None,
            chained_body,
        });
    }

    Ok(PklValue::Object {
        values,
        amended_by: None,
        chained_body: None,
    })

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
    separated(
        0..,
        parse_block_field,
        opt(delimited(space0, one_of([';', '\n']), space0)),
    )
    .parse_next(input)
}

fn parse_block_field<'source>(input: &mut &'source str) -> PResult<ObjectField<'source>> {
    alt((
        variable_field,
        default_field,
        when_generator_field,
        for_generator_field,
        spread_syntax_field,
    ))
    .parse_next(input)

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

fn variable_field<'source>(input: &mut &'source str) -> PResult<ObjectField<'source>> {
    let name = identifier(input)?;
    multispace0.parse_next(input)?;

    let (_, _, value) = alt((
        ('=', multispace0, parse_value),
        ('{', multispace0, parse_object),
    ))
    .parse_next(input)?;

    Ok(ObjectField::Variable {
        name,
        value: Expression::Value(value),
    })
}

fn default_field<'source>(input: &mut &'source str) -> PResult<ObjectField<'source>> {
    "default".parse_next(input)?;
    let values = parse_object_values(input)?;

    Ok(ObjectField::DefaultObject(values))
}

/// covers `Spread` and `NullableSpread`
fn spread_syntax_field<'source>(input: &mut &'source str) -> PResult<ObjectField<'source>> {
    "...".parse_next(input)?;

    alt((
        ('?', identifier).map(|(_, id)| ObjectField::NullableSpread(id)),
        (identifier.map(|id| ObjectField::NullableSpread(id))),
    ))
    .parse_next(input)
}

fn when_generator_field<'source>(input: &mut &'source str) -> PResult<ObjectField<'source>> {
    "when".parse_next(input)?;
    multispace0.parse_next(input)?;
    cut_err('(')
        .context(expected("opening parenthesis"))
        .parse_next(input)?;

    let condition = parse_expr(input)?;

    cut_err(')')
        .context(expected("closing parenthesis"))
        .parse_next(input)?;
    multispace0.parse_next(input)?;
    cut_err('{')
        .context(expected("opening bracket"))
        .parse_next(input)?;

    let members = parse_object_values(input)?;
    cut_err('}')
        .context(expected("closing bracket"))
        .parse_next(input)?;

    let _else = opt(preceded(multispace0, when_generator_else_branch)).parse_next(input)?;

    Ok(ObjectField::WhenGenerator {
        condition,
        members,
        _else,
    })
}
fn when_generator_else_branch<'source>(
    input: &mut &'source str,
) -> PResult<Vec<ObjectField<'source>>> {
    "else".parse_next(input)?;
    multispace0.parse_next(input)?;
    cut_err('{')
        .context(expected("opening bracket"))
        .parse_next(input)?;
    let members = parse_object_values(input)?;
    cut_err('}')
        .context(expected("closing bracket"))
        .parse_next(input)?;

    Ok(members)
}

fn for_generator_field<'source>(input: &mut &'source str) -> PResult<ObjectField<'source>> {
    "for".parse_next(input)?;
    multispace0.parse_next(input)?;
    cut_err('(')
        .context(expected("opening parenthesis"))
        .parse_next(input)?;
    multispace0.parse_next(input)?;

    let (key, value, iterable) = for_generator_iterables(input)?;

    cut_err(')')
        .context(expected("closing parenthesis"))
        .parse_next(input)?;
    multispace0.parse_next(input)?;
    cut_err('{')
        .context(expected("opening bracket"))
        .parse_next(input)?;

    let members = parse_object_values(input)?;
    cut_err('}')
        .context(expected("closing bracket"))
        .parse_next(input)?;

    let _else = opt(preceded(multispace0, when_generator_else_branch)).parse_next(input)?;

    Ok(ObjectField::ForGenerator {
        key,
        value,
        iterable,
        members,
    })
}

fn for_generator_iterables<'source>(
    input: &mut &'source str,
) -> PResult<(Option<&'source str>, &'source str, Expression<'source>)> {
    let first_id = identifier(input)?;
    let second_id = opt(preceded(
        multispace0,
        preceded(',', preceded(multispace0, identifier)),
    ))
    .parse_next(input)?;

    multispace1.parse_next(input)?;
    cut_err("in").context(expected("in")).parse_next(input)?;
    multispace1.parse_next(input)?;

    let iterable = parse_expr(input)?;

    // return (key, value, iterable) yet if Some(second_id), second_id is value otherwise it ain't
    if second_id.is_some() {
        Ok((Some(first_id), second_id.unwrap(), iterable))
    } else {
        Ok((second_id, first_id, iterable))
    }
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
