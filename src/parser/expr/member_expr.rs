use super::fn_call::{parse_fn_call, FuncCall};
use crate::{lexer::PklToken, parser::Identifier, PklResult};
use logos::Lexer;
use std::ops::Range;

#[derive(Debug, PartialEq, Clone)]
pub enum ExprMember<'a> {
    Identifier(Identifier<'a>),
    FuncCall(FuncCall<'a>),
}

impl<'a> ExprMember<'a> {
    pub fn span(&self) -> Range<usize> {
        match self {
            ExprMember::Identifier(id) => id.span(),
            ExprMember::FuncCall(fn_call) => fn_call.span(),
        }
    }
}

impl<'a> From<Identifier<'a>> for ExprMember<'a> {
    fn from(value: Identifier<'a>) -> Self {
        ExprMember::Identifier(value)
    }
}

pub fn parse_member_expr_member<'a>(
    lexer: &mut Lexer<'a, PklToken<'a>>,
) -> PklResult<ExprMember<'a>> {
    let start = lexer.span().end;

    while let Some(token) = lexer.next() {
        match token {
            Ok(PklToken::Identifier(id)) | Ok(PklToken::IllegalIdentifier(id)) => {
                return Ok(Identifier(id, start..lexer.span().end).into())
            }
            Ok(PklToken::FunctionCall(id)) => {
                return Ok(ExprMember::FuncCall(parse_fn_call(
                    lexer,
                    Identifier(id, lexer.span()),
                )?))
            }
            Ok(PklToken::NewLine) | Ok(PklToken::Space) => {
                // Skip spaces and newlines
            }
            Err(e) => {
                return Err((e.to_string(), lexer.span()));
            }
            _ => {
                return Err((
                    "unexpected token, expected identifier".to_owned(),
                    lexer.span(),
                ));
            }
        }
    }

    Err((
        "expected identifier but got nothing".to_owned(),
        lexer.span(),
    ))
}
