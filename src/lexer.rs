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
    // if necessary add Tab
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
    #[token("default")]
    Default,

    #[token("local")]
    Local,
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
    /// - ??
    /// - &&, ||, |
    /// - +,-,*,/
    /// - **, %,|>, ~/
    #[regex(r#"==|<=|<|>=|>|!=|\?\?|\&\&|\&|\|\||\||/|\+|-|\*|\*\*|\|>|%|~/"#, |lex| lex.slice())]
    Operator(&'source str),
    #[token("=")]
    EqualSign,

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

    #[regex(r#"\([a-zA-Z_][a-zA-Z0-9_]*\)\s*\{"#, |lex| {let val = lex.slice(); &val[1..val.find(')').unwrap()]})]
    /// Token representing an object definition with the object amending another object, that is for example: ```(object_name) {```
    AmendedObjectBracket(&'source str),

    #[token("typealias")]
    TypeAlias,

    /// This variant shall represent types with a given generic such as `Listing<Bird>` or `Map<Int, String>`  
    // We assume a type starts with an UpperCase
    #[regex(
        r"[A-Z][A-Za-z0-9]*<\s*([A-Za-z_][A-Za-z0-9_]*)\s*(?:,\s*([A-Za-z_][A-Za-z0-9_]*))?\s*>"
    )]
    GenericTypeAnnotation,

    /// It's simply `GenericTypeAnnotation` variant followed by `(`.
    /// It's supposed to represent a generic type with restrictions.
    #[regex(
        r"[A-Z][A-Za-z0-9]*<\s*([A-Za-z_][A-Za-z0-9_]*)\s*(?:,\s*([A-Za-z_][A-Za-z0-9_]*))?\s*>\("
    )]
    GenericTypeAnnotationFunctionCall,

    // We assume a type starts with an UpperCase
    #[regex(r"[A-Z][a-zA-Z0-9_]*\?", |lex| {let val=lex.slice(); &val[..val.len()-1]})]
    PotentiallyNullType(&'source str),

    /// This variant represents a identifier followed by '!!', meaning that the variable cannot be null, otherwise throwing an error
    // We assume a type starts with an UpperCase
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*(\.[a-zA-Z_][a-zA-Z0-9_]*)*!!", |lex| {let val=lex.slice(); &val[0..val.len()-2]})]
    NonNullIdentifier(&'source str),

    /// This variant represents '!', the logical NOT operator
    #[regex(r"!")]
    LogicalNotOperator,

    /// This variant represents a type preceded by '*', meaning that the type is the default one in an union.
    #[regex(r"\*[A-Z][a-zA-Z0-9_]*(\.[a-zA-Z_][a-zA-Z0-9_]*)*", |lex| {let val=lex.slice(); &val[1..]})]
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
        let val = lex.slice();

        // Remove underscores from the string
        let clean_value = val.replace("_", "");
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
    #[regex(r"(-?((\d*\.\d+((e|E)-?\d+)?)|(Infinity)))|NaN", |lex| lex.slice().parse(), priority = 4)]
    Float(f64),

    #[regex(r#"`[a-zA-Z_][a-zA-Z0-9_]*`"#, |lex| lex.slice())]
    IllegalIdentifier(&'source str),

    // we retrieve the string like this and we pass it through a lexing fn to obtain a Vec<StringFragment>
    #[regex(r#""[^"]*""#, |lex| {let val = lex.slice(); &val[1..val.len()-1]})]
    StringLiteral(&'source str),

    /// This variant needs to be changed fast
    #[regex(r#""""[^"]*""""#, |lex| {let val = lex.slice(); &val[3..val.len()-3]})]
    MultipleLinesString(&'source str),

    // #[regex("[A-Z][a-zA-Z]*")]
    // PascalCaseValue,
    // #[regex("[a-z][a-zA-Z]*")]
    // CamelCaseValue, // in pkl words written in camelCase are meant to be used as values
    /// Matches a simple identifier (ex: `foo`) as well as an object accessor (`foo.bar`).
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*(\.[a-zA-Z_][a-zA-Z0-9_]*)*", |lex| lex.slice())]
    Identifier(&'source str),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*(\.[a-zA-Z_][a-zA-Z0-9_]*)*\(", |lex| {let val = lex.slice(); &val[..val.len()-1]})]
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

impl<'source> std::fmt::Display for PklToken<'source> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // THIS SHOULD ONLY BE USED IN ORDER TO PRINT ERRORS
        match self {
            PklToken::NewLine => write!(f, "line end"),
            PklToken::OpenBracket => write!(f, "'{{'"),
            PklToken::CloseBracket => write!(f, "'}}'"),
            PklToken::OpenBrace => write!(f, "'['"),
            PklToken::CloseBrace => write!(f, "']'"),
            PklToken::OpenParenthesis => write!(f, "'('"),
            PklToken::CloseParenthesis => write!(f, "')'"),
            PklToken::Module => write!(f, "'module'"),
            PklToken::ModuleInfo => write!(f, "@ModuleInfo"),
            PklToken::DeprecatedInstruction => write!(f, "@Deprecated"),
            PklToken::GlobbedImport => write!(f, "import*"),
            PklToken::Import => write!(f, "import"),
            PklToken::Extends => write!(f, "extends"),
            PklToken::Amends => write!(f, "amends"),
            PklToken::Abstract => write!(f, "abstract"),
            PklToken::Open => write!(f, "open"),
            PklToken::New => write!(f, "new"),
            PklToken::Class => write!(f, "class"),
            PklToken::Default => write!(f, "default"),
            PklToken::Local => write!(f, "local"),
            PklToken::Hidden => write!(f, "hidden"),
            PklToken::Fixed => write!(f, "fixed"),
            PklToken::Function => write!(f, "function"),
            PklToken::ArrowOperator => write!(f, "->"),
            PklToken::If => write!(f, "if"),
            PklToken::Else => write!(f, "else"),
            PklToken::Let => write!(f, "let"),
            PklToken::As => write!(f, "as"),
            PklToken::For => write!(f, "for"),
            PklToken::In => write!(f, "in"),
            PklToken::When => write!(f, "when"),
            PklToken::Is => write!(f, "is"),
            PklToken::Operator(op) => write!(f, "'{}'", op),
            PklToken::EqualSign => write!(f, "'='"),
            PklToken::Colon => write!(f, "':'"),
            PklToken::Comma => write!(f, "','"),
            PklToken::SpreadSyntax => write!(f, "'...'"),
            PklToken::Dot => write!(f, "'.'"),
            PklToken::SemiColon => write!(f, "';'"),
            PklToken::AmendedObjectBracket(s) => write!(f, "({{}} {} {{)", s),
            PklToken::TypeAlias => write!(f, "typealias"),
            PklToken::GenericTypeAnnotation | PklToken::GenericTypeAnnotationFunctionCall => {
                write!(f, "'GenericType Annotation: Type<Annotation>'")
            }
            PklToken::PotentiallyNullType(s) => write!(f, "{}?", s),
            PklToken::NonNullIdentifier(s) => write!(f, "{}!!", s),
            PklToken::LogicalNotOperator => write!(f, "'!'"),
            PklToken::DefaultUnionType(s) => write!(f, "*{}", s),
            PklToken::Null => write!(f, "null"),
            PklToken::Boolean(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            PklToken::Duration => write!(f, "Duration"),
            PklToken::DataSize => write!(f, "DataSize"),
            PklToken::Integer(i) => write!(f, "{}", i),
            PklToken::Float(fl) => write!(f, "{}", fl),
            PklToken::IllegalIdentifier(s) => write!(f, "`{}`", s),
            PklToken::StringLiteral(s) => write!(f, "\"{}\"", s),
            PklToken::MultipleLinesString(s) => write!(f, "\"\"\"{}\"\"\"", s),
            PklToken::Identifier(s) => write!(f, "{}", s),
            PklToken::FunctionCall(s) => write!(f, "{}(", s),
            PklToken::LineComment => write!(f, "//"),
            PklToken::DocComment => write!(f, "///"),
            PklToken::BlockComment => write!(f, "/* ... */"),
        }
    }
}
