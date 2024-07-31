use crate::parser::statement::PklStatement;
use crate::parser::utils::parse_simple_string;
use crate::{lexer::PklToken, PklResult};
use logos::{Lexer, Span};

#[derive(Debug, Clone, PartialEq)]
pub struct Import<'a> {
    pub name: &'a str,
    pub local_name: Option<&'a str>,
    pub span: Span,
}

impl<'a> Import<'a> {
    pub fn not_allowed_here_err(&self) -> String {
        String::from("Keyword `import` is not allowed here. (If you must use this name as identifier, enclose it in backticks.)")
    }
}

/// Function called after 'import' keyword.
pub fn parse_import<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklStatement<'a>> {
    let start = lexer.span().start;

    let name = parse_simple_string(lexer)?;

    Ok(PklStatement::Import(Import {
        name,
        local_name: None,
        span: start..lexer.span().end,
    }))
}
