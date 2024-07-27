use super::expr::PklExpr;
use class::ClassField;
use hashbrown::HashMap;
use std::ops::{Deref, DerefMut, Range};

pub mod class;
pub mod constant;
pub mod import;

/* ANCHOR: statements */
/// Represent any valid Pkl value.
#[derive(Debug, PartialEq, Clone)]
pub enum PklStatement<'a> {
    /// A constant/variable statement
    Constant(&'a str, PklExpr<'a>, Range<usize>),

    /// Am import statement:
    /// - name: &str
    /// - local name: Option<&str>
    Import(&'a str, Option<&'a str>, Range<usize>),

    /// A class declaration
    Class(&'a str, HashMap<ClassField<'a>, PklExpr<'a>>, Range<usize>),
}
/* ANCHOR_END: statements */

impl<'a> Deref for PklStatement<'a> {
    type Target = PklExpr<'a>;

    fn deref(&self) -> &Self::Target {
        match self {
            PklStatement::Constant(_, value, _) => value,
            PklStatement::Import(_, _, _) => unreachable!(),
        }
    }
}
impl<'a> DerefMut for PklStatement<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            PklStatement::Constant(_, value, _) => value,
            PklStatement::Import(_, _, _) => unreachable!(),
        }
    }
}
impl<'a> PklStatement<'a> {
    pub fn span(&self) -> Range<usize> {
        match self {
            PklStatement::Constant(_, _, rng) => rng.clone(),
            PklStatement::Import(_, _, rng) => rng.clone(),
        }
    }
    pub fn is_import(&self) -> bool {
        matches!(self, &PklStatement::Import(_, _, _))
    }
    pub fn is_constant(&self) -> bool {
        matches!(self, &PklStatement::Constant(_, _, _))
    }
}
