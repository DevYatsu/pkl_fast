#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub enum PklType<'a> {
    String, 
    Boolean,

    Int, 
    UInt16,
    Float,
    Number,

    Duration,
    DataSize,
    Null,

    Listing,
    Map,

    Class(&'a str)
}   
