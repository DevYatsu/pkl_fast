#[derive(Debug, PartialEq, PartialOrd, Clone)]
/// A struct representating the Pkl DataSize value composed of a `Number` and a unit component of type `String`.
pub struct DataSize {
    pub value: DataSizeValue,
    pub unit: DataSizeUnit,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
/// DataSizeValue represents the value of the DataSize, which can be an i64 or an f64
pub enum DataSizeValue {
    Integer(i64),
    Float(f64),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone)]
/// DataSizeUnit represents the unit of the DataSize, restricted to these values: "b"|"kb"|"kib"|"mb"|"mib"|"gb"|"gib"|"tb"|"tib"|"pb"|"pib".
pub enum DataSizeUnit {
    // Data sizes with decimal unit (factor 1000)
    Bytes,
    Kylobytes,
    Megabytes,
    Gigabytes,
    Terabytes,
    Petabytes,

    // Data sizes with binary unit (factor 1024)
    KibiBytes,
    MebiBytes,
    GibiBytes,
    Tebibytes,
    Pebibytes,
}

/// DataSizeUnit represents the unit of the DataSize, restricted to these values: "b"|"kb"|"kib"|"mb"|"mib"|"gb"|"gib"|"tb"|"tib"|"pb"|"pib".
impl From<&str> for DataSizeUnit {
    fn from(value: &str) -> Self {
        match value {
            "b" => DataSizeUnit::Bytes,

            "kib" => DataSizeUnit::KibiBytes,
            "mib" => DataSizeUnit::MebiBytes,
            "gib" => DataSizeUnit::GibiBytes,
            "tib" => DataSizeUnit::Tebibytes,
            "pib" => DataSizeUnit::Pebibytes,

            "kb" => DataSizeUnit::Kylobytes,
            "mb" => DataSizeUnit::Megabytes,
            "gb" => DataSizeUnit::Gigabytes,
            "tb" => DataSizeUnit::Terabytes,
            "pb" => DataSizeUnit::Petabytes,
            _ => unreachable!(),
        }
    }
}

impl From<f64> for DataSizeValue {
    fn from(value: f64) -> Self {
        DataSizeValue::Float(value)
    }
}
impl From<i64> for DataSizeValue {
    fn from(value: i64) -> Self {
        DataSizeValue::Integer(value)
    }
}

use std::fmt;

impl fmt::Display for DataSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value {
            DataSizeValue::Integer(i) => write!(f, "{}", i)?,
            DataSizeValue::Float(float) => write!(f, "{}", float)?,
        };

        write!(f, ".")?;

        match self.unit {
            DataSizeUnit::Bytes => write!(f, "b")?,
            DataSizeUnit::Kylobytes => write!(f, "kb")?,
            DataSizeUnit::Megabytes => write!(f, "mb")?,
            DataSizeUnit::Gigabytes => write!(f, "gb")?,
            DataSizeUnit::Terabytes => write!(f, "tb")?,
            DataSizeUnit::Petabytes => write!(f, "pb")?,
            DataSizeUnit::KibiBytes => write!(f, "kib")?,
            DataSizeUnit::MebiBytes => write!(f, "mib")?,
            DataSizeUnit::GibiBytes => write!(f, "gib")?,
            DataSizeUnit::Tebibytes => write!(f, "tib")?,
            DataSizeUnit::Pebibytes => write!(f, "pib")?,
        };

        Ok(())
    }
}
