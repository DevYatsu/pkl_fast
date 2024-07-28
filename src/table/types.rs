use crate::PklValue;
// use hashbrown::HashMap;

#[derive(Debug, Clone, PartialEq)]
/// Representation of a Pkl Type
pub enum PklType {
    Basic(String),
    StringLiteral(String),
    Union(Box<PklType>, Box<PklType>),
    Nullable(Box<PklType>),

    WithAttributes {
        name: String,
        attributes: Vec<PklType>,
    },

    WithRequirement {
        base_type: Box<PklType>,
        requirements: Box<PklValue>,
    },
}

// #[derive(Debug, Clone, PartialEq)]
// pub enum TypeKind {
//     Named(String),
//     DerivedOf(String),
//     Union(Box<TypeKind>, Box<TypeKind>),

//     ReferTo(TypeKind),
// }

// pub struct TypeHierarchy {
//     hierarchy: HashMap<String, Vec>,
// }

// impl From<Vec<&str>> for Vec<TypeKind> {
//     fn from(value: Vec<&str>) -> Self {
//         value
//             .into_iter()
//             .map(|s| TypeKind::Named(s.to_owned()))
//             .collect()
//     }
// }

// impl TypeHierarchy {
//     pub fn new() -> Self {
//         let mut hierarchy = HashMap::new();

//         hierarchy.insert(
//             "Any".to_owned(),
//             vec![
//                 "Any",
//                 "Null",
//                 "Module",
//                 "Annotation",
//                 "Boolean",
//                 "String",
//                 "Duration",
//                 "DataSize",
//                 "Number",
//                 "Object",
//             ],
//         );

//         hierarchy.insert("Object".to_owned(), vec!["Dynamic", "Typed"]);

//         hierarchy.insert("Number".to_owned(), vec!["Int", "Float"]);
//         hierarchy.insert(
//             "Int".to_owned(),
//             vec!["Int8", "Int16", "Int32", "UInt8", "UInt16", "UInt32"],
//         );

//         hierarchy.insert(
//             "Annotation".to_owned(),
//             vec![
//                 "Deprecated",
//                 "AlsoKnownAs",
//                 "Unlisted",
//                 "DocExample",
//                 "SourceCode",
//                 "ModuleInfo",
//             ],
//         );

//         TypeHierarchy { hierarchy }
//     }

//     pub fn is_type_parent_of(&self, parent_type: PklType, son_type: PklType) -> bool {
//         match parent_type {
//             PklType::Basic(name) => self.hierarchy.get(name),
//             PklType::StringLiteral(_) => todo!(),
//             PklType::Union(_, _) => todo!(),
//             PklType::Nullable(_) => todo!(),
//             PklType::WithAttributes { name, attributes } => todo!(),
//             PklType::WithRequirement {
//                 base_type,
//                 requirements,
//             } => todo!(),
//         }
//     }
// }
