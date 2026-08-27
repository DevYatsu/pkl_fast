use std::collections::BTreeMap;
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

/// Convenience trait for deserializing from a Pkl `Value`.
///
/// Automatically implemented for any type that implements `serde::Deserialize`.
/// Prefer implementing `serde::Deserialize` directly via `#[derive(serde::Deserialize)]`.
pub trait PklDecode: Sized {
    /// Decode `Self` from a Pkl `Value`.
    fn decode(value: Value) -> PklResult<Self>;
}

// Blanket impl: any serde Deserialize type can be decoded from a Value
impl<T: serde::de::DeserializeOwned> PklDecode for T {
    fn decode(value: Value) -> PklResult<Self> {
        serde::Deserialize::deserialize(&value)
            .map_err(|e| PklError::DecodeError(format!("serde: {}", e)))
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
            let items = match data.first() {
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
