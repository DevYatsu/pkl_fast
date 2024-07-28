use crate::parser::statement::PklStatement;
use crate::parser::utils::parse_simple_string;
use crate::{lexer::PklToken, PklResult};
use logos::Lexer;

/// Function called after 'import' keyword.
pub fn parse_import<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklStatement<'a>> {
    let start = lexer.span().start;

    let name = parse_simple_string(lexer)?;

    return Ok(PklStatement::Import {
        name,
        local_name: None,
        span: start..lexer.span().end,
    });
}
