use std::cmp::Ordering;
use std::collections::BTreeMap;
use crate::types::{DataSize, Duration};

/// Represents a runtime Pkl value produced by the evaluator.
///
/// This mirrors Pkl's type system and is the intermediate representation
/// used by `PklDecode` implementations to deserialize into Rust types.
#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Boolean(bool),
    Int(i64),
    Float(f64),
    String(String),
    Duration(Duration),
    DataSize(DataSize),
    /// A Pkl object (Dynamic or Typed) with named properties.
    Object(BTreeMap<String, Value>),
    /// A Pkl Listing (ordered, lazy collection).
    Listing(Vec<Value>),
    /// A Pkl Mapping (keyed, lazy collection).
    Mapping(BTreeMap<Value, Value>),
    /// A Pkl List (eager).
    List(Vec<Value>),
    /// A Pkl Set (eager, unique).
    Set(Vec<Value>),
    /// A Pkl Map (eager key-value).
    Map(Vec<(Value, Value)>),
    /// Pkl Pair.
    Pair(Box<(Value, Value)>),
    /// Pkl IntSeq.
    IntSeq { start: i64, end: i64, step: i64 },
    /// Pkl Regex.
    Regex(String),
    /// Pkl Bytes.
    Bytes(Vec<u8>),
}

// Manual PartialEq + Eq — f64 doesn't implement Eq
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (Null, Null) => true,
            (Boolean(a), Boolean(b)) => a == b,
            (Int(a), Int(b)) => a == b,
            (Float(a), Float(b)) => a.to_bits() == b.to_bits(),
            (String(a), String(b)) => a == b,
            (Duration(a), Duration(b)) => a == b,
            (DataSize(a), DataSize(b)) => a == b,
            (Object(a), Object(b)) => a == b,
            (Listing(a), Listing(b)) => a == b,
            (Mapping(a), Mapping(b)) => a == b,
            (List(a), List(b)) => a == b,
            (Set(a), Set(b)) => a == b,
            (Map(a), Map(b)) => a == b,
            (Pair(a), Pair(b)) => a == b,
            (IntSeq { start: s1, end: e1, step: st1 }, IntSeq { start: s2, end: e2, step: st2 }) => {
                s1 == s2 && e1 == e2 && st1 == st2
            }
            (Regex(a), Regex(b)) => a == b,
            (Bytes(a), Bytes(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Value {}

// Manual Ord implementation — f64 doesn't implement Ord, so we use total ordering
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        use Value::*;
        // Order by variant discriminant first
        let variant_order = |v: &Value| match v {
            Null => 0,
            Boolean(_) => 1,
            Int(_) => 2,
            Float(_) => 3,
            String(_) => 4,
            Duration(_) => 5,
            DataSize(_) => 6,
            Object(_) => 7,
            Listing(_) => 8,
            Mapping(_) => 9,
            List(_) => 10,
            Set(_) => 11,
            Map(_) => 12,
            Pair(_) => 13,
            IntSeq { .. } => 14,
            Regex(_) => 15,
            Bytes(_) => 16,
        };
        let ord = variant_order(self).cmp(&variant_order(other));
        if ord != Ordering::Equal {
            return ord;
        }
        // Same variant — compare by content
        match (self, other) {
            (Null, Null) => Ordering::Equal,
            (Boolean(a), Boolean(b)) => a.cmp(b),
            (Int(a), Int(b)) => a.cmp(b),
            (Float(a), Float(b)) => a.total_cmp(b),
            (String(a), String(b)) => a.cmp(b),
            (Duration(a), Duration(b)) => a.partial_cmp(b).unwrap_or(Ordering::Equal),
            (DataSize(a), DataSize(b)) => a.partial_cmp(b).unwrap_or(Ordering::Equal),
            (Object(a), Object(b)) => {
                let len_cmp = a.len().cmp(&b.len());
                if len_cmp != Ordering::Equal { return len_cmp; }
                for (ka, va) in a {
                    match b.get(ka) {
                        Some(vb) => {
                            let c = va.cmp(vb);
                            if c != Ordering::Equal { return c; }
                        }
                        None => return Ordering::Greater,
                    }
                }
                Ordering::Equal
            }
            (Listing(a), Listing(b)) => a.cmp(b),
            (Mapping(a), Mapping(b)) => {
                let len_cmp = a.len().cmp(&b.len());
                if len_cmp != Ordering::Equal { return len_cmp; }
                for (ka, va) in a {
                    match b.get(ka) {
                        Some(vb) => {
                            let c = va.cmp(vb);
                            if c != Ordering::Equal { return c; }
                        }
                        None => return Ordering::Greater,
                    }
                }
                Ordering::Equal
            }
            (List(a), List(b)) => a.cmp(b),
            (Set(a), Set(b)) => a.cmp(b),
            (Map(a), Map(b)) => a.len().cmp(&b.len()),
            (Pair(a), Pair(b)) => a.0.cmp(&b.0).then(a.1.cmp(&b.1)),
            (IntSeq { start: s1, end: e1, step: st1 }, IntSeq { start: s2, end: e2, step: st2 }) => {
                s1.cmp(s2).then(e1.cmp(e2)).then(st1.cmp(st2))
            }
            (Regex(a), Regex(b)) => a.cmp(b),
            (Bytes(a), Bytes(b)) => a.cmp(b),
            _ => Ordering::Equal, // unreachable
        }
    }
}

impl Value {
    /// Attempt to downcast this value to a bool.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Attempt to downcast this value to an i64.
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(*i),
            _ => None,
        }
    }

    /// Attempt to downcast this value to a f64.
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// Attempt to downcast this value to a &str.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    /// Attempt to downcast this value to an object (property map).
    pub fn as_object(&self) -> Option<&BTreeMap<String, Value>> {
        match self {
            Value::Object(m) => Some(m),
            _ => None,
        }
    }

    /// Attempt to downcast this value to a list/listing.
    pub fn as_list(&self) -> Option<&[Value]> {
        match self {
            Value::List(v) | Value::Listing(v) => Some(v.as_slice()),
            _ => None,
        }
    }
}
