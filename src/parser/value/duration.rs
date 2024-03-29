#[derive(Debug, PartialEq, PartialOrd, Clone)]
/// A struct representating the Pkl Duration value composed of a `Number` and a unit component of type `String`.
pub struct Duration {
    pub value: DurationValue,
    pub unit: DurationUnit,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
/// DurationValue represents the value of the Duration, which can be an i64 or an f64
pub enum DurationValue {
    Integer(i64),
    Float(f64),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone)]
/// DurationUnit represents the unit of the DataSize, restricted to these values: "ns"|"us"|"ms"|"s"|"min"|"h"|"d".
pub enum DurationUnit {
    NanoSeconds,
    MicroSeconds,
    MilliSeconds,
    Seconds,
    Minutes,
    Hours,
    Days,
}

impl From<&str> for DurationUnit {
    fn from(value: &str) -> Self {
        match value {
            "s" => DurationUnit::Seconds,
            "ns" => DurationUnit::NanoSeconds,
            "ms" => DurationUnit::MilliSeconds,
            "us" => DurationUnit::MicroSeconds,
            "min" => DurationUnit::Minutes,
            "h" => DurationUnit::Hours,
            "d" => DurationUnit::Days,

            _ => unreachable!(),
        }
    }
}

impl From<f64> for DurationValue {
    fn from(value: f64) -> Self {
        DurationValue::Float(value)
    }
}
impl From<i64> for DurationValue {
    fn from(value: i64) -> Self {
        DurationValue::Integer(value)
    }
}

use std::fmt;

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value {
            DurationValue::Integer(i) => write!(f, "{}", i)?,
            DurationValue::Float(float) => write!(f, "{}", float)?,
        };

        write!(f, ".")?;

        match self.unit {
            DurationUnit::NanoSeconds => write!(f, "ns")?,
            DurationUnit::MicroSeconds => write!(f, "us")?,
            DurationUnit::MilliSeconds => write!(f, "ms")?,
            DurationUnit::Seconds => write!(f, "s")?,
            DurationUnit::Minutes => write!(f, "min")?,
            DurationUnit::Hours => write!(f, "h")?,
            DurationUnit::Days => write!(f, "d")?,
        };

        Ok(())
    }
}
