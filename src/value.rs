use std::collections::HashMap;

pub use self::datasize::DataSize;
pub use self::duration::Duration;
mod datasize;
mod duration;

#[derive(Debug, PartialEq, Clone)]
pub enum PklValue<'a> {
    String(&'a str),
    Boolean(bool),
    Integer(i64),
    Float(f64),
    Object(HashMap<&'a str, PklValue<'a>>),
    Array(Vec<PklValue<'a>>),
    Duration(Duration),
    DataSize(DataSize),
}
