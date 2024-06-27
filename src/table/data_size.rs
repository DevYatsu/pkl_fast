// pub const DATA_SIZE_UNITS: [&str; 11] = [
//     "b", "kb", "mb", "gb", "tb", "pb", "kib", "mib", "gib", "tib", "pib",
// ];

use crate::{PklResult, PklValue};
use std::fmt;
use std::ops::Range;

/// Based on v0.26.0
pub fn match_data_size_props_api<'a, 'b>(
    byte: Byte<'b>,
    property: &'a str,
    range: Range<usize>,
) -> PklResult<PklValue<'b>> {
    match property {
        "value" => {
            return Ok(*byte.initial_value);
        }
        "unit" => {
            return Ok(PklValue::String(byte.unit.to_string()));
        }
        "isPositive" => return Ok(PklValue::Bool(byte.bytes >= 0)),
        "isBinaryUnit" => {
            return Ok(PklValue::Bool(match byte.unit {
                Unit::B | Unit::GiB | Unit::KiB | Unit::MiB | Unit::PiB | Unit::TiB => true,
                _ => false,
            }))
        }
        "isDecimalUnit" => {
            return Ok(PklValue::Bool(match byte.unit {
                Unit::GiB | Unit::KiB | Unit::MiB | Unit::PiB | Unit::TiB => false,
                _ => true,
            }))
        }
        _ => {
            return Err((
                format!("DataSize does not possess {} property", property),
                range,
            ))
        }
    }
}

/// An enum representing both binary (kibibytes, mebibytes, etc.)
/// and decimal (kilobytes, megabytes, etc.) data size units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    B,
    KB,
    MB,
    GB,
    TB,
    PB,
    KiB,
    MiB,
    GiB,
    TiB,
    PiB,
}

impl Unit {
    /// Parses a string slice into an `Option<Unit>`.
    /// Returns `None` if the string does not correspond to a known data size unit.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "b" => Some(Unit::B),
            "kb" => Some(Unit::KB),
            "mb" => Some(Unit::MB),
            "gb" => Some(Unit::GB),
            "tb" => Some(Unit::TB),
            "pb" => Some(Unit::PB),
            "kib" => Some(Unit::KiB),
            "mib" => Some(Unit::MiB),
            "gib" => Some(Unit::GiB),
            "tib" => Some(Unit::TiB),
            "pib" => Some(Unit::PiB),
            _ => None,
        }
    }
}

/// Represents data sizes in bytes.
#[derive(Debug, Clone, PartialEq)]
pub struct Byte<'a> {
    bytes: i64,
    initial_value: Box<PklValue<'a>>,
    unit: Unit,
}

impl<'a> Byte<'a> {
    /// Creates a new `Byte` from a floating point value and a unit.
    ///
    /// # Arguments
    /// * `value` - The numeric value of the data size.
    /// * `unit` - The unit of the data size (`Unit`).
    ///
    /// # Returns
    /// Returns a new `Byte` representing the size in bytes.
    pub fn from_float_and_unit(value: f64, unit: Unit) -> Self {
        let bytes = calculate_bytes(value, unit);
        Byte {
            bytes,
            initial_value: Box::new(PklValue::Float(value)),
            unit,
        }
    }

    /// Creates a new `Byte` from a i64 value and a unit.
    ///
    /// # Arguments
    /// * `value` - The numeric value of the data size.
    /// * `unit` - The unit of the data size (`Unit`).
    ///
    /// # Returns
    /// Returns a new `Byte` representing the size in bytes.
    pub fn from_int_and_unit(value: i64, unit: Unit) -> Self {
        let bytes = calculate_bytes(value as f64, unit);
        Byte {
            bytes,
            initial_value: Box::new(PklValue::Int(value)),
            unit,
        }
    }
}

fn calculate_bytes(value: f64, unit: Unit) -> i64 {
    let bytes = match unit {
        Unit::B => value,
        Unit::KB => value * 1_000.0,
        Unit::MB => value * 1_000_000.0,
        Unit::GB => value * 1_000_000_000.0,
        Unit::TB => value * 1_000_000_000_000.0,
        Unit::PB => value * 1_000_000_000_000_000.0,
        Unit::KiB => value * 1_024.0,
        Unit::MiB => value * 1_024.0 * 1_024.0,
        Unit::GiB => value * 1_024.0 * 1_024.0 * 1_024.0,
        Unit::TiB => value * 1_024.0 * 1_024.0 * 1_024.0 * 1_024.0,
        Unit::PiB => value * 1_024.0 * 1_024.0 * 1_024.0 * 1_024.0 * 1_024.0,
    };

    bytes as i64
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unit_str = match self {
            Unit::B => "b",
            Unit::KB => "kb",
            Unit::MB => "mb",
            Unit::GB => "gb",
            Unit::TB => "tb",
            Unit::PB => "pb",
            Unit::KiB => "kib",
            Unit::MiB => "mib",
            Unit::GiB => "gib",
            Unit::TiB => "tib",
            Unit::PiB => "pib",
        };
        write!(f, "{}", unit_str)
    }
}
