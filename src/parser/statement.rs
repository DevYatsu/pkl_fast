use super::expr::PklExpr;
use amends::Amends;
use class::ClassDeclaration;
use constant::Constant;
use extends::Extends;
use import::Import;
use logos::Span;
use module::Module;
use typealias::TypeAlias;

pub mod amends;
pub mod class;
pub mod constant;
pub mod extends;
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

    /// An amends clause, it's like extending
    /// but then you can't create any variable
    /// that is not declared in the amended
    /// module
    AmendsClause(Amends<'a>),

    /// An extends clause, literally it's like importing
    /// but directly in the main context,
    /// not in a variable creating in the context
    /// containing the import values.
    ExtendsClause(Extends<'a>),
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
            PklStatement::AmendsClause(Amends { span, .. }) => span.clone(),
            PklStatement::ExtendsClause(Extends { span, .. }) => span.clone(),
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
