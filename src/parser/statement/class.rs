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
    pub field_type: FieldType,
}

impl<'a> PartialEq for ClassField<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<'a> ClassField<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            field_type: FieldType::Classical,
        }
    }

    pub fn set_hidden(&mut self) {
        self.field_type = FieldType::Hidden
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldType {
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
