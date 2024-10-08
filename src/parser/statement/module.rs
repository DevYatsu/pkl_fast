use crate::parser::statement::PklStatement;
use crate::parser::utils::parse_id;
use crate::parser::Identifier;
use crate::{lexer::PklToken, PklResult};
use logos::{Lexer, Span};

#[derive(Debug, Clone, PartialEq)]
pub struct Module<'a> {
    pub full_name: Identifier<'a>,
    pub span: Span,
    pub is_open: bool,
}

impl<'a> Module<'a> {
    pub fn last_name_component(&self) -> &str {
        &self.full_name.0.split('.').last().unwrap()
    }

    pub fn not_allowed_here_err(&self) -> String {
        String::from("Keyword `module` is not allowed here. (If you must use this name as identifier, enclose it in backticks.)")
    }
}

/// Function called after 'import' keyword.
pub fn parse_module_clause<'a>(
    lexer: &mut Lexer<'a, PklToken<'a>>,
    is_open: bool,
) -> PklResult<PklStatement<'a>> {
    let start = lexer.span().start;
    let name = parse_id(lexer)?;

    Ok(PklStatement::ModuleClause(Module {
        full_name: name,
        span: start..lexer.span().end,
        is_open,
    }))
}
