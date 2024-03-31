mod amends;
mod class;
mod extends;
pub mod import;
mod info;
mod module;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement<'a> {
    Import {
        clause: ImportClause<'a>,
        imported_as: Option<&'a str>,
    },
    GlobbedImport {
        clause: ImportClause<'a>,
        imported_as: Option<&'a str>,
    },
    Amends(&'a str),
    Module {
        value: &'a str,
        open: bool,
    },
    Extends(&'a str),
    VariableDeclaration {
        name: &'a str,
        optional_type: Option<PklType<'a>>,
        value: Expression<'a>,
    },
    ClassDeclaration {
        name: &'a str,
        extends: Option<&'a str>,
        _type: ClassType,
        fields: HashMap<&'a str, ClassArgument<'a>>,
    },

    TypeAlias {
        alias: &'a str,
        generics_params: Option<Vec<PklType<'a>>>,
        equivalent_type: PklType<'a>,
    },

    Info {
        name: &'a str,
        infos: Vec<InfoField<'a>>,
    },
}

use std::collections::HashMap;

pub use amends::parse_amends;
pub use class::parse_class_declaration;
pub use class::ClassArgument;
pub use extends::parse_extends;
pub use import::ImportClause;
pub use info::parse_info;
pub use module::parse_module;

pub use self::class::ClassType;
use self::info::InfoField;

use super::expression::Expression;
use super::types::PklType;
