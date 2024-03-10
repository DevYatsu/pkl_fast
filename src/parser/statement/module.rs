use std::{fmt::Display, vec};

use winnow::{
    ascii::multispace1,
    combinator::{cut_err, opt},
    PResult, Parser,
};

use crate::parser::utils::{expected, identifier};

use super::Statement;
pub fn module_statement<'source>(input: &mut &'source str) -> PResult<Statement<'source>> {
    // module keyword already parsed
    cut_err(multispace1)
        .context(expected("space"))
        .parse_next(input)?;
    let value = cut_err(module_segment)
        .context(expected("module name"))
        .parse_next(input)?;

    Ok(Statement::Module { value, open: false })
}

pub fn open_module_statement<'source>(input: &mut &'source str) -> PResult<Statement<'source>> {
    // open keyword already parsed
    multispace1.parse_next(input)?;
    "module".context(expected("module")).parse_next(input)?;
    cut_err(multispace1)
        .context(expected("space"))
        .parse_next(input)?;
    let value = cut_err(module_segment)
        .context(expected("module name"))
        .parse_next(input)?;

    Ok(Statement::Module { value, open: true })
}

#[derive(Debug, Clone, PartialEq)]
/// Represents a module name. A parent is an identifier optionally followed by `.` while a child is another `ModuleSegment`.
pub struct ModuleSegment<'a> {
    name: &'a str,
    child: Option<Box<ModuleSegment<'a>>>,
}

/// Parse dot-separated identifier segments recursively
fn module_segment<'source>(input: &mut &'source str) -> PResult<ModuleSegment<'source>> {
    let name = identifier(input)?;

    if let Some(_) = opt('.').parse_next(input)? {
        return Ok(ModuleSegment {
            name,
            child: Some(Box::new(module_segment(input)?)),
        });
    }

    Ok(ModuleSegment { name, child: None })
}

impl<'a> ModuleSegment<'a> {
    pub fn to_vec(&self) -> Vec<&'a str> {
        let mut current_segment = self;
        let mut vec = vec![current_segment.name];

        while current_segment.child.is_some() {
            current_segment = current_segment.child.as_ref().unwrap();
            vec.push(current_segment.name);
        }

        vec
    }
}

impl Display for ModuleSegment<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if self.child.is_some() {
            write!(f, "{}", self.child.as_ref().unwrap())?;
        }

        Ok(())
    }
}
