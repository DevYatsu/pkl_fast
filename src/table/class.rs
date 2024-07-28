use super::types::PklType;
use crate::parser::statement::class::{ClassDeclaration, ClassField};
use hashbrown::HashMap;

pub type ClassSchema = HashMap<String, PklType>;

pub fn generate_class_schema(
    ClassDeclaration { name, fields, .. }: ClassDeclaration<'_>,
) -> (&str, ClassSchema) {
    let name = name.0;
    let mut types = HashMap::new();

    for (ClassField { name, kind, .. }, _type) in fields {
        types.insert(name.to_owned(), _type.into());
    }

    (name, types)
}
