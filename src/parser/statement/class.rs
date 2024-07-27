use super::constant::parse_const_expr;
use super::PklStatement;
use crate::lexer::PklToken;
use crate::PklResult;
use logos::Lexer;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClassType {
    Open,
    Classical,
}

#[derive(Debug, Clone, Eq, Hash)]
pub struct ClassField<'a> {
    pub name: &'a str,
    pub kind: FieldKind,
}

impl<'a> ClassField<'a> {
    pub fn new(name: &'a str, kind: FieldKind) -> Self {
        Self { name, kind }
    }
}

impl<'a> PartialEq for ClassField<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldKind {
    Hidden,
    Classical,
}

/// Parse a token stream into a Pkl class Statement.
pub fn parse_class_declaration<'a>(
    lexer: &mut Lexer<'a, PklToken<'a>>,
    name: &'a str,
) -> PklResult<PklStatement<'a>> {
    let start = lexer.span().start;
    let value = parse_const_expr(lexer)?;
    let end = lexer.span().end;

    Ok(PklStatement::Class {
        name,
        _type: todo!(),
        extends: todo!(),
        fields: todo!(),
        span: todo!(),
    })
}
