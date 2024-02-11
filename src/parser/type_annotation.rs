#[derive(Debug)]
pub enum PklType<'a> {
    String,
    UInt16,
    Int,
    Boolean,
    Listing(Box<PklType<'a>>),
    Class(&'a str),
}
