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

impl<'a> From<&'a str> for PklType<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "Any" => PklType::Any,
            "unknown" => PklType::Unknown,
            "nothing" => PklType::Nothing,
            "String" => PklType::String,
            "Boolean" => PklType::Boolean,
            "Int" => PklType::Int,
            "UInt16" => PklType::UInt16,
            "Float" => PklType::Float,
            "Number" => PklType::Number,
            "Duration" => PklType::Duration,
            "DataSize" => PklType::DataSize,
            "Null" => PklType::Null,
            "Collection" => PklType::Collection(Box::new(PklType::Unknown)), // Adjust as needed
            "Listing" => PklType::Listing(Box::new(PklType::Unknown)),       // Adjust as needed
            "List" => PklType::List(Box::new(PklType::Unknown)),             // Adjust as needed
            "Pair" => PklType::Pair(Box::new(PklType::Unknown), Box::new(PklType::Unknown)), // Adjust as needed
            "Map" => PklType::Map(Box::new(PklType::Unknown), Box::new(PklType::Unknown)), // Adjust as needed
            "Mapping" => PklType::Mapping(Box::new(PklType::Unknown), Box::new(PklType::Unknown)), // Adjust as needed
            "Set" => PklType::Set(Box::new(PklType::Unknown)), // Adjust as needed
            _ => PklType::Class(value),
        }
    }
}
