use logos::Span;

use super::{expr::PklExpr, ExprHash};

/// Represent any valid Pkl value.
#[derive(Debug, PartialEq, Clone)]
pub enum AstPklValue<'a> {
    Null(Span),

    /// true or false.
    Bool(bool, Span),
    /// Any floating point number.
    Float(f64, Span),
    /// Any Integer.
    Int(i64, Span),

    /// Any quoted string.
    String(&'a str, Span),
    /// Any multiline string.
    MultiLineString(&'a str, Span),

    /// An object.
    Object(ExprHash<'a>),

    /// An object.
    List(Vec<PklExpr<'a>>, Span),

    /// A Class instance.
    ClassInstance(&'a str, ExprHash<'a>, Span),

    /// ### An object amending another object:
    /// - First comes the name of the amended object,
    /// - Then the additional values
    /// - Finally the range
    ///
    /// **Corresponds to:**
    /// ```pkl
    /// x = (other_object) {
    ///     prop = "attribute"
    /// }
    /// ```
    AmendingObject(&'a str, ExprHash<'a>, Span),

    /// ### An amended object.
    /// Different from `AmendingObject`
    ///
    /// **Corresponds to:**
    /// ```pkl
    /// x = {
    ///    prop = "attribute"
    /// } {
    ///    other_prop = "other_attribute"
    /// }
    /// ```
    AmendedObject(Box<AstPklValue<'a>>, ExprHash<'a>, Span),
}

impl<'a> AstPklValue<'a> {
    pub fn span(&self) -> Span {
        match self {
            AstPklValue::Int(_, rng)
            | AstPklValue::Bool(_, rng)
            | AstPklValue::Float(_, rng)
            | AstPklValue::Object((_, rng))
            | AstPklValue::AmendingObject(_, _, rng)
            | AstPklValue::AmendedObject(_, _, rng)
            | AstPklValue::ClassInstance(_, _, rng)
            | AstPklValue::String(_, rng)
            | AstPklValue::List(_, rng)
            | AstPklValue::MultiLineString(_, rng)
            | AstPklValue::Null(rng) => rng.clone(),
        }
    }
}

impl<'a> From<ExprHash<'a>> for AstPklValue<'a> {
    fn from(value: ExprHash<'a>) -> Self {
        AstPklValue::Object(value)
    }
}
