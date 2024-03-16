use std::{fmt::Display, rc::Rc};

use winnow::{
    ascii::multispace0,
    combinator::{alt, cut_err, delimited, preceded, todo},
    PResult, Parser,
};

use crate::parser::{
    expression::{parse_expr, Expression},
    types::PklType,
    utils::{expected, var::local_variable},
};

use super::{
    amending::utils::default_field,
    object::{object, object_values, ObjectField},
    utils::object_kind_list,
};
#[derive(Debug, PartialEq, Clone)]
pub enum MappingField<'a> {
    LocalVariable {
        name: &'a str,
        _type: Option<PklType<'a>>,
        value: Option<Expression<'a>>,
    },
    DefaultObject(Vec<ObjectField<'a>>),
    AmendingElement {
        key: Rc<Expression<'a>>,
        amended_fields: Vec<ObjectField<'a>>,
    },

    Pair {
        key: Rc<Expression<'a>>,
        value: Expression<'a>,
        dynamic: Option<Expression<'a>>,
    },
}

pub fn mapping<'source>(input: &mut &'source str) -> PResult<Vec<MappingField<'source>>> {
    object_kind_list(mapping_field).parse_next(input)
}

pub fn mapping_field<'source>(input: &mut &'source str) -> PResult<MappingField<'source>> {
    alt((
        local_variable.map(|(name, optional_type, value)| MappingField::LocalVariable {
            name,
            _type: optional_type,
            value,
        }),
        default_field.map(MappingField::DefaultObject),
    ));

    todo(input)
    // match next_token {
    //     PklToken::Local => {
    //         let name = parse_identifier(lexer)?;

    //         match retrieve_next_token(lexer)? {
    //             PklToken::EqualSign => {
    //                 let (value, next) = parse_expr(lexer, None)?;
    //                 Ok((
    //                     MappingField::LocalVariable {
    //                         name,
    //                         _type: None,
    //                         value,
    //                     },
    //                     next,
    //                 ))
    //             }
    //             PklToken::Colon => {
    //                 let (_type, opt_token) = parse_type(lexer, None)?;
    //                 assert_token_eq(lexer, opt_token, PklToken::EqualSign)?;
    //                 let (value, next) = parse_expr(lexer, Some(next_token))?;
    //                 Ok((
    //                     MappingField::LocalVariable {
    //                         name,
    //                         _type: Some(_type),
    //                         value,
    //                     },
    //                     next,
    //                 ))
    //             }
    //             _ => Err(ParsingError::unexpected(lexer, "'=' or ':'".to_owned())),
    //         }
    //     }
    //     PklToken::Default => {
    //         expect_token(lexer, PklToken::OpenBracket)?;
    //         let (value, token) = parse_object(lexer, None)?;

    //         Ok((MappingField::DefaultObject(Expression::Value(value)), token))
    //     }
    //     PklToken::OpenBrace => parse_mapping_variable(lexer),
    //     _ => {
    //         let (expr, next) = parse_expr(lexer, Some(next_token))?;
    //         Ok((MappingField::Expression(expr), next))
    //     }
    // }
}

// parser called whenever a '[' was found
pub fn parse_mapping_variable<'source>(input: &mut &'source str) -> PResult<MappingField<'source>> {
    '['.parse_next(input)?;
    multispace0(input)?;
    let key = Rc::new(parse_expr(input)?);
    multispace0(input)?;
    cut_err(']')
        .context(expected("closing brace"))
        .parse_next(input)?;
    multispace0(input)?;

    let field = alt((
        object_values.map(|amended_fields| MappingField::AmendingElement {
            key: Rc::clone(&key),
            amended_fields,
        }),
        preceded(
            ('=', multispace0),
            alt((
                (
                    delimited(('(', multispace0), parse_expr, (multispace0, ')')).map(Some),
                    object.map(Expression::Value),
                )
                    .map(|(dynamic, value)| MappingField::Pair {
                        key: Rc::clone(&key),
                        value,
                        dynamic,
                    }),
                parse_expr.map(|value| MappingField::Pair {
                    key: Rc::clone(&key),
                    value,
                    dynamic: None,
                }),
            )),
        ),
    ))
    .parse_next(input)?;

    Ok(field)
}

impl<'a> Display for MappingField<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MappingField::LocalVariable { name, value, _type } => {
                if _type.is_some() {
                    if value.is_some() {
                        write!(
                            f,
                            "local {name}: {} = {}",
                            _type.as_ref().unwrap(),
                            value.as_ref().unwrap()
                        )
                    } else {
                        write!(f, "local {name}: {}", _type.as_ref().unwrap())
                    }
                } else {
                    write!(f, "local {name} = {}", value.as_ref().unwrap())
                }
            }
            MappingField::DefaultObject(fields) => {
                write!(f, "default {{\n")?;
                for field in fields {
                    write!(f, "\t{field},\n")?;
                }
                write!(f, "}}")
            }
            MappingField::AmendingElement {
                key,
                amended_fields,
            } => {
                write!(f, "({key}) {{\n")?;
                for field in amended_fields {
                    write!(f, "\t{field}\n")?;
                }
                write!(f, "}}")
            }
            MappingField::Pair {
                key,
                value,
                dynamic,
            } => {
                if dynamic.is_some() {
                    write!(f, "[{key}] = ({}) {value}", dynamic.as_ref().unwrap())
                } else {
                    write!(f, "[{key}] = {value}")
                }
            }
        }
    }
}
