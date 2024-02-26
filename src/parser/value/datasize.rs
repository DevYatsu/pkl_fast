#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone)]
/// A struct representating the Pkl DataSize value composed of a `Number` and a unit component of type `String`.
pub struct DataSize {
    value: u64,
    unit: DataSizeUnit,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone)]
/// DataSizeUnit represents the unit of the DataSize, restricted to these values: "b"|"kb"|"kib"|"mb"|"mib"|"gb"|"gib"|"tb"|"tib"|"pb"|"pib".
pub enum DataSizeUnit {
    // Data sizes with decimal unit (factor 1000)
    Bytes,
    Kylobites,
    Megabits,
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
