use crate::{parser::types::AstPklType, PklValue};
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

impl PklType {
    pub fn can_be_any(&self) -> bool {
        match self {
            PklType::Basic(x) if x == "Any" => true,
            PklType::Union(a, b) => a.can_be_any() || b.can_be_any(),
            PklType::Nullable(a) if a.can_be_any() => true,
            _ => false,
        }
    }
    pub fn can_be_nullable(&self) -> bool {
        match self {
            PklType::Basic(x) if x == "Null" => true,
            PklType::Union(a, b) => a.can_be_nullable() || b.can_be_nullable(),
            PklType::Nullable(_) => true,
            x if x.can_be_any() => true,
            _ => false,
        }
    }
    pub fn can_be_instance_of(&self, name: &str) -> bool {
        match self {
            PklType::Basic(x) if x == name => true,
            PklType::Union(a, b) => a.can_be_instance_of(name) || b.can_be_instance_of(name),
            PklType::Nullable(x) if x.can_be_instance_of(name) => true,
            x if x.can_be_any() => true,
            _ => false,
        }
    }

    pub fn can_be_bool(&self) -> bool {
        match self {
            PklType::Basic(x) if x == "Boolean" => true,
            PklType::Union(a, b) => a.can_be_bool() || b.can_be_bool(),
            PklType::Nullable(x) if x.can_be_bool() => true,
            x if x.can_be_any() => true,
            _ => false,
        }
    }
    pub fn can_be_str(&self, s: &str) -> bool {
        match self {
            PklType::Basic(x) if x == "String" => true,
            PklType::Union(a, b) => a.can_be_str(s) || b.can_be_str(s),
            PklType::Nullable(x) if x.can_be_str(s) => true,
            PklType::StringLiteral(target_s) if target_s == s => true,
            PklType::WithRequirement { base_type, .. } => base_type.can_be_str(s),
            x if x.can_be_any() => true,
            _ => false,
        }
    }
    pub fn can_be_collection(&self) -> bool {
        match self {
            PklType::Basic(x) if x == "Collection" => true,
            PklType::Union(a, b) => a.can_be_collection() || b.can_be_collection(),
            PklType::Nullable(x) if x.can_be_collection() => true,
            PklType::WithAttributes { name: x, .. } if x == "Collection" => true,
            PklType::WithRequirement { base_type, .. } => base_type.can_be_collection(),
            x if x.can_be_any() => true,
            _ => false,
        }
    }
    pub fn can_be_list(&self, elements: &Vec<PklValue>) -> bool {
        match self {
            PklType::Basic(x) if x == "List" => true,
            PklType::Union(a, b) => a.can_be_list(elements) || b.can_be_list(elements),
            PklType::Nullable(x) if x.can_be_list(elements) => true,
            PklType::WithAttributes {
                name: x,
                attributes,
            } if x == "List" => {
                if attributes.len() != 1 {
                    return false;
                }

                elements
                    .into_iter()
                    .map(|e| e.is_instance_of(attributes.get(0).unwrap()))
                    .all(|e| e)
            }
            PklType::WithRequirement { base_type, .. } => base_type.can_be_list(elements),
            x if x.can_be_collection() => true,
            _ => false,
        }
    }
    pub fn can_be_object(&self) -> bool {
        match self {
            PklType::Basic(x) if x == "Object" => true,
            PklType::Union(a, b) => a.can_be_object() || b.can_be_object(),
            PklType::Nullable(x) if x.can_be_object() => true,
            PklType::WithRequirement { base_type, .. } => base_type.can_be_object(),
            x if x.can_be_any() => true,
            _ => false,
        }
    }
    pub fn can_be_dynamic(&self) -> bool {
        match self {
            PklType::Basic(x) if x == "Dynamic" => true,
            PklType::Union(a, b) => a.can_be_object() || b.can_be_object(),
            PklType::Nullable(x) if x.can_be_object() => true,
            PklType::WithRequirement { base_type, .. } => base_type.can_be_object(),
            x if x.can_be_object() => true,
            _ => false,
        }
    }
    pub fn can_be_typed(&self) -> bool {
        match self {
            PklType::Basic(x) if x == "Typed" => true,
            PklType::Union(a, b) => a.can_be_object() || b.can_be_object(),
            PklType::Nullable(x) if x.can_be_object() => true,
            PklType::WithRequirement { base_type, .. } => base_type.can_be_object(),
            x if x.can_be_object() => true,
            _ => false,
        }
    }

    pub fn can_be_duration(&self) -> bool {
        match self {
            PklType::Basic(x) if x == "Duration" => true,
            PklType::Union(a, b) => a.can_be_duration() || b.can_be_duration(),
            PklType::Nullable(x) if x.can_be_duration() => true,
            PklType::WithRequirement { base_type, .. } => base_type.can_be_duration(),
            x if x.can_be_any() => true,
            _ => false,
        }
    }
    pub fn can_be_datasize(&self) -> bool {
        match self {
            PklType::Basic(x) if x == "DataSize" => true,
            PklType::Union(a, b) => a.can_be_datasize() || b.can_be_datasize(),
            PklType::Nullable(x) if x.can_be_datasize() => true,
            PklType::WithRequirement { base_type, .. } => base_type.can_be_datasize(),
            x if x.can_be_any() => true,
            _ => false,
        }
    }
    pub fn can_be_number(&self) -> bool {
        match self {
            PklType::Basic(x) if x == "Number" => true,
            PklType::Union(a, b) => a.can_be_number() || b.can_be_number(),
            PklType::Nullable(x) if x.can_be_number() => true,
            PklType::WithRequirement { base_type, .. } => base_type.can_be_number(),
            x if x.can_be_any() => true,
            _ => false,
        }
    }
    pub fn can_be_float(&self) -> bool {
        match self {
            PklType::Basic(x) if x == "Float" => true,
            PklType::Union(a, b) => a.can_be_float() || b.can_be_float(),
            PklType::Nullable(x) if x.can_be_float() => true,
            PklType::WithRequirement { base_type, .. } => base_type.can_be_float(),
            x if x.can_be_number() => true,
            _ => false,
        }
    }
    pub fn can_be_int(&self, i: i64) -> bool {
        match self {
            PklType::Basic(x) if x == "Int" => true,
            PklType::Basic(x) if x == "Int8" && i >= i8::MIN as i64 && i <= i8::MAX as i64 => true,
            PklType::Basic(x) if x == "Int16" && i >= i16::MIN as i64 && i <= i16::MAX as i64 => {
                true
            }
            PklType::Basic(x) if x == "Int32" && i >= i32::MIN as i64 && i <= i32::MAX as i64 => {
                true
            }
            PklType::Basic(x) if x == "UInt8" && i >= u8::MIN as i64 && i <= u8::MAX as i64 => true,
            PklType::Basic(x) if x == "UInt16" && i >= u16::MIN as i64 && i <= u16::MAX as i64 => {
                true
            }
            PklType::Basic(x) if x == "UInt32" && i >= u32::MIN as i64 && i <= u32::MAX as i64 => {
                true
            }

            PklType::Union(a, b) => a.can_be_int(i) || b.can_be_int(i),
            PklType::Nullable(x) if x.can_be_int(i) => true,
            PklType::WithRequirement { base_type, .. } => base_type.can_be_int(i),
            x if x.can_be_number() => true,
            _ => false,
        }
    }
}

impl<'a> From<AstPklType<'a>> for PklType {
    fn from(value: AstPklType<'a>) -> Self {
        match value {
            AstPklType::Basic(a, _) => PklType::Basic(a.to_owned()),
            AstPklType::StringLiteral(a, _) => PklType::StringLiteral(a.to_owned()),
            AstPklType::Union(a, b) => PklType::Union(Box::new((*a).into()), Box::new((*b).into())),
            AstPklType::Nullable(a) => PklType::Nullable(Box::new((*a).into())),
            AstPklType::WithAttributes {
                name,
                attributes,
                span,
            } => PklType::WithAttributes {
                name: name.to_owned(),
                attributes: attributes.into_iter().map(|a| a.into()).collect(),
            },
            AstPklType::WithRequirement {
                base_type,
                requirements,
                span,
            } => {
                todo!();
                // PklType::WithRequirement {
                //     base_type: Box::new((*base_type).into()),
                //     requirements,
                // }
            }
        }
    }
}

use std::fmt;
impl fmt::Display for PklType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PklType::Basic(name) => write!(f, "{}", name),
            PklType::StringLiteral(literal) => write!(f, "\"{}\"", literal),
            PklType::Union(left, right) => write!(f, "{} | {}", left, right),
            PklType::Nullable(inner) => write!(f, "{}?", inner),
            PklType::WithAttributes { name, attributes } => {
                let attrs = attributes
                    .iter()
                    .map(|attr| format!("{}", attr))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{}<{}>", name, attrs)
            }
            PklType::WithRequirement {
                base_type,
                requirements,
            } => {
                write!(f, "{}({:?})", base_type, requirements)
            }
        }
    }
}
