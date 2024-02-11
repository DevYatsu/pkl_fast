use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(error = LexingError)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("module")]
    Module,
    #[token("@ModuleInfo")]
    ModuleInfo,

    #[token("import")]
    Import,
    #[token("extends")]
    Extends,
    #[token("amends")]
    Amends,

    #[token("open")]
    Open,
    #[token("new")]
    New,
    #[token("class")]
    Class,

    #[token("function")]
    Function,

    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("let")]
    Let,
    #[token("as")]
    As,

    #[token("=")]
    EqualSign,
    #[token("+")]
    PlusSign,
    #[token("-")]
    MinusSign,
    #[token("|>")]
    PipeOperator,

    #[token(":")]
    SemiColon,
    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,
    #[token("(")]
    OpenParenthesis,
    #[token(")")]
    CloseParenthesis,

    #[token("typealias")]
    TypeAlias,

    #[token("String")]
    StringType,
    #[token("Boolean")]
    BooleanType,
    #[token("Int")]
    IntType,
    #[token("UInt16")]
    UInt16Type,
    #[regex(r"Listing<\w+>")]
    ListingType,

    #[token("List")]
    List,
    #[regex(r"true|false", |lex| lex.slice() == "true")]
    Boolean(bool),
    #[regex(r#"\d+"#, |lex| lex.slice().parse())]
    Integer(i32),
    #[regex(r#"\d+\.\d+"#, |lex| lex.slice().parse())]
    Float(f64),

    #[regex(r#""(?:\\.|[^\\"])*\(\s*\w+\s*\)(?:\\.|[^\\"])*""#)]
    InterpolatedString,
    #[token("\\(")]
    EscapeOpenParenthesis,
    #[token("\\)")]
    EscapeCloseParenthesis,
    #[regex(r#""[^"]*""#)]
    StringLiteral,

    // #[regex("[A-Z][a-zA-Z]*")]
    // PascalCaseValue,
    // #[regex("[a-z][a-zA-Z]*")]
    // CamelCaseValue, // in pkl words written in camelCase are meant to be used as values
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[regex("//.*")]
    SingleLineComment,
    #[regex("///.*")]
    TripleSlashComment,
    #[regex(r"/\*([^*]|\*[^/])*\*/")]
    MultiLineComment,
}

use std::num::{ParseFloatError, ParseIntError};

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
    InvalidInteger(String),
    InvalidFloat(String),

    #[default]
    NonAsciiCharacter,
}

impl From<ParseIntError> for LexingError {
    fn from(err: ParseIntError) -> Self {
        use std::num::IntErrorKind::*;
        match err.kind() {
            PosOverflow | NegOverflow => LexingError::InvalidInteger("overflow error".to_owned()),
            _ => LexingError::InvalidInteger("other error".to_owned()),
        }
    }
}

impl From<ParseFloatError> for LexingError {
    fn from(_err: ParseFloatError) -> Self {
        LexingError::InvalidFloat("invalidFloat".to_owned())
    }
}
