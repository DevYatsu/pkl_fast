use winnow::{
    ascii::{multispace0, multispace1},
    combinator::{alt, cut_err, opt, preceded, terminated},
    token::one_of,
    PResult, Parser,
};

use crate::parser::{
    expression::Expression,
    types::{parse_type, PklType},
    utils::{expected, identifier},
    value::{parse_object, parse_value},
};

use super::Statement;

pub fn var_statement<'source>(input: &mut &'source str) -> PResult<Statement<'source>> {
    let is_local = is_local.parse_next(input)?;

    let name = identifier.parse_next(input)?;
    multispace0.parse_next(input)?;
    let optional_type = opt(parse_var_type).parse_next(input)?;

    if let Some(_type) = opt(parse_var_type).parse_next(input)? {
        if let Some(_) = opt(preceded(multispace0, '=')).parse_next(input)? {
            let value = parse_value.parse_next(input)?;

            return Ok(Statement::VariableDeclaration {
                name,
                optional_type,
                value: Expression::Value(value),
                is_local,
            });
        }

        let value = Expression::Value(_type.default_value(input)?);
        return Ok(Statement::VariableDeclaration {
            name,
            optional_type,
            value,
            is_local,
        });
    }

    multispace0.parse_next(input)?;

    let (_, _, value) = alt((
        ('=', multispace0, parse_value),
        ('{', multispace0, parse_object),
    ))
    .parse_next(input)?;
    Ok(Statement::VariableDeclaration {
        name,
        optional_type,
        value: Expression::Value(value),
        is_local,
    })
}

pub fn is_local<'source>(input: &mut &'source str) -> PResult<bool> {
    opt(terminated(
        "local",
        cut_err(multispace1).context(expected("space")),
    ))
    .map(|opt| opt.is_some())
    .parse_next(input)
}

pub fn parse_var_type<'source>(input: &mut &'source str) -> PResult<PklType<'source>> {
    ':'.parse_next(input)?;
    multispace0(input)?;

    cut_err(parse_type)
        .context(expected("type"))
        .parse_next(input)
}
