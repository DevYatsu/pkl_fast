use super::{expr::PklExpr, types::AstPklType, Identifier};
use class::{ClassField, ClassKind};
use hashbrown::HashMap;
use logos::Span;
use std::ops::{Deref, DerefMut};

pub mod class;
pub mod constant;
pub mod import;
pub mod typealias;

/* ANCHOR: statements */
/// Represent any valid Pkl value.
#[derive(Debug, PartialEq, Clone)]
pub enum PklStatement<'a> {
    /// A constant/variable statement
    Constant {
        name: Identifier<'a>,
        _type: Option<AstPklType<'a>>,
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
        name: Identifier<'a>,
        _type: ClassKind,
        extends: Option<Identifier<'a>>,
        fields: HashMap<ClassField<'a>, AstPklType<'a>>,
        span: Span,
    },

    /// A typealias
    TypeAlias {
        name: Identifier<'a>,
        attributes: Vec<Identifier<'a>>,
        refering_type: AstPklType<'a>,
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
            PklStatement::TypeAlias { .. } => unreachable!(),
        }
    }
}
impl<'a> DerefMut for PklStatement<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            PklStatement::Constant { value, .. } => value,
            PklStatement::Import { .. } => unreachable!(),
            PklStatement::Class { .. } => unreachable!(),
            PklStatement::TypeAlias { .. } => unreachable!(),
        }
    }
}
impl<'a> PklStatement<'a> {
    pub fn span(&self) -> Span {
        match self {
            PklStatement::Constant { span, .. } => span.clone(),
            PklStatement::Import { span, .. } => span.clone(),
            PklStatement::Class { span, .. } => span.clone(),
            PklStatement::TypeAlias { span, .. } => span.clone(),
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
