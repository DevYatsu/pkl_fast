mod amends;
mod extends;
mod identifier;
mod import;
mod module;
mod typealias;

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
    Module(&'a str),
    Extends(&'a str),
    VariableDeclaration {
        name: &'a str,
        optional_type: Option<PklType<'a>>,
        value: PklValue<'a>,
    },
    TypeAlias {
        alias: &'a str,
        equivalent_type: PklType<'a>,
    },
}

pub use amends::parse_amends;
pub use extends::parse_extends;
pub use identifier::parse_identifier_statement;
pub use import::ImportClause;
pub use import::{parse_globbed_import, parse_import};
pub use module::parse_module;
pub use typealias::parse_typealias;

use super::types::PklType;
use super::value::PklValue;
