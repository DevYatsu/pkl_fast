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
pub enum PklToken<'source> {
    #[token("\n")]
    NewLine,
    #[token("\t")]
    Tab,

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
    #[token("fixed")]
    Fixed,

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
    #[token("is")]
    Is,

    /// support for:
    /// - ==, <=, >=, >
    /// - !, !!, ?, ??
    /// - &&, ||, |
    #[regex(r#"==|<=|<|>=|>|!=|!!|!|\?\?|\?|\&\&|\&|\|\||\||"#, |lex| lex.slice())]
    Operator(&'source str),
    #[token("=")]
    EqualSign,
    #[regex(r#"\+|-|\*|\*\*|\|>|%|~/"#, |lex| lex.slice())]
    ArithmeticOperation(&'source str),

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

    #[regex(r#"\([a-zA-Z_][a-zA-Z0-9_]*\)\s*\{"#, |lex| {let raw_value = lex.slice(); &raw_value[1..raw_value.find(')').unwrap()]})]
    /// Token representing an object definition with the object amending another object, that is for example: ```rust (object_name) {```
    AmendedObjectBracket(&'source str),

    #[token("typealias")]
    TypeAlias,

    /// This variant shall represent types with a given generic such as `Listing<Bird>` or `Map<Int, String>`  
    #[regex(
        r"[A-Za-z][A-Za-z0-9]*<\s*([A-Za-z_][A-Za-z0-9_]*)\s*(?:,\s*([A-Za-z_][A-Za-z0-9_]*))?\s*>"
    )]
    GenericTypeAnnotation,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*(\.[a-zA-Z_][a-zA-Z0-9_]*)*\?", |lex| {let raw_value=lex.slice(); &raw_value[..raw_value.len()-1]})]
    PotentiallyNullType(&'source str),
    #[regex(r"\*[a-zA-Z_][a-zA-Z0-9_]*(\.[a-zA-Z_][a-zA-Z0-9_]*)*", |lex| {let raw_value=lex.slice(); &raw_value[1..]})]
    DefaultUnionType(&'source str),

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

    /// Integer variant includes basic integers as well as binary, hexadecimal and octal numbers
    /// For example, these values are valid integers:
    /// - 123, -123_000
    /// - 0x012AFF, 0x0_12_.AFF
    /// - 0b00010111, 0b000_101_11
    /// - 0o755, 0o75_5
    #[regex(r"-?(\d(?:_?\d)*)|0[xX]([0-9a-fA-F](?:_?[0-9a-fA-F])*)|0b([01](?:_?[01])*)|0o([0-7](?:_?[0-7])*)", |lex| {
        let raw_value = lex.slice();

        // Remove underscores from the string
        let clean_value = raw_value.replace("_", "");
        // Check if the value starts with a radix specifier
        let parsed_value = if clean_value.starts_with("0x") {
            // Parse hexadecimal value
            i64::from_str_radix(&clean_value[2..], 16)
        } else if clean_value.starts_with("0b") {
            // Parse binary value
            i64::from_str_radix(&clean_value[2..], 2)
        } else if clean_value.starts_with("0o") {
            // Parse octal value
            i64::from_str_radix(&clean_value[2..], 8)
        } else {
            // Parse decimal value
            clean_value.parse::<i64>()
        };
        parsed_value
    }, priority = 3)]
    Integer(i64),

    /// Float variant includes float number with optional decimal part and/or exponent
    /// Infinity, -Infinity and NaN are also valid floats
    /// For example, these values are valid floats:
    /// - .23, -123.23
    /// - .5e-2,  2.12e9
    /// - Infinity, -Infinity
    /// - Nan
    #[regex(r"(-?((\d*\.\d+(e-?\d+)?)|(Infinity)))|NaN", |lex| lex.slice().parse(), priority = 4)]
    Float(f64),

    #[regex(r#"`[a-zA-Z_][a-zA-Z0-9_]*`"#, |lex| lex.slice())]
    IllegalIdentifier(&'source str),

    #[regex(r#""[^"]*""#, |lex| {let raw_value = lex.slice(); &raw_value[1..raw_value.len()-1]})]
    StringLiteral(&'source str),

    // #[regex("[A-Z][a-zA-Z]*")]
    // PascalCaseValue,
    // #[regex("[a-z][a-zA-Z]*")]
    // CamelCaseValue, // in pkl words written in camelCase are meant to be used as values
    /// Matches a simple identifier (ex: `foo`) as well as an object accessor (`foo.bar`).
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*(\.[a-zA-Z_][a-zA-Z0-9_]*)*", |lex| lex.slice())]
    Identifier(&'source str),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*(\.[a-zA-Z_][a-zA-Z0-9_]*)*\(", |lex| {let raw_value = lex.slice(); &raw_value[1..raw_value.len()-1]})]
    FunctionCall(&'source str),

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

    #[error("UnknownError")]
    #[default]
    UnknownError,
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
