// everything regarding Pkl types

use crate::parser::expr::PklExpr;

#[derive(Debug, Clone, PartialEq)]
/// Representation of a Pkl Type
pub struct PklType<'a> {
    pub name: &'a str,

    /// For types that can take different types in,
    /// that's the case of a Listing or a Mapping
    pub elements: Option<Vec<&'a str>>,

    /// For types that can take requirements,
    /// that's the case of a String (length <= 5),
    pub requirements: Option<PklExpr<'a>>,
}

impl<'a> PklType<'a> {
    pub fn new(
        name: &'a str,
        elements: Option<Vec<&'a str>>,
        requirements: Option<PklExpr<'a>>,
    ) -> Self {
        Self {
            name,
            elements,
            requirements,
        }
    }

    pub fn has_elements(&self) -> bool {
        self.elements.is_some()
    }
    pub fn requirements(&self) -> bool {
        self.requirements.is_some()
    }
}
