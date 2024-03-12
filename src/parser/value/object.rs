use std::fmt;

use winnow::{
    ascii::{multispace0, space0},
    combinator::{alt, cut_err, delimited, opt, preceded, separated},
    token::one_of,
    PResult, Parser,
};

use crate::{
    parser::{
        expression::{parse_expr, Expression},
        utils::{
            cut_multispace1, expected,
            id::{cut_identifier, identifier},
        },
    },
    prelude::PklValue,
};

use super::{parse_value, utils::object_kind_list};

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
}

/// Function called to parse an object, starting with `{` and ending with `}`.
pub fn object<'source>(input: &mut &'source str) -> PResult<PklValue<'source>> {
    let values = object_values.parse_next(input)?;

    if opt(preceded(multispace0, '{')).parse_next(input)?.is_some() {
        let chained_body = object_values.map(|v| Some(v)).parse_next(input)?;

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
}

/// Parse object values.
///
/// **The ending `}` keyword will be parsed or a cut error will get returned**.
pub fn object_values<'source>(input: &mut &'source str) -> PResult<Vec<ObjectField<'source>>> {
    object_kind_list(object_field).parse_next(input)
}

fn object_field<'source>(input: &mut &'source str) -> PResult<ObjectField<'source>> {
    alt((
        variable_field,
        when_generator_field,
        for_generator_field,
        spread_syntax_field,
    ))
    .parse_next(input)
}

fn variable_field<'source>(input: &mut &'source str) -> PResult<ObjectField<'source>> {
    let name = identifier.parse_next(input)?;
    multispace0.parse_next(input)?;

    let (_, _, value) =
        alt((('=', multispace0, parse_value), ('{', multispace0, object))).parse_next(input)?;

    Ok(ObjectField::Variable {
        name,
        value: Expression::Value(value),
    })
}

/// covers `Spread` and `NullableSpread`
fn spread_syntax_field<'source>(input: &mut &'source str) -> PResult<ObjectField<'source>> {
    "...".parse_next(input)?;

    alt((
        ('?', cut_identifier).map(|(_, id)| ObjectField::NullableSpread(id)),
        (cut_identifier.map(|id| ObjectField::Spread(id))),
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

    let members = object_values(input)?;
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
    let members = object_values(input)?;
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

    let members = object_values(input)?;
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
    let first_id = cut_identifier(input)?;
    let second_id = opt(preceded(
        multispace0,
        preceded((',', multispace0), cut_identifier),
    ))
    .parse_next(input)?;

    cut_multispace1.parse_next(input)?;
    cut_err("in").context(expected("in")).parse_next(input)?;
    cut_multispace1.parse_next(input)?;

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
        }
    }
}
