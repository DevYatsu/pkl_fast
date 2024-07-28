use super::expr::PklExpr;
use class::ClassDeclaration;
use constant::Constant;
use import::Import;
use logos::Span;
use std::ops::{Deref, DerefMut};
use typealias::TypeAlias;

pub mod class;
pub mod constant;
pub mod import;
pub mod typealias;

/* ANCHOR: statements */
/// Represent any valid Pkl value.
#[derive(Debug, PartialEq, Clone)]
pub enum PklStatement<'a> {
    /// A constant/variable statement
    Constant(Constant<'a>),

    /// Am import statement
    Import(Import<'a>),

    /// A class declaration
    Class(ClassDeclaration<'a>),

    /// A typealias
    TypeAlias(TypeAlias<'a>),
}
/* ANCHOR_END: statements */

impl<'a> Deref for PklStatement<'a> {
    type Target = PklExpr<'a>;

    fn deref(&self) -> &Self::Target {
        match self {
            PklStatement::Constant(Constant { value, .. }) => value,
            PklStatement::Import { .. } => unreachable!(),
            PklStatement::Class { .. } => unreachable!(),
            PklStatement::TypeAlias { .. } => unreachable!(),
        }
    }
}
impl<'a> DerefMut for PklStatement<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            PklStatement::Constant(Constant { value, .. }) => value,
            PklStatement::Import { .. } => unreachable!(),
            PklStatement::Class { .. } => unreachable!(),
            PklStatement::TypeAlias { .. } => unreachable!(),
        }
    }
}
impl<'a> PklStatement<'a> {
    pub fn span(&self) -> Span {
        match self {
            PklStatement::Constant(Constant { span, .. }) => span.clone(),
            PklStatement::Import(Import { span, .. }) => span.clone(),
            PklStatement::Class(ClassDeclaration { span, .. }) => span.clone(),
            PklStatement::TypeAlias(TypeAlias { span, .. }) => span.clone(),
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
