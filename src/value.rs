use std::collections::HashMap;

pub use self::datasize::DataSize;
pub use self::duration::Duration;
mod datasize;
mod duration;

#[derive(Debug, PartialEq, Clone)]
/// An enum representing any Pkl value
pub enum PklValue<'a> {
    String(&'a str),
    Boolean(bool),
    Int(i64),
    Float(f64),
    Object(HashMap<&'a str, PklValue<'a>>),
    
    List(Vec<PklValue<'a>>),
    Listing(Vec<PklValue<'a>>),

    Map(Vec<PklValue<'a>>),

    /// For now, only indexing with &str is supported.
    /// In the future we shall support other any data type as key!
    Mapping(HashMap<&'a str, PklValue<'a>>),

    Duration(Duration),
    DataSize(DataSize),
    Null
}
