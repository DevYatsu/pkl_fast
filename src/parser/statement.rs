use super::{type_annotation::PklType, value::PklValue};

#[derive(Debug)]
pub enum PklStatement<'a> {
    ModuleInfo(Vec<PklStatement<'a>>),
    Module(&'a str),
    Import(&'a str),
    Extends(&'a str),
    Amends(&'a str),

    Const { name: &'a str, value: PklValue<'a> },
    TypeAnnotation { name: &'a str, value: PklType<'a> },
}

pub fn parse_statement(line: &str) -> Option<PklStatement> {
    let line = line.trim();
    match line {
        line if line.starts_with("module") => Some(PklStatement::Module(
            line.trim_start_matches("module").trim(),
        )),
        line if line.starts_with("extends") => Some(PklStatement::Extends(
            line.trim_start_matches("extends").trim(),
        )),
        line if line.starts_with("import") => Some(PklStatement::Import(
            line.trim_start_matches("import").trim(),
        )),
        line if line.starts_with("amends") => Some(PklStatement::Amends(
            line.trim_start_matches("amends").trim(),
        )),
        line if line.starts_with("import") => Some(PklStatement::Import(
            line.trim_start_matches("import").trim(),
        )),

        _ => None,
    }
}
