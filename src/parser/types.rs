#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub enum PklType<'a> {
    Any,
    Unknown,
    Nothing,

    String,
    Boolean,

    Int,
    UInt16,
    Float,
    Number,

    Duration,
    DataSize,
    Null,

    Collection(Box<PklType<'a>>),
    Listing(Box<PklType<'a>>),
    List(Box<PklType<'a>>),

    Pair(Box<PklType<'a>>, Box<PklType<'a>>),
    Map(Box<PklType<'a>>, Box<PklType<'a>>),
    Mapping(Box<PklType<'a>>, Box<PklType<'a>>),

    Set(Box<PklType<'a>>),

    Class(&'a str),
}
