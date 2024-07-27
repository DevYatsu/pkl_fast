use crate::parse_string;
use crate::parser::statement::PklStatement;
use crate::{lexer::PklToken, PklResult};
use logos::Lexer;
use std::ops::Range;

/// Function called after 'import' keyword.
pub fn parse_import<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklStatement<'a>> {
    let start = lexer.span().start;

    fn parse_value<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<(&'a str, Range<usize>)> {
        parse_string!(
            lexer,
            "unexpected token here, expected an import value (context: import)",
            "Missing import value"
        )
    }

    let (value, rng) = parse_value(lexer)?;

    return Ok(PklStatement::Import(value, None, start..rng.end));
}
