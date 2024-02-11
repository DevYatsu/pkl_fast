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

    #[token("hidden")]
    Hidden,

    #[token("function")]
    Function,
    #[token("->")]
    ArrowOperator,

    #[token("throw")]
    Throw,
    #[token("trace")]
    Trace,

    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("let")]
    Let,
    #[token("as")]
    As,
    #[token("for")]
    For,
    #[token("in")]
    In,

    /// support for:
    /// - ==, <=, >=, >
    /// - !, !!, ?, ??
    /// - &&, ||, |
    #[regex(r#"==|<=|<|>=|>|!=|!!|!|\?\?|\?|\&\&|\&|\|\||\||"#)]
    Operators,
    #[token("=")]
    EqualSign,
    #[regex(r#"\+|-|\*|\*\*|\|>|%|~/"#)]
    ArithmeticOperation,

    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token(";")]
    SemiColon,
    #[token("{")]
    OpenBracket,
    #[token("}")]
    CloseBracket,

    #[token("[")]
    OpenBrace,
    #[token("]")]
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
    #[token("Number")]
    NumberType,
    #[token("NonNull")]
    NonNullType,
    #[token("UInt16")]
    UInt16Type,
    #[token("Duration")]
    DurationType,
    #[token("DataSize")]
    DataSizeType,
    #[regex(r"Listing<\w+>")]
    ListingType,

    #[token("Infinity")]
    Infinity,
    #[token("-Infinity")]
    NegativeInfinity,
    #[token("NaN")]
    NotANumber,

    #[token("null")]
    Null,

    #[regex(r"true|false", |lex| lex.slice() == "true")]
    Boolean(bool),
    #[regex(r"-?(\d(?:_?\d)*|0x[0-9a-fA-F]+|0b[01]+|0o[0-7]+)", |lex| lex.slice().parse(), priority = 3)]
    Integer(i32),
    #[regex(r"-?(\d*\.\d+(e\d+)?)", |lex| lex.slice().parse(), priority = 4)]
    Float(f64),

    #[regex(r#""(?:\\.|[^\\"])*\(\s*\w+\s*\)(?:\\.|[^\\"])*""#)]
    InterpolatedString,
    #[token("\\(")]
    EscapeOpenParenthesis,
    #[token("\\)")]
    EscapeCloseParenthesis,
    #[regex(r#""[^"]*""#)]
    StringLiteral,

    #[regex(r#"ns|us|ms|s|min|h|d"#, priority = 3)]
    MinIndication,

    #[regex(r#"b|kb|mb|gb|tb|pb|kib|mib|gib|tib|pib"#, priority = 3)]
    DataSizeIndication,

    // #[regex("[A-Z][a-zA-Z]*")]
    // PascalCaseValue,
    // #[regex("[a-z][a-zA-Z]*")]
    // CamelCaseValue, // in pkl words written in camelCase are meant to be used as values
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[regex("//.*")]
    LineComment,
    #[regex("///.*")]
    DocComment,
    #[regex(r"/\*([^*]|\*[^/])*\*/")]
    BlockComment,
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
