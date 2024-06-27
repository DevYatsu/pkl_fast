use logos::Logos;

/* ANCHOR: tokens */
/// All meaningful Pkl tokens.
///
/// > NOTE: regexes for [`PklToken::Int`], [`PklToken::Float`], [`PklToken::MultiLineString`] and [`PklToken::String`]
/// > may not catch all possible values, especially for strings. If you find
/// > errors, please report them so that we can improve the regex.
///
/// > NOTE: Only basic Pkl is covered for the moment!
#[derive(Debug, PartialEq, PartialOrd, Logos, Clone)]
#[logos(error = LexingError)]
#[logos(skip r"[\t]+")]
pub enum PklToken<'a> {
    #[token("_", priority = 3)]
    BlankIdentifier,
    #[token(" ")]
    Space,
    #[token("\n")]
    NewLine,
    #[token("=")]
    EqualSign,
    #[token("true", |_| true)]
    #[token("false", |_| false)]
    Bool(bool),

    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token(",")]
    Comma,
    #[token("new")]
    New,
    #[token("import")]
    Import,
    #[token("as")]
    As,
    #[token(".")]
    Dot,
    #[token("null")]
    Null,

    #[regex(r"-?\d+(?:_?\d)*", |lex| {
        let raw = lex.slice();
        // Remove underscores for parsing
        let clean_raw: String = raw.chars().filter(|&c| c != '_').collect();
        clean_raw.parse::<i64>()
    }, priority = 3)]
    Int(i64),

    #[regex(r"-?0x[0-9a-fA-F]+(?:_?[0-9a-fA-F])*", |lex| {
        let raw = lex.slice();
        // Check for the optional minus sign
        let (is_negative, hex_str) = if raw.starts_with('-') {
            (true, &raw[3..]) // Skip "-0x"
        } else {
            (false, &raw[2..]) // Skip "0x"
        };

        // Remove underscores for parsing
        let clean_hex: String = hex_str.chars().filter(|&c| c != '_').collect();
        let value = i64::from_str_radix(&clean_hex, 16);

        if is_negative {
            value.map(|v| -v)
        } else {
            value
        }
    })]
    HexInt(i64),

    #[regex(r"-?0b[01]+(?:_?[01])*", |lex| {
        let raw = lex.slice();
        // Check for the optional minus sign
        let (is_negative, hex_str) = if raw.starts_with('-') {
            (true, &raw[3..]) // Skip "-0b"
        } else {
            (false, &raw[2..]) // Skip "0b"
        };

        // Remove underscores for parsing
        let clean_hex: String = hex_str.chars().filter(|&c| c != '_').collect();
        let value = i64::from_str_radix(&clean_hex, 2);

        if is_negative {
            value.map(|v| -v)
        } else {
            value
        }
    })]
    BinaryInt(i64),

    #[regex(r"-?0o[0-7]+(?:_?[0-7])*", |lex| {
        let raw = lex.slice();
        // Check for the optional minus sign
        let (is_negative, hex_str) = if raw.starts_with('-') {
            (true, &raw[3..]) // Skip "-0o"
        } else {
            (false, &raw[2..]) // Skip "0o"
        };

        // Remove underscores for parsing
        let clean_hex: String = hex_str.chars().filter(|&c| c != '_').collect();
        let value = i64::from_str_radix(&clean_hex, 8);

        if is_negative {
            value.map(|v| -v)
        } else {
            value
        }
    })]
    OctalInt(i64),

    #[token("NaN", |_| std::f64::NAN)]
    #[token("Infinity", |_| std::f64::INFINITY)]
    #[token("-Infinity", |_| std::f64::NEG_INFINITY)]
    #[regex(r"-?(?:0|[1-9]+(?:_?\d)*)?(?:\.\d+(?:_?\d)*)(?:[eE][+-]?\d+(?:_?\d)*)?", |lex| {
        let raw = lex.slice();
        let clean_raw: String = raw.chars().filter(|&c| c != '_').collect();
        clean_raw.parse::<f64>()
    }, priority = 2)]
    Float(f64),

    #[regex(r#"(_|\$)[a-zA-Z0-9_]+\("#, |lex| {let raw=lex.slice();&raw[..raw.len()-1]})]
    #[regex(r#"[a-zA-Z][a-zA-Z0-9_]*\("#, |lex| {let raw=lex.slice();&raw[..raw.len()-1]})]
    #[regex(r#"`([^`\\]|\\[`\\bnfrt]|\\u\{[a-fA-F0-9]+})*`\("#, |lex| {let raw=lex.slice();&raw[1..raw.len()-2]})]
    FunctionCall(&'a str),

    #[regex(r#"(_|\$)[a-zA-Z0-9_]+"#, |lex| lex.slice())]
    #[regex(r#"[a-zA-Z][a-zA-Z0-9_]*"#, |lex| lex.slice())]
    Identifier(&'a str),
    #[regex(r#"`([^`\\]|\\[`\\bnfrt]|\\u\{[a-fA-F0-9]+})*`"#, |lex| {let raw=lex.slice();&raw[1..raw.len()-1]})]
    IllegalIdentifier(&'a str),

    #[regex(r#"//[^\n\\]*"#, |lex| let raw=lex.slice();&raw[2..raw.len()-1])]
    LineComment(&'a str),
    #[regex(r#"///[^\n\\]*"#, |lex| let raw=lex.slice();&raw[3..raw.len()-1])]
    DocComment(&'a str),
    #[regex(r#"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/"#, |lex| let raw=lex.slice();&raw[2..raw.len()-2])]
    MultilineComment(&'a str),

    #[regex(r#""([^"\\]|\\["\\bnfrt]|\\u\{[a-fA-F0-9]+})*""#, |lex| let raw=lex.slice();&raw[1..raw.len()-1])]
    String(&'a str),

    // does pkl support <"> character in the multiline strings ?
    #[regex(r#""""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""""#, |lex| {
        let raw=lex.slice();

        if raw[3..=3] != *"\n" {
            return Err(LexingError::ExpectedNewLineAfterMultilineStringStart)
        }

        // return err if raw[raw.len()-4..=raw.len()-4] != "\n"
        if raw[raw.len()-4..=raw.len()-4] != *"\n" {
            return Err(LexingError::ExpectedNewLineBeforeMultilineStringEnd)
        }

        Ok(&raw[4..raw.len()-4])
    })]
    MultiLineString(&'a str),
}
/* ANCHOR_END: tokens */

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LexingError {
    InvalidInteger(String),
    InvalidFloat(String),

    ExpectedNewLineBeforeMultilineStringEnd,
    ExpectedNewLineAfterMultilineStringStart,
    #[default]
    DefaultLexingError,
}

use std::{
    fmt,
    num::{ParseFloatError, ParseIntError},
};

/// Error type returned by calling `lex.slice().parse()` to u8.
impl From<ParseIntError> for LexingError {
    fn from(err: ParseIntError) -> Self {
        LexingError::InvalidInteger(err.to_string())
    }
}
/// Error type returned by calling `lex.slice().parse()` to u8.
impl From<ParseFloatError> for LexingError {
    fn from(err: ParseFloatError) -> Self {
        LexingError::InvalidFloat(err.to_string())
    }
}

impl fmt::Display for LexingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexingError::InvalidInteger(s) => write!(f, "Invalid integer: {}", s),
            LexingError::InvalidFloat(s) => write!(f, "Invalid float: {}", s),
            LexingError::ExpectedNewLineBeforeMultilineStringEnd => {
                write!(
                    f,
                    "Expected a newline before the end of the multiline string"
                )
            }
            LexingError::ExpectedNewLineAfterMultilineStringStart => write!(
                f,
                "Expected a newline after the start of the multiline string"
            ),
            LexingError::DefaultLexingError => write!(f, "An unspecified lexing error occurred"),
        }
    }
}
