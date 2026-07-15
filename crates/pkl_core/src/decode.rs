use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;
use crate::error::{PklError, PklResult};
use crate::types::{DataSize, Duration};
use crate::value::Value;

// Pkl binary protocol type codes
const CODE_OBJECT: u8 = 0x01;
const CODE_MAP: u8 = 0x02;
const CODE_MAPPING: u8 = 0x03;
const CODE_LIST: u8 = 0x04;
const CODE_LISTING: u8 = 0x05;
const CODE_SET: u8 = 0x06;
const CODE_DURATION: u8 = 0x07;
const CODE_DATASIZE: u8 = 0x08;
const CODE_PAIR: u8 = 0x09;
const CODE_INT_SEQ: u8 = 0x0A;
const CODE_REGEX: u8 = 0x0B;
const CODE_CLASS: u8 = 0x0C;
const CODE_TYPE_ALIAS: u8 = 0x0D;
const CODE_BYTES: u8 = 0x0F;

/// The `PklDecode` trait — similar to `serde::Deserialize` but for Pkl's value model.
///
/// Types that implement this trait can be deserialized from a Pkl `Value`.
/// You can derive it with `#[derive(PklDecode)]` for structs.
pub trait PklDecode: Sized {
    /// Decode `Self` from a Pkl `Value`.
    fn decode(value: Value) -> PklResult<Self>;
}

// ── Primitive implementations ──

impl PklDecode for Value {
    fn decode(value: Value) -> PklResult<Self> {
        Ok(value)
    }
}

impl PklDecode for bool {
    fn decode(value: Value) -> PklResult<Self> {
        value.as_bool().ok_or_else(|| PklError::TypeMismatch {
            expected: "bool",
            actual: format!("{:?}", value),
        })
    }
}

impl PklDecode for i64 {
    fn decode(value: Value) -> PklResult<Self> {
        value.as_int().ok_or_else(|| PklError::TypeMismatch {
            expected: "Int",
            actual: format!("{:?}", value),
        })
    }
}

impl PklDecode for i32 {
    fn decode(value: Value) -> PklResult<Self> {
        let v = i64::decode(value)?;
        i32::try_from(v).map_err(|_| PklError::DecodeError(format!("Int {} out of range for i32", v)))
    }
}

impl PklDecode for i16 {
    fn decode(value: Value) -> PklResult<Self> {
        let v = i64::decode(value)?;
        i16::try_from(v).map_err(|_| PklError::DecodeError(format!("Int {} out of range for i16", v)))
    }
}

impl PklDecode for i8 {
    fn decode(value: Value) -> PklResult<Self> {
        let v = i64::decode(value)?;
        i8::try_from(v).map_err(|_| PklError::DecodeError(format!("Int {} out of range for i8", v)))
    }
}

impl PklDecode for u64 {
    fn decode(value: Value) -> PklResult<Self> {
        let v = value.as_int().ok_or_else(|| PklError::TypeMismatch {
            expected: "UInt",
            actual: format!("{:?}", value),
        })?;
        u64::try_from(v).map_err(|_| PklError::DecodeError(format!("Int {} out of range for u64", v)))
    }
}

impl PklDecode for u32 {
    fn decode(value: Value) -> PklResult<Self> {
        let v = i64::decode(value)?;
        u32::try_from(v).map_err(|_| PklError::DecodeError(format!("Int {} out of range for u32", v)))
    }
}

impl PklDecode for u16 {
    fn decode(value: Value) -> PklResult<Self> {
        let v = i64::decode(value)?;
        u16::try_from(v).map_err(|_| PklError::DecodeError(format!("Int {} out of range for u16", v)))
    }
}

impl PklDecode for u8 {
    fn decode(value: Value) -> PklResult<Self> {
        let v = i64::decode(value)?;
        u8::try_from(v).map_err(|_| PklError::DecodeError(format!("Int {} out of range for u8", v)))
    }
}

impl PklDecode for f64 {
    fn decode(value: Value) -> PklResult<Self> {
        match value {
            Value::Float(f) => Ok(f),
            Value::Int(i) => Ok(i as f64),
            other => Err(PklError::TypeMismatch {
                expected: "Float",
                actual: format!("{:?}", other),
            }),
        }
    }
}

impl PklDecode for String {
    fn decode(value: Value) -> PklResult<Self> {
        value.as_str().map(|s| s.to_string()).ok_or_else(|| PklError::TypeMismatch {
            expected: "String",
            actual: format!("{:?}", value),
        })
    }
}

impl PklDecode for Duration {
    fn decode(value: Value) -> PklResult<Self> {
        match value {
            Value::Duration(d) => Ok(d),
            other => Err(PklError::TypeMismatch {
                expected: "Duration",
                actual: format!("{:?}", other),
            }),
        }
    }
}

impl PklDecode for DataSize {
    fn decode(value: Value) -> PklResult<Self> {
        match value {
            Value::DataSize(d) => Ok(d),
            other => Err(PklError::TypeMismatch {
                expected: "DataSize",
                actual: format!("{:?}", other),
            }),
        }
    }
}

impl<K: PklDecode + Eq + std::hash::Hash, V: PklDecode> PklDecode for HashMap<K, V> {
    fn decode(value: Value) -> PklResult<Self> {
        match value {
            Value::Map(pairs) => {
                let mut map = HashMap::new();
                for (k, v) in pairs {
                    map.insert(K::decode(k)?, V::decode(v)?);
                }
                Ok(map)
            }
            Value::Mapping(m) => {
                let mut map = HashMap::new();
                for (k, v) in m {
                    map.insert(K::decode(k)?, V::decode(v)?);
                }
                Ok(map)
            }
            Value::Object(props) => {
                let mut map = HashMap::new();
                for (k, v) in props {
                    map.insert(K::decode(Value::String(k))?, V::decode(v)?);
                }
                Ok(map)
            }
            other => Err(PklError::TypeMismatch {
                expected: "Map/Mapping/Object",
                actual: format!("{:?}", other),
            }),
        }
    }
}

impl<T: PklDecode + std::cmp::Eq + std::hash::Hash> PklDecode for HashSet<T> {
    fn decode(value: Value) -> PklResult<Self> {
        match value {
            Value::Set(items) | Value::List(items) | Value::Listing(items) => {
                items.into_iter().map(T::decode).collect()
            }
            other => Err(PklError::TypeMismatch {
                expected: "Set/List/Listing",
                actual: format!("{:?}", other),
            }),
        }
    }
}

impl<T: PklDecode> PklDecode for Box<T> {
    fn decode(value: Value) -> PklResult<Self> {
        T::decode(value).map(Box::new)
    }
}

impl<T: PklDecode> PklDecode for Arc<T> {
    fn decode(value: Value) -> PklResult<Self> {
        T::decode(value).map(Arc::new)
    }
}

impl<'a> PklDecode for Cow<'a, str> {
    fn decode(value: Value) -> PklResult<Self> {
        String::decode(value).map(Cow::Owned)
    }
}

// Generate PklDecode for fixed-size arrays [T; N] for N = 0..=32
macro_rules! impl_arrays {
    ($($n:expr),+ $(,)?) => {
        $(
            impl<T: PklDecode> PklDecode for [T; $n] {
                fn decode(value: Value) -> PklResult<Self> {
                    let items = <Vec<T>>::decode(value)?;
                    if items.len() != $n {
                        return Err(PklError::DecodeError(
                            format!("expected array of length {}, got {}", $n, items.len())
                        ));
                    }
                    let mut arr: [std::mem::MaybeUninit<T>; $n] =
                        unsafe { std::mem::MaybeUninit::uninit().assume_init() };
                    for (i, item) in items.into_iter().enumerate() {
                        arr[i] = std::mem::MaybeUninit::new(item);
                    }
                    unsafe { Ok(std::mem::transmute_copy(&arr)) }
                }
            }
        )+
    };
}

impl_arrays!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);

impl<T: PklDecode> PklDecode for Vec<T> {
    fn decode(value: Value) -> PklResult<Self> {
        match value {
            Value::List(items) | Value::Listing(items) | Value::Set(items) => {
                items.into_iter().map(T::decode).collect()
            }
            other => Err(PklError::TypeMismatch {
                expected: "List/Listing/Set",
                actual: format!("{:?}", other),
            }),
        }
    }
}

impl<K: PklDecode + Ord, V: PklDecode> PklDecode for BTreeMap<K, V> {
    fn decode(value: Value) -> PklResult<Self> {
        match value {
            Value::Map(pairs) => {
                let mut map = BTreeMap::new();
                for (k, v) in pairs {
                    map.insert(K::decode(k)?, V::decode(v)?);
                }
                Ok(map)
            }
            Value::Mapping(m) => {
                let mut map = BTreeMap::new();
                for (k, v) in m {
                    map.insert(K::decode(k)?, V::decode(v)?);
                }
                Ok(map)
            }
            Value::Object(props) => {
                // Object properties have string keys — requires K = String
                let mut map = BTreeMap::new();
                for (k, v) in props {
                    map.insert(K::decode(Value::String(k))?, V::decode(v)?);
                }
                Ok(map)
            }
            other => Err(PklError::TypeMismatch {
                expected: "Map/Mapping/Object",
                actual: format!("{:?}", other),
            }),
        }
    }
}

impl<T: PklDecode> PklDecode for Option<T> {
    fn decode(value: Value) -> PklResult<Self> {
        match value {
            Value::Null => Ok(None),
            other => Ok(Some(T::decode(other)?)),
        }
    }
}

// ── Binary Decoder (from Pkl CLI MessagePack output) ──
//
// The Pkl CLI encodes values using a custom protocol over MessagePack.
// Each value is an array where the first element is a type code integer:
//
//   [code, ...data]
//
// For objects, the format is:
//   [0x01, name, moduleUri, [member, member, ...]]
//
// Where each member is:
//   [0x10, name, value]  (property)
//   [0x11, key, value]   (entry)
//   [0x12, value]        (element)

use rmpv::Value as RmpValue;
use rmpv::decode::read_value;

const MAX_DECODE_DEPTH: usize = 128;

/// Decode raw bytes from the Pkl CLI's output format into a `Value` tree.
pub fn decode_response(bytes: &[u8]) -> PklResult<Value> {
    let mut cursor = std::io::Cursor::new(bytes);
    let rmp = read_value(&mut cursor)
        .map_err(|e| PklError::DecodeError(format!("MessagePack decode: {}", e)))?;
    decode_rmpv_depth(&rmp, 0)
}

fn decode_rmpv_depth(val: &RmpValue, depth: usize) -> PklResult<Value> {
    if depth > MAX_DECODE_DEPTH {
        return Err(PklError::DecodeError(
            "exceeded maximum decode depth (128)".to_string()
        ));
    }
    match val {
        RmpValue::Nil => Ok(Value::Null),
        RmpValue::Boolean(b) => Ok(Value::Boolean(*b)),
        RmpValue::Integer(i) => {
            if let Some(n) = i.as_i64() {
                Ok(Value::Int(n))
            } else if let Some(n) = i.as_u64() {
                Ok(Value::Int(n as i64))
            } else {
                Err(PklError::DecodeError(format!("Integer overflow: {:?}", i)))
            }
        }
        RmpValue::F32(f) => Ok(Value::Float(*f as f64)),
        RmpValue::F64(f) => Ok(Value::Float(*f)),
        RmpValue::String(s) => Ok(Value::String(
            s.as_str().ok_or_else(|| PklError::DecodeError("non-UTF-8 string".to_string()))?.to_string(),
        )),
        RmpValue::Binary(b) => Ok(Value::Bytes(b.clone())),
        RmpValue::Array(arr) => {
            if arr.is_empty() {
                return Err(PklError::DecodeError("empty array in Pkl protocol".to_string()));
            }
            let code = match &arr[0] {
                RmpValue::Integer(i) => i.as_u64().ok_or_else(|| {
                    PklError::DecodeError("invalid type code".to_string())
                })? as u8,
                _ => {
                    let items = arr.iter()
                        .map(|v| decode_rmpv_depth(v, depth + 1))
                        .collect::<PklResult<Vec<_>>>()?;
                    return Ok(Value::List(items));
                }
            };
            decode_tagged_array_depth(code, &arr[1..], depth + 1)
        }
        RmpValue::Map(map) => {
            let mut props = BTreeMap::new();
            for (k, v) in map {
                let key = match k {
                    RmpValue::String(s) => s.as_str().unwrap_or("").to_string(),
                    other => format!("{:?}", other),
                };
                props.insert(key, decode_rmpv_depth(v, depth + 1)?);
            }
            Ok(Value::Object(props))
        }
        RmpValue::Ext(_, _) => {
            Err(PklError::DecodeError("extension types not supported".to_string()))
        }
    }
}

fn decode_rmpv(val: &RmpValue) -> PklResult<Value> {
    decode_rmpv_depth(val, 0)
}

fn decode_tagged_array(code: u8, data: &[RmpValue]) -> PklResult<Value> {
    decode_tagged_array_depth(code, data, 0)
}

fn decode_tagged_array_depth(code: u8, data: &[RmpValue], depth: usize) -> PklResult<Value> {
    if depth > MAX_DECODE_DEPTH {
        return Err(PklError::DecodeError(
            "exceeded maximum decode depth (128)".to_string()
        ));
    }
    match code {
        CODE_OBJECT => {
            if data.len() < 3 {
                return Err(PklError::DecodeError(
                    format!("truncated Pkl object: expected 3+ fields, got {}", data.len())
                ));
            }
            let _name = decode_rmpv_depth(&data[0], depth + 1)?;
            let _module_uri = decode_rmpv_depth(&data[1], depth + 1)?;
            let members = decode_object_members(&data[2])?;
            // name might be the "type name" for typed objects
            Ok(Value::Object(members))
        }
        CODE_MAP => {
            // Format: [code, {key: value, ...}]
            if data.is_empty() {
                return Ok(Value::Map(Vec::new()));
            }
            let entries = match &data[0] {
                RmpValue::Map(map) => {
                    let mut pairs = Vec::new();
                    for (k, v) in map {
                        pairs.push((decode_rmpv_depth(k, depth + 1)?, decode_rmpv_depth(v, depth + 1)?));
                    }
                    pairs
                }
                other => return Err(PklError::DecodeError(format!("map entries not a map: {:?}", other))),
            };
            Ok(Value::Map(entries))
        }
        CODE_MAPPING => {
            // Format: [code, {key: value, ...}]
            if data.is_empty() {
                return Ok(Value::Mapping(BTreeMap::new()));
            }
            let entries = match &data[0] {
                RmpValue::Map(map) => {
                    let mut result = BTreeMap::new();
                    for (k, v) in map {
                        result.insert(decode_rmpv_depth(k, depth + 1)?, decode_rmpv_depth(v, depth + 1)?);
                    }
                    result
                }
                other => return Err(PklError::DecodeError(format!("mapping entries not a map: {:?}", other))),
            };
            Ok(Value::Mapping(entries))
        }
        CODE_LIST | CODE_LISTING | CODE_SET => {
            // Format: [code, [elem1, elem2, ...]]
            let items = match data.get(0) {
                Some(RmpValue::Array(arr)) => {
                    arr.iter().map(decode_rmpv).collect::<PklResult<Vec<_>>>()?
                }
                other => return Err(PklError::DecodeError(
                    format!("expected array of list elements, got {:?}", other)
                )),
            };
            match code {
                CODE_SET => Ok(Value::Set(items)),
                CODE_LISTING => Ok(Value::Listing(items)),
                _ => Ok(Value::List(items)),
            }
        }
        CODE_DURATION => {
            if data.len() < 2 {
                return Err(PklError::DecodeError(
                    format!("truncated Duration: expected 2 fields (value, unit), got {}", data.len())
                ));
            }
            let value = match decode_rmpv(&data[0])? {
                Value::Int(i) => i as f64,
                Value::Float(f) => f,
                other => return Err(PklError::DecodeError(format!("duration value: {:?}", other))),
            };
            let unit = match decode_rmpv(&data[1])? {
                Value::String(s) => s,
                other => return Err(PklError::DecodeError(format!("duration unit: {:?}", other))),
            };
            Ok(Value::Duration(Duration { value, unit }))
        }
        CODE_DATASIZE => {
            if data.len() < 2 {
                return Err(PklError::DecodeError(
                    format!("truncated DataSize: expected 2 fields (value, unit), got {}", data.len())
                ));
            }
            let value = match decode_rmpv(&data[0])? {
                Value::Int(i) => i as f64,
                Value::Float(f) => f,
                other => return Err(PklError::DecodeError(format!("datasize value: {:?}", other))),
            };
            let unit = match decode_rmpv(&data[1])? {
                Value::String(s) => s,
                other => return Err(PklError::DecodeError(format!("datasize unit: {:?}", other))),
            };
            Ok(Value::DataSize(DataSize { value, unit }))
        }
        CODE_PAIR => {
            if data.len() < 2 {
                return Err(PklError::DecodeError("truncated pair".to_string()));
            }
            let first = decode_rmpv(&data[0])?;
            let second = decode_rmpv(&data[1])?;
            Ok(Value::Pair(Box::new((first, second))))
        }
        CODE_INT_SEQ => {
            if data.len() < 3 {
                return Err(PklError::DecodeError("truncated intseq".to_string()));
            }
            let start = decode_int_value(&data[0])?;
            let end = decode_int_value(&data[1])?;
            let step = decode_int_value(&data[2])?;
            Ok(Value::IntSeq { start, end, step })
        }
        CODE_REGEX => {
            if data.is_empty() {
                return Err(PklError::DecodeError("truncated regex".to_string()));
            }
            let pattern = match decode_rmpv(&data[0])? {
                Value::String(s) => s,
                other => return Err(PklError::DecodeError(format!("regex pattern: {:?}", other))),
            };
            Ok(Value::Regex(pattern))
        }
        CODE_BYTES => {
            if data.is_empty() {
                return Ok(Value::Bytes(Vec::new()));
            }
            match &data[0] {
                RmpValue::Binary(b) => Ok(Value::Bytes(b.clone())),
                RmpValue::Array(arr) => {
                    let bytes = arr
                        .iter()
                        .map(|v| match v {
                            RmpValue::Integer(i) => i.as_u64().map(|n| n as u8).ok_or_else(|| {
                                PklError::DecodeError("invalid byte value".to_string())
                            }),
                            _ => Err(PklError::DecodeError("non-integer in bytes".to_string())),
                        })
                        .collect::<PklResult<Vec<u8>>>()?;
                    Ok(Value::Bytes(bytes))
                }
                other => {
                    let decoded = decode_rmpv(other)?;
                    Ok(Value::Bytes(format!("{:?}", decoded).into_bytes()))
                }
            }
        }
        CODE_CLASS | CODE_TYPE_ALIAS => {
            // Skip class/type-alias values for now
            Ok(Value::Null)
        }
        _ => Err(PklError::UnknownObjectCode(code)),
    }
}

fn decode_object_members(val: &RmpValue) -> PklResult<BTreeMap<String, Value>> {
    let arr = match val {
        RmpValue::Array(a) => a,
        _ => return Err(PklError::DecodeError(
            format!("expected object members array but got {:?}", val)
        )),
    };
    let mut map = BTreeMap::new();
    for member in arr {
        let member_arr = match member {
            RmpValue::Array(a) => a,
            _ => continue,
        };
        if member_arr.is_empty() {
            continue;
        }
        let member_code = match &member_arr[0] {
            RmpValue::Integer(i) => i.as_u64().unwrap_or(0) as u8,
            _ => continue,
        };
        match member_code {
            0x10 => {
                // Property: [0x10, name, value]
                if member_arr.len() < 3 {
                    continue;
                }
                let name = match &member_arr[1] {
                    RmpValue::String(s) => s.as_str().unwrap_or("").to_string(),
                    other => format!("{:?}", other),
                };
                let value = decode_rmpv(&member_arr[2])?;
                map.insert(name, value);
            }
            0x11 => {
                // Entry: [0x11, key, value] - for mappings inside objects
                // We don't generally have these at top level
            }
            0x12 => {
                // Element: [0x12, value] - for listings inside objects
            }
            _ => {}
        }
    }
    Ok(map)
}

/// Decode JSON bytes from the Pkl CLI's JSON output format into a `Value` tree.
///
/// This is a fallback for when the binary format is not available.
/// Note: JSON output loses Pkl type information (Duration, DataSize, etc.).
pub fn decode_json_response(bytes: &[u8]) -> PklResult<Value> {
    let json_val: serde_json::Value = serde_json::from_slice(bytes)
        .map_err(|e| PklError::DecodeError(format!("JSON decode: {}", e)))?;
    convert_json_value(json_val)
}

fn convert_json_value(val: serde_json::Value) -> PklResult<Value> {
    match val {
        serde_json::Value::Null => Ok(Value::Null),
        serde_json::Value::Bool(b) => Ok(Value::Boolean(b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(Value::Int(i))
            } else if let Some(f) = n.as_f64() {
                Ok(Value::Float(f))
            } else {
                Err(PklError::DecodeError(format!("invalid number: {}", n)))
            }
        }
        serde_json::Value::String(s) => Ok(Value::String(s)),
        serde_json::Value::Array(arr) => {
            let items = arr
                .into_iter()
                .map(convert_json_value)
                .collect::<PklResult<Vec<_>>>()?;
            Ok(Value::List(items))
        }
        serde_json::Value::Object(map) => {
            let mut props = std::collections::BTreeMap::new();
            for (k, v) in map {
                props.insert(k, convert_json_value(v)?);
            }
            Ok(Value::Object(props))
        }
    }
}

fn decode_int_value(val: &RmpValue) -> PklResult<i64> {
    match val {
        RmpValue::Integer(i) => {
            if let Some(n) = i.as_i64() {
                Ok(n)
            } else if let Some(n) = i.as_u64() {
                Ok(n as i64)
            } else {
                Err(PklError::DecodeError("integer overflow".to_string()))
            }
        }
        _ => Err(PklError::DecodeError("expected integer".to_string())),
    }
}
