use logos::Logos;
use miette::Diagnostic;
use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;

/**
PklToken enum possesses a `lexer` method that lexes an input into tokens constituting the Pkl syntax
*/
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = LexingError)]
#[logos(skip r"[\f\ ]+")]
pub enum PklToken {
    #[token("\n")]
    NewLine,
    #[token("\t")]
    Tab,

    #[token("module")]
    Module,
    #[token("@ModuleInfo")]
    ModuleInfo,

    #[token("@Deprecated")]
    DeprecatedInstruction,

    #[token("import*")]
    GlobbedImport,
    #[token("import")]
    Import,
    #[token("extends")]
    Extends,
    #[token("amends")]
    Amends,

    #[token("abstract")]
    Abstract,
    #[token("open")]
    Open,
    #[token("new")]
    New,
    #[token("class")]
    Class,
    #[token("local")]
    Local,
    #[token("this")]
    This,
    #[token("default")]
    Default,

    #[token("hidden")]
    Hidden,

    #[token("function")]
    Function,
    #[token("->")]
    ArrowOperator,

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
    #[token("when")]
    When,

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
    #[token("...", priority = 3)]
    SpreadSyntax,
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

    #[regex(r#"-?\d+(\.\d+)?\.(ns|us|ms|s|min|h|d)"#, priority = 4)]
    Duration,
    #[regex(
        r#"-?\d+(\.\d+)?\.(b|kb|mb|gb|tb|pb|kib|mib|gib|tib|pib)"#,
        priority = 4
    )]
    DataSize,

    #[regex(r"-?(\d(?:_?\d)*|0x[0-9a-fA-F]+|0b[01]+|0o[0-7]+)", priority = 3)]
    Integer,
    #[regex(r"-?(\d*\.\d+(e\d+)?)", priority = 4)]
    Float,

    #[regex(r#""(?:\\.|[^\\"])*\(\s*\w+\s*\)(?:\\.|[^\\"])*""#)]
    InterpolatedString,
    #[token("\\(")]
    EscapeOpenParenthesis,
    #[token("\\)")]
    EscapeCloseParenthesis,
    #[regex(r#"`[^"]*`"#)]
    IllegalIdentifier,
    #[regex(r#""[^"]*""#)]
    StringLiteral,

    // #[regex("[A-Z][a-zA-Z]*")]
    // PascalCaseValue,
    // #[regex("[a-z][a-zA-Z]*")]
    // CamelCaseValue, // in pkl words written in camelCase are meant to be used as values
    ///Matches a simple identifier (ex: `foo`) as well as an object accessor (`foo.bar`).
    #[regex(r"([a-zA-Z_][a-zA-Z0-9_]*)(\.([a-zA-Z_][a-zA-Z0-9_]*))*")]
    Identifier,

    #[regex("//.*")]
    LineComment,
    #[regex("///.*")]
    DocComment,
    #[regex(r"/\*([^*]|\*[^/])*\*/")]
    BlockComment,
}

#[derive(Default, Debug, Clone, PartialEq, Error, Diagnostic)]
#[diagnostic(code(pkl_fast::lexing_error), help("try removing a character"))]
#[error("Lexing Error: Unexpected Token")]
/**
LexingError enum. Used by lexing operations in the protocol. Is only returned through [`ParsingError`](crate::parsing::ParsingError)
*/
pub enum LexingError {
    #[error("Invalid Integer")]
    InvalidInteger,

    #[error("Invalid Float")]
    InvalidFloat,

    #[error("Not a valid ASCII Character")]
    #[default]
    NonAsciiCharacter,
}

impl From<ParseIntError> for LexingError {
    fn from(_err: ParseIntError) -> Self {
        LexingError::InvalidInteger
    }
}

impl From<ParseFloatError> for LexingError {
    fn from(_err: ParseFloatError) -> Self {
        LexingError::InvalidFloat
    }
}
