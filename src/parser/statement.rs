use super::expr::PklExpr;
use class::ClassDeclaration;
use constant::Constant;
use import::Import;
use logos::Span;
use module::Module;
use typealias::TypeAlias;

pub mod class;
pub mod constant;
pub mod import;
pub mod module;
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

    /// A module clause, used to declare a module name
    ModuleClause(Module<'a>),
}
/* ANCHOR_END: statements */

impl<'a> PklStatement<'a> {
    pub fn span(&self) -> Span {
        match self {
            PklStatement::Constant(Constant { span, .. }) => span.clone(),
            PklStatement::Import(Import { span, .. }) => span.clone(),
            PklStatement::Class(ClassDeclaration { span, .. }) => span.clone(),
            PklStatement::TypeAlias(TypeAlias { span, .. }) => span.clone(),
            PklStatement::ModuleClause(Module { span, .. }) => span.clone(),
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
