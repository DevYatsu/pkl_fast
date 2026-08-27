use crate::error::PklError;
use crate::types::{DataSize, Duration};
use crate::value::Value;
use serde::de::{self, Deserializer, Error, Visitor};
use serde::forward_to_deserialize_any;
use serde::Deserialize;
use std::collections::BTreeMap;

impl<'de> Deserializer<'de> for &'de Value {
    type Error = PklError;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self {
            Value::Null => visitor.visit_unit(),
            Value::Boolean(b) => visitor.visit_bool(*b),
            Value::Int(i) => visitor.visit_i64(*i),
            Value::Float(f) => visitor.visit_f64(*f),
            Value::String(s) => visitor.visit_str(s),
            Value::Object(_m) => {
                // Use a direct approach: visit the object as a custom map
                struct ObjMap<'a>(&'a BTreeMap<String, Value>, usize);
                impl<'de> de::MapAccess<'de> for ObjMap<'_> {
                    type Error = PklError;
                    fn next_key_seed<K: de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
                        if self.1 >= self.0.len() { return Ok(None); }
                        let key = self.0.keys().nth(self.1).unwrap();
                        self.1 += 1;
                        seed.deserialize(de::value::StrDeserializer::<PklError>::new(key)).map(Some)
                    }
                    fn next_value_seed<V: de::DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
                        let key = self.0.keys().nth(self.1 - 1).unwrap();
                        seed.deserialize(&self.0[key])
                    }
                }
                visitor.visit_map(&mut ObjMap(m, 0))
            }
            Value::List(items) | Value::Listing(items) | Value::Set(items) => {
                visitor.visit_seq(PklValSeq(items.iter().collect()))
            }
            Value::Pair(p) => {
                visitor.visit_seq(PklValSeq(vec![&p.0, &p.1]))
            }
            Value::IntSeq { start, end, step } => {
                let items = vec![Value::Int(*start), Value::Int(*end), Value::Int(*step)];
                let refs: Vec<&Value> = items.iter().collect();
                visitor.visit_seq(PklValSeq(refs))
            }
            Value::Bytes(b) => {
                let items: Vec<Value> = b.iter().map(|&x| Value::Int(x as i64)).collect();
                let refs: Vec<&Value> = items.iter().collect();
                visitor.visit_seq(PklValSeq(refs))
            }
            Value::Mapping(m) => {
                struct MapMap<'a>(&'a BTreeMap<Value, Value>, Vec<&'a Value>, usize);
                impl<'de> de::MapAccess<'de> for MapMap<'_> {
                    type Error = PklError;
                    fn next_key_seed<K: de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
                        if self.2 >= self.1.len() { return Ok(None); }
                        seed.deserialize(self.1[self.2]).map(Some)
                    }
                    fn next_value_seed<V: de::DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
                        let k = self.1[self.2];
                        self.2 += 1;
                        seed.deserialize(&self.0[k])
                    }
                }
                let keys: Vec<&Value> = m.keys().collect();
                visitor.visit_map(&mut MapMap(m, keys, 0))
            }
            Value::Map(pairs) => {
                let vs: Vec<Value> = pairs.iter().map(|(k, v)| Value::List(vec![k.clone(), v.clone()])).collect();
                let refs: Vec<&Value> = vs.iter().collect();
                visitor.visit_seq(PklValSeq(refs))
            }
            Value::Duration(d) => {
                visitor.visit_map(&mut DurationMap::new(d))
            }
            Value::DataSize(d) => {
                visitor.visit_map(&mut DataSizeMap::new(d))
            }
            Value::Pair(p) => {
                let items = vec![&p.0 as &Value, &p.1 as &Value];
                visitor.visit_seq(de::value::SeqDeserializer::new(items.into_iter()))
            }
            Value::IntSeq { start, end, step } => {
                let items = vec![Value::Int(*start), Value::Int(*end), Value::Int(*step)];
                let refs: Vec<&Value> = items.iter().collect();
                visitor.visit_seq(de::value::SeqDeserializer::new(refs.into_iter()))
            }
            Value::Regex(s) => visitor.visit_str(s),
            Value::Bytes(b) => {
                let items: Vec<Value> = b.iter().map(|&x| Value::Int(x as i64)).collect();
                let refs: Vec<&Value> = items.iter().collect();
                visitor.visit_seq(de::value::SeqDeserializer::new(refs.into_iter()))
            }
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

// ── Duration/DataSize as Map ──

struct DurationMap {
    val: Value,
    unit: Value,
    state: u8,
}

impl DurationMap {
    fn new(d: &Duration) -> Self {
        Self {
            val: Value::Float(d.value),
            unit: Value::String(d.unit.clone()),
            state: 0,
        }
    }
}

impl<'de> de::MapAccess<'de> for DurationMap {
    type Error = PklError;
    fn next_key_seed<K: de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
        if self.state >= 2 { return Ok(None); }
        let key = if self.state == 0 { "value" } else { "unit" };
        self.state += 1;
        seed.deserialize(de::value::StrDeserializer::<PklError>::new(key)).map(Some)
    }
    fn next_value_seed<V: de::DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
        let val = if self.state - 1 == 0 { &self.val } else { &self.unit };
        seed.deserialize(val)
    }
}

struct DataSizeMap {
    val: Value,
    unit: Value,
    state: u8,
}

impl DataSizeMap {
    fn new(d: &DataSize) -> Self {
        Self {
            val: Value::Float(d.value),
            unit: Value::String(d.unit.clone()),
            state: 0,
        }
    }
}

impl<'de> de::MapAccess<'de> for DataSizeMap {
    type Error = PklError;
    fn next_key_seed<K: de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
        if self.state >= 2 { return Ok(None); }
        let key = if self.state == 0 { "value" } else { "unit" };
        self.state += 1;
        seed.deserialize(de::value::StrDeserializer::<PklError>::new(key)).map(Some)
    }
    fn next_value_seed<V: de::DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
        let val = if self.state - 1 == 0 { &self.val } else { &self.unit };
        seed.deserialize(val)
    }
}

// ── Serde error conversion ──

impl de::Error for PklError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        PklError::DecodeError(msg.to_string())
    }
}

// ── Duration/DataSize Deserialize impls ──

impl<'de> Deserialize<'de> for Duration {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Dur { value: f64, unit: String }
        let helper = Dur::deserialize(d)?;
        Ok(Duration::new(helper.value, helper.unit))
    }
}

impl<'de> Deserialize<'de> for DataSize {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Ds { value: f64, unit: String }
        let helper = Ds::deserialize(d)?;
        Ok(DataSize::new(helper.value, helper.unit))
    }
}
