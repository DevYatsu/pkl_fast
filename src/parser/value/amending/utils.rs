use winnow::{ascii::multispace0, PResult, Parser};

use crate::parser::value::object::{object_values, ObjectField};

/// Parsing a default value, starting with `default` keyword and being an object.
pub fn default_field<'source>(input: &mut &'source str) -> PResult<Vec<ObjectField<'source>>> {
    "default".parse_next(input)?;
    multispace0.parse_next(input)?;
    let values = object_values(input)?;

    Ok(values)
}
