use super::{expr::PklExpr, types::PklType};
use class::{ClassField, ClassType};
use hashbrown::HashMap;
use logos::Span;
use std::ops::{Deref, DerefMut};

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
        _type: Option<PklType<'a>>,
        value: PklExpr<'a>,
        span: Span,
    },

    /// Am import statement
    Import {
        name: &'a str,
        local_name: Option<&'a str>,
        span: Span,
    },

    /// A class declaration
    Class {
        name: &'a str,
        _type: ClassType,
        extends: Option<&'a str>,
        fields: HashMap<ClassField<'a>, PklType<'a>>,
        span: Span,
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
    pub fn span(&self) -> Span {
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
