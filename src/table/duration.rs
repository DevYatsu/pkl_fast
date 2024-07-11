use crate::{generate_method, PklResult, PklValue};
use std::fmt;
use std::{ops::Range, time::Duration as StdDuration};

// pub const DURATION_UNITS: [&str; 7] = ["ns", "us", "ms", "s", "min", "h", "d"];

/// Based on v0.26.0
pub fn match_duration_props_api(
    duration: Duration,
    property: &str,
    range: Range<usize>,
) -> PklResult<PklValue> {
    match property {
        "value" => {
            return Ok(*duration.initial_value);
        }
        "unit" => {
            return Ok(PklValue::String(duration.unit.to_string()));
        }
        "isPositive" => return Ok(PklValue::Bool(!duration.is_negative)),
        "isoString" => return Ok(PklValue::String(duration.to_iso_string())),
        _ => {
            return Err((
                format!("Duration does not possess {} property", property),
                range,
            ))
        }
    }
}

/// Based on v0.26.0
pub fn match_duration_methods_api(
    duration: Duration,
    property: &str,
    args: Vec<PklValue>,
    range: Range<usize>,
) -> PklResult<PklValue> {
    match property {
        "isBetween" => {
            generate_method!(
                "isBetween", &args;
                0: Duration, 1: Duration;
                |(start, inclusive_end): (Duration, Duration)| {
                    Ok((duration >= start && duration <= inclusive_end).into())
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
                        let mut x = duration;
                        x.to_unit(unit);
                        return Ok((x).into())
                    }

                    Err((format!("'{unit}' is not a valid Duration Unit"), range))
                };
                range
            )
        }
        _ => {
            return Err((
                format!("Duration does not possess {} method", property),
                range,
            ))
        }
    }
}

/// An enum representing duration units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    NS,
    US,
    MS,
    S,
    MIN,
    H,
    D,
}

impl Unit {
    /// Parses a string slice into an `Option<Unit>`.
    /// Returns `None` if the string does not correspond to a known data size unit.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "ns" => Some(Unit::NS),
            "us" => Some(Unit::US),
            "ms" => Some(Unit::MS),
            "s" => Some(Unit::S),
            "min" => Some(Unit::MIN),
            "h" => Some(Unit::H),
            "d" => Some(Unit::D),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Duration {
    pub duration: StdDuration,
    pub unit: Unit,
    pub is_negative: bool,
    initial_value: Box<PklValue>,
    initial_unit: Unit,
}

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.is_negative && !other.is_negative {
            return Some(std::cmp::Ordering::Less);
        }
        if !self.is_negative && other.is_negative {
            return Some(std::cmp::Ordering::Greater);
        }

        if self.is_negative && other.is_negative {
            if self.duration > other.duration {
                Some(std::cmp::Ordering::Less)
            } else if self.duration < other.duration {
                Some(std::cmp::Ordering::Greater)
            } else {
                Some(std::cmp::Ordering::Equal)
            }
        } else {
            // both positive
            if self.duration > other.duration {
                Some(std::cmp::Ordering::Greater)
            } else if self.duration < other.duration {
                Some(std::cmp::Ordering::Less)
            } else {
                Some(std::cmp::Ordering::Equal)
            }
        }
    }
}

impl PartialEq for Duration {
    fn eq(&self, other: &Self) -> bool {
        self.duration == other.duration && self.is_negative == other.is_negative
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Duration {
    pub fn from_float_and_unit(value: f64, unit: Unit) -> Self {
        let initial_value = Box::new(PklValue::Float(value));
        let is_negative = value.is_sign_negative();
        let value = if is_negative { value.abs() } else { value };

        let duration = match unit {
            Unit::NS => StdDuration::from_secs_f64(value * 10e-9),
            Unit::US => StdDuration::from_secs_f64(value * 10e-6),
            Unit::MS => StdDuration::from_secs_f64(value * 10e-3),
            Unit::S => StdDuration::from_secs_f64(value),
            Unit::MIN => StdDuration::from_secs_f64(value * 60.0),
            Unit::H => StdDuration::from_secs_f64(value * 60.0 * 60.0),
            Unit::D => StdDuration::from_secs_f64(value * 60.0 * 60.0 * 24.0),
        };

        Self {
            duration,
            unit,
            initial_unit: unit,
            initial_value,
            is_negative,
        }
    }

    pub fn to_iso_string(&self) -> String {
        let seconds = self.duration.as_secs();
        let nanos = self.duration.subsec_nanos();

        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;
        let millis = nanos / 1_000_000; // Convert nanoseconds to milliseconds

        let mut iso_string = String::from("");

        if self.is_negative {
            iso_string.push('-');
        }

        iso_string.push_str("PT");

        if hours > 0 {
            iso_string.push_str(&format!("{}H", hours));
        }
        if minutes > 0 {
            iso_string.push_str(&format!("{}M", minutes));
        }
        if secs > 0 || millis > 0 {
            if millis > 0 {
                iso_string.push_str(&format!("{}.{}S", secs, format!("{:03}", millis)));
            } else {
                iso_string.push_str(&format!("{}S", secs));
            }
        } else if iso_string == "PT" || iso_string == "-PT" {
            // If there are no hours, minutes, or seconds, and the string is still "PT"
            iso_string.push_str("0S"); // Handle the edge case where the duration is zero
        }

        iso_string
    }

    pub fn from_int_and_unit(value: i64, unit: Unit) -> Self {
        let initial_value = Box::new(PklValue::Int(value));
        let is_negative = value < 0;
        let value = if is_negative {
            (value as f64).abs()
        } else {
            value as f64
        };

        let duration = match unit {
            Unit::NS => StdDuration::from_secs_f64(value * 10e-9),
            Unit::US => StdDuration::from_secs_f64(value * 10e-6),
            Unit::MS => StdDuration::from_secs_f64(value * 10e-3),
            Unit::S => StdDuration::from_secs_f64(value),
            Unit::MIN => StdDuration::from_secs_f64(value * 60.0),
            Unit::H => StdDuration::from_secs_f64(value * 60.0 * 60.0),
            Unit::D => StdDuration::from_secs_f64(value * 60.0 * 60.0 * 24.0),
        };

        Self {
            duration,
            unit,
            initial_unit: unit,
            initial_value,
            is_negative,
        }
    }

    pub fn to_unit(&mut self, unit: Unit) -> &mut Self {
        self.unit = unit;
        self
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unit_str = match self {
            Unit::NS => "ns",
            Unit::US => "us",
            Unit::MS => "ms",
            Unit::S => "s",
            Unit::MIN => "min",
            Unit::H => "h",
            Unit::D => "d",
        };
        write!(f, "{}", unit_str)
    }
}
