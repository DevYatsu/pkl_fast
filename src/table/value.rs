use super::base::duration::Duration;
use crate::values::Byte;
use hashbrown::HashMap;

/// Represents a value in the PKL format.
///
/// The `PklValue` enum encapsulates various types of values that can be parsed from a PKL string.
/// These include booleans, floats, integers, strings, multiline strings, objects, and class instances.
///
/// # Variants
///
/// * `Bool` - Represents a boolean value.
/// * `Float` - Represents a floating-point number.
/// * `Int` - Represents an integer, which can be decimal, octal, hex, or binary.
/// * `String` - Represents a single-line string.
/// * `MultiLineString` - Represents a multiline string.
/// * `Object` - Represents a nested object (Dynamic Object), which is a hashmap of key-value pairs.
/// * `ClassInstance` - Represents an instance of a class (Typed Object), which includes the class name and its properties.
#[derive(Debug, PartialEq, Clone)]
pub enum PklValue {
    Null,

    /// A boolean value.
    Bool(bool),

    /// A floating-point number.
    Float(f64),

    /// An integer value.
    Int(i64),

    /// A single-line string.
    ///
    /// String are String and not &str
    /// because we may need to manipulate and modify them.
    String(String),

    /// A List
    List(Vec<PklValue>),

    /// A nested object represented as a hashmap of key-value pairs.
    ///
    /// It represents a [Dynamic object](https://pkl-lang.org/main/current/language-reference/index.html#typed-objects)
    /// in the documentation.
    Object(HashMap<String, PklValue>),

    /// An instance of a class, including the class name it is refering to and its properties.
    ///
    /// It represents a [Typed object](https://pkl-lang.org/main/current/language-reference/index.html#typed-objects)
    /// in the documentation.
    ClassInstance(String, HashMap<String, PklValue>),

    /// A duration
    Duration(Duration),

    // A datasize
    DataSize(Byte),
}

impl PklValue {
    pub fn get_type(&self) -> &str {
        match self {
            PklValue::Null => "Null",
            PklValue::Bool(_) => "Boolean",
            PklValue::Float(_) => "Float",
            PklValue::Int(_) => "Int",
            PklValue::String(_) => "String",
            PklValue::List(_) => "List",
            PklValue::Object(_) => "Dynamic",
            PklValue::ClassInstance(class_name, _) => &class_name,
            PklValue::Duration(_) => "Duration",
            PklValue::DataSize(_) => "DataSize",
        }
    }

    pub fn is_string(&self) -> bool {
        matches!(self, PklValue::String(_))
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, PklValue::Bool(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, PklValue::Float(_) | PklValue::Int(_))
    }

    pub fn is_null(&self) -> bool {
        matches!(self, PklValue::Null)
    }

    pub fn is_list(&self) -> bool {
        matches!(self, PklValue::List(_))
    }

    pub fn is_object(&self) -> bool {
        matches!(self, PklValue::Object(_))
    }

    pub fn is_datasize(&self) -> bool {
        matches!(self, PklValue::DataSize(_))
    }

    pub fn is_duration(&self) -> bool {
        matches!(self, PklValue::Duration(_))
    }

    pub fn as_string(&self) -> Option<&String> {
        if let PklValue::String(ref s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let PklValue::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            PklValue::Float(f) => Some(*f),
            PklValue::Int(i) => Some(*i as f64),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            PklValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            PklValue::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&Vec<PklValue>> {
        if let PklValue::List(ref l) = self {
            Some(l)
        } else {
            None
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, PklValue>> {
        if let PklValue::Object(ref o) = self {
            Some(o)
        } else {
            None
        }
    }

    pub fn as_datasize(&self) -> Option<&Byte> {
        if let PklValue::DataSize(ref d) = self {
            Some(d)
        } else {
            None
        }
    }

    pub fn as_duration(&self) -> Option<&Duration> {
        if let PklValue::Duration(ref d) = self {
            Some(d)
        } else {
            None
        }
    }
}

impl From<bool> for PklValue {
    fn from(value: bool) -> Self {
        PklValue::Bool(value)
    }
}

impl From<f64> for PklValue {
    fn from(value: f64) -> Self {
        PklValue::Float(value)
    }
}

impl From<i64> for PklValue {
    fn from(value: i64) -> Self {
        PklValue::Int(value)
    }
}

impl From<String> for PklValue {
    fn from(value: String) -> Self {
        PklValue::String(value)
    }
}

impl From<Vec<PklValue>> for PklValue {
    fn from(value: Vec<PklValue>) -> Self {
        PklValue::List(value)
    }
}

impl From<HashMap<String, PklValue>> for PklValue {
    fn from(value: HashMap<String, PklValue>) -> Self {
        PklValue::Object(value)
    }
}

impl From<(String, HashMap<String, PklValue>)> for PklValue {
    fn from(value: (String, HashMap<String, PklValue>)) -> Self {
        PklValue::ClassInstance(value.0, value.1)
    }
}

impl From<Duration> for PklValue {
    fn from(value: Duration) -> Self {
        PklValue::Duration(value)
    }
}

impl From<Byte> for PklValue {
    fn from(value: Byte) -> Self {
        PklValue::DataSize(value)
    }
}

impl From<()> for PklValue {
    fn from(_: ()) -> Self {
        PklValue::Null
    }
}
