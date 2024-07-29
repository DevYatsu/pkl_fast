use crate::parser::statement::PklStatement;
use crate::parser::utils::parse_simple_string;
use crate::{lexer::PklToken, PklResult};
use logos::{Lexer, Span};

#[derive(Debug, Clone, PartialEq)]
pub struct Extends<'a> {
    pub name: &'a str,
    pub span: Span,
}

/// Function called after 'import' keyword.
pub fn parse_extends_clause<'a>(
    lexer: &mut Lexer<'a, PklToken<'a>>,
) -> PklResult<PklStatement<'a>> {
    let start = lexer.span().start;

    let name = parse_simple_string(lexer)?;

    Ok(PklStatement::ExtendsClause(Extends {
        name,
        span: start..lexer.span().end,
    }))
}
