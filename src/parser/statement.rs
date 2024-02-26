mod amends;
mod as_statement;
mod constant;
mod extends;
mod import;
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
    Module(&'a str),
    Extends(&'a str),
}

pub use amends::parse_amends;
pub use as_statement::parse_as;
pub use extends::parse_extends;
pub use import::ImportClause;
pub use import::{parse_globbed_import, parse_import};
pub use module::parse_module;
