use super::expr::PklExpr;
use class::{ClassField, ClassType};
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
    Constant {
        name: &'a str,
        value: PklExpr<'a>,
        span: Range<usize>,
    },

    /// Am import statement
    Import {
        name: &'a str,
        local_name: Option<&'a str>,
        span: Range<usize>,
    },

    /// A class declaration
    Class {
        name: &'a str,
        _type: ClassType,
        extends: Option<&'a str>,
        fields: HashMap<ClassField<'a>, PklExpr<'a>>,
        span: Range<usize>,
    },
}
/* ANCHOR_END: statements */

impl<'a> Deref for PklStatement<'a> {
    type Target = PklExpr<'a>;

    fn deref(&self) -> &Self::Target {
        match self {
            PklStatement::Constant { value, .. } => value,
            PklStatement::Import { .. } => unreachable!(),
            PklStatement::Class { .. } => unreachable!(),
        }
    }
}
impl<'a> DerefMut for PklStatement<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            PklStatement::Constant { value, .. } => value,
            PklStatement::Import { .. } => unreachable!(),
            PklStatement::Class { .. } => unreachable!(),
        }
    }
}
impl<'a> PklStatement<'a> {
    pub fn span(&self) -> Range<usize> {
        match self {
            PklStatement::Constant { span, .. } => span.clone(),
            PklStatement::Import { span, .. } => span.clone(),
            PklStatement::Class { span, .. } => span.clone(),
        }
    }
    pub fn is_import(&self) -> bool {
        matches!(self, &PklStatement::Import { .. })
    }
    pub fn is_constant(&self) -> bool {
        matches!(self, &PklStatement::Constant { .. })
    }
    pub fn is_class_declaration(&self) -> bool {
        matches!(self, &PklStatement::Class { .. })
    }
}
