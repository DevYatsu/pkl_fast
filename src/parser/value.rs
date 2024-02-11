#[derive(Debug)]
pub enum PklValue<'a> {
    String(&'a str),
    Int(usize),
    Float(f64),
    Bool(bool),
    // Comment(&'a str),
}
