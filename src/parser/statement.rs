mod amends;
mod class;
mod extends;
pub mod import;
mod info;
mod module;
mod var_declaration;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement<'a> {
    Import {
        clause: ImportClause<'a>,
        imported_as: Option<&'a str>,
        is_globbed: bool,
    },
    Amends(&'a str),
    Module {
        value: ModuleSegment<'a>,
        open: bool,
    },
    Extends(&'a str),
    VariableDeclaration {
        name: &'a str,
        optional_type: Option<PklType<'a>>,
        value: Expression<'a>,
        is_local: bool,
    },
    ClassDeclaration {
        name: &'a str,
        extends: Option<&'a str>,
        _type: ClassType,
        fields: Option<HashMap<&'a str, ClassArgument<'a>>>,
    },

    TypeAlias {
        alias: &'a str,
        generics_params: Option<Vec<PklType<'a>>>,
        equivalent_type: PklType<'a>,
    },
    /// Info represents the information annotation may be `@ModuleInfo` or `@Deprecated` for instance.
    ///
    /// **WARNING**: The name can contain a dot, that's the case for `@go.Package` in the pkl-go package.
    Info {
        name: &'a str,
        infos: Vec<InfoField<'a>>,
    },

    Expression(Expression<'a>),
}

use std::collections::HashMap;

pub use self::class::ClassType;
use self::info::InfoField;
use self::module::ModuleSegment;
pub use amends::amends_statement;
pub use class::parse_class_field;
pub use class::ClassArgument;
pub use extends::extends_statement;
pub use import::ImportClause;
pub use info::info_statement;
pub use module::{module_statement, open_module_statement};
pub use var_declaration::var_statement;

use super::expression::Expression;
use super::types::PklType;
