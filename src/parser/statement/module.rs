use crate::parser::statement::PklStatement;
use crate::parser::utils::parse_id_as_str;
use crate::{lexer::PklToken, PklResult};
use logos::{Lexer, Span};

#[derive(Debug, Clone, PartialEq)]
pub struct Module<'a> {
    pub full_name: &'a str,
    pub span: Span,
}

impl<'a> Module<'a> {
    pub fn new(full_name: &'a str, span: Span) -> Self {
        Self { full_name, span }
    }

    pub fn last_name_component(&self) -> &str {
        self.full_name.split('.').last().unwrap()
    }
}

/// Function called after 'import' keyword.
pub fn parse_module_clause<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklStatement<'a>> {
    let name = parse_id_as_str(lexer)?;

    Ok(PklStatement::ModuleClause(Module {
        full_name: name,
        span: lexer.span(),
    }))
}
