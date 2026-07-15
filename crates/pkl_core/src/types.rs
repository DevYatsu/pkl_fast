use std::fmt;
use serde::{Deserialize, Serialize};

/// Represents a Pkl Duration value.
///
/// Durations have a numeric value and a unit (ns, us, ms, s, min, h, d).
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Duration {
    /// The numeric value of this duration.
    pub value: f64,
    /// The unit string (e.g. "ns", "us", "ms", "s", "min", "h", "d").
    pub unit: String,
}

impl Duration {
    /// Create a new Duration with the given value and unit.
    pub fn new(value: f64, unit: impl Into<String>) -> Self {
        Self {
            value,
            unit: unit.into(),
        }
    }

    /// Convert this duration to nanoseconds (approximate for non-ns units).
    pub fn to_nanos(&self) -> f64 {
        match self.unit.as_str() {
            "ns" => self.value,
            "us" => self.value * 1_000.0,
            "ms" => self.value * 1_000_000.0,
            "s" => self.value * 1_000_000_000.0,
            "min" => self.value * 60_000_000_000.0,
            "h" => self.value * 3_600_000_000_000.0,
            "d" => self.value * 86_400_000_000_000.0,
            _ => self.value,
        }
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}

/// Represents a Pkl DataSize value.
///
/// Data sizes have a numeric value and a unit (b, kb, mb, gb, tb, pb, kib, mib, gib, tib, pib).
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct DataSize {
    /// The numeric value of this data size.
    pub value: f64,
    /// The unit string (e.g. "b", "kb", "mb", "gb", "tb", "pb", "kib", "mib", "gib", "tib", "pib").
    pub unit: String,
}

impl DataSize {
    /// Create a new DataSize with the given value and unit.
    pub fn new(value: f64, unit: impl Into<String>) -> Self {
        Self {
            value,
            unit: unit.into(),
        }
    }

    /// Convert this data size to bytes (approximate for non-byte units).
    pub fn to_bytes(&self) -> f64 {
        match self.unit.as_str() {
            "b" => self.value,
            "kb" => self.value * 1_000.0,
            "mb" => self.value * 1_000_000.0,
            "gb" => self.value * 1_000_000_000.0,
            "tb" => self.value * 1_000_000_000_000.0,
            "pb" => self.value * 1_000_000_000_000_000.0,
            "kib" => self.value * 1_024.0,
            "mib" => self.value * 1_048_576.0,
            "gib" => self.value * 1_073_741_824.0,
            "tib" => self.value * 1_099_511_627_776.0,
            "pib" => self.value * 1_125_899_906_842_624.0,
            _ => self.value,
        }
    }
}

impl fmt::Display for DataSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}
