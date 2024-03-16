use winnow::{
    ascii::multispace0,
    combinator::{alt, cut_err, delimited, opt, preceded},
    Parser,
};

use crate::{
    parser::{
        expression::{parse_expr, Expression},
        types::{parse_type, PklType},
        value::object,
    },
    prelude::ParsingResult,
};

use super::{cut_multispace1, expected, id::identifier_not_keyword};

/// Parses a variable with format (name, Option<Type>, expression)
pub fn variable<'source>(
    input: &mut &'source str,
) -> ParsingResult<(
    &'source str,
    Option<PklType<'source>>,
    Option<Expression<'source>>,
)> {
    let name = identifier_not_keyword.parse_next(input)?;
    multispace0.parse_next(input)?;

    if let Some(_type) = opt(parse_var_type).parse_next(input)? {
        println!("{:?}", _type);
        if let Some(_) = opt(delimited(multispace0, '=', multispace0)).parse_next(input)? {
            let expr = parse_expr.parse_next(input)?;

            return Ok((name, Some(_type), Some(expr)));
        }

        return Ok((name, Some(_type), None));
    }

    // there is no type if we are here

    let expr = alt((
        preceded(
            ('=', multispace0),
            cut_err(parse_expr).context(expected("expr")),
        ),
        object.map(Expression::Value),
    ))
    .parse_next(input)?;

    Ok((name, None, Some(expr)))
}

/// Parses a local variable, returning a cut_err if a variable does not follow `local` keyword.
pub fn local_variable<'source>(
    input: &mut &'source str,
) -> ParsingResult<(
    &'source str,
    Option<PklType<'source>>,
    Option<Expression<'source>>,
)> {
    "local".parse_next(input)?;
    cut_multispace1.parse_next(input)?;

    cut_err(variable).parse_next(input)
}

fn parse_var_type<'source>(input: &mut &'source str) -> ParsingResult<PklType<'source>> {
    ':'.parse_next(input)?;
    multispace0(input)?;

    cut_err(parse_type)
        .context(expected("type"))
        .parse_next(input)
}
