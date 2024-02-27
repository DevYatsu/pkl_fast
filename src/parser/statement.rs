mod amends;
mod extends;
mod import;
mod info;
mod module;
mod typealias;
mod var;

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
    /// ModuleInfo variant represents the annotation @ModuleInfo { package: "version" }
    /// The documentation does not contain any precise information on this annotation, so I write the enum variant so that there can be several infos add in one @ModuleInfo, that is ModuleInfo contains a Vec<ModuleField>
    ModuleInfo {
        infos: Vec<InfoField<'a>>,
    },
    DeprecatedInfo {
        infos: Vec<InfoField<'a>>,
    },
}

pub use amends::parse_amends;
pub use extends::parse_extends;
pub use import::ImportClause;
pub use import::{parse_globbed_import, parse_import};
pub use info::parse_deprecated;
pub use info::parse_module_info;
pub use module::parse_module;
pub use typealias::parse_typealias;
pub use var::parse_var_statement;

use self::info::InfoField;

use super::types::PklType;
use super::value::PklValue;
