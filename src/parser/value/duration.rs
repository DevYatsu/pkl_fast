#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone)]
/// A struct representating the Pkl Duration value composed of a `Number` and a unit component of type `String`.
pub struct Duration {
    pub value: u64,
    pub unit: DurationUnit,
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
