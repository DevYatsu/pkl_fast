use hashbrown::HashMap;

use crate::PklValue;

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

pub struct TypeHierarchy {
    equivalences: HashMap<String, Vec<PklType>>,
}

impl TypeHierarchy {
    pub fn new() -> Self {
        let mut equivalences = HashMap::new();

        equivalences.insert(
            "Number".into(),
            vec![
                PklType::Basic("Int".to_owned()),
                PklType::Basic("Float".to_owned()),
            ],
        );

        TypeHierarchy { equivalences }
    }

    pub fn is_equivalent(&self, base_type: &str, other_type: &PklType) -> bool {
        if let Some(equivalent_types) = self.equivalences.get(base_type) {
            equivalent_types.contains(other_type)
        } else {
            false
        }
    }
}
