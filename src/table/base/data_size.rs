// pub const DATA_SIZE_UNITS: [&str; 11] = [
//     "b", "kb", "mb", "gb", "tb", "pb", "kib", "mib", "gib", "tib", "pib",
// ];

use crate::{generate_method, PklResult, PklValue};
use std::fmt;
use std::ops::Range;

/// Based on v0.26.0
pub fn match_data_size_props_api(
    byte: Byte,
    property: &str,
    range: Range<usize>,
) -> PklResult<PklValue> {
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

/// Based on v0.26.0
pub fn match_data_size_methods_api(
    byte: Byte,
    property: &str,
    args: Vec<PklValue>,
    range: Range<usize>,
) -> PklResult<PklValue> {
    match property {
        "isBetween" => {
            generate_method!(
                "isBetween", &args;
                0: DataSize, 1: DataSize;
                |(start, inclusive_end): (Byte, Byte)| {
                    Ok((byte >= start && byte <= inclusive_end).into())
                };
                range
            )
        }
        "toUnit" => {
            generate_method!(
                "toUnit", &args;
                0: String;
                |unit: String| {
                    if let Some(unit) = Unit::from_str(&unit) {
                        let mut x = byte;
                        x.to_unit(unit);
                        return Ok((x).into())
                    }

                    Err((format!("'{unit}' is not a valid DataSize Unit"), range))
                };
                range
            )
        }
        "toBinaryUnit" => {
            generate_method!(
                "toBinaryUnit", &args;
                {
                    let mut x = byte;
                    x.to_binary_unit();
                    return Ok((x).into())
                };
                range
            )
        }
        "toDecimalUnit" => {
            generate_method!(
                "toDecimalUnit", &args;
                {
                    let mut x = byte;
                    x.to_decimal_unit();
                    return Ok((x).into())
                };
                range
            )
        }
        _ => {
            return Err((
                format!("DataSize does not possess {} method", property),
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
#[derive(Debug, Clone)]
pub struct Byte {
    pub bytes: i64,
    pub is_negative: bool,
    pub unit: Unit,
    initial_value: Box<PklValue>,
    #[allow(dead_code)]
    initial_unit: Unit,
}

impl PartialOrd for Byte {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.is_negative && !other.is_negative {
            return Some(std::cmp::Ordering::Less);
        }
        if !self.is_negative && other.is_negative {
            return Some(std::cmp::Ordering::Greater);
        }

        if self.is_negative && other.is_negative {
            if self.bytes > other.bytes {
                Some(std::cmp::Ordering::Less)
            } else if self.bytes < other.bytes {
                Some(std::cmp::Ordering::Greater)
            } else {
                Some(std::cmp::Ordering::Equal)
            }
        } else {
            // both positive
            if self.bytes > other.bytes {
                Some(std::cmp::Ordering::Greater)
            } else if self.bytes < other.bytes {
                Some(std::cmp::Ordering::Less)
            } else {
                Some(std::cmp::Ordering::Equal)
            }
        }
    }
}

impl PartialEq for Byte {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Byte {
    /// Creates a new `Byte` from a floating point value and a unit.
    ///
    /// # Arguments
    /// * `value` - The numeric value of the data size.
    /// * `unit` - The unit of the data size (`Unit`).
    ///
    /// # Returns
    /// Returns a new `Byte` representing the size in bytes.
    pub fn from_float_and_unit(value: f64, unit: Unit) -> Self {
        let is_negative = value.is_sign_negative();
        let bytes = calculate_bytes(value, unit);
        Byte {
            bytes,
            initial_value: Box::new(PklValue::Float(value)),
            unit,
            initial_unit: unit,
            is_negative,
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
        let is_negative = value.is_negative();
        let bytes = calculate_bytes(value as f64, unit);
        Byte {
            bytes,
            initial_value: Box::new(PklValue::Int(value)),
            unit,
            initial_unit: unit,
            is_negative,
        }
    }

    pub fn to_unit(&mut self, unit: Unit) -> &mut Self {
        self.unit = unit;
        self
    }
    pub fn to_binary_unit(&mut self) -> &mut Self {
        match self.unit {
            Unit::KB => self.unit = Unit::KiB,
            Unit::MB => self.unit = Unit::MiB,
            Unit::GB => self.unit = Unit::GiB,
            Unit::TB => self.unit = Unit::TiB,
            Unit::PB => self.unit = Unit::PiB,
            _ => (),
        }
        self
    }
    pub fn to_decimal_unit(&mut self) -> &mut Self {
        match self.unit {
            Unit::KiB => self.unit = Unit::KB,
            Unit::MiB => self.unit = Unit::MB,
            Unit::GiB => self.unit = Unit::GB,
            Unit::TiB => self.unit = Unit::TB,
            Unit::PiB => self.unit = Unit::PB,
            _ => (),
        }
        self
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
