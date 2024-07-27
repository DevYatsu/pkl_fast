use crate::lexer::PklToken;

use super::{
    statement::class::parse_class_instance, value::AstPklValue, ExprHash, Identifier, PklResult,
};
use fn_call::{parse_fn_call, FuncCall};
use logos::Lexer;
use member_expr::ExprMember;
use object::parse_amended_object;
use std::ops::Range;

pub mod fn_call;
pub mod member_expr;
pub mod object;

#[derive(Debug, PartialEq, Clone)]
pub enum PklExpr<'a> {
    Identifier(Identifier<'a>),
    Value(AstPklValue<'a>),
    MemberExpression(Box<PklExpr<'a>>, ExprMember<'a>, Range<usize>),
    FuncCall(FuncCall<'a>),
}

impl<'a> PklExpr<'a> {
    /// This function MUST be called only when we are sure `PklExpr` is a `AstPklValue`
    pub fn extract_value(self) -> AstPklValue<'a> {
        match self {
            Self::Value(v) => v,
            _ => unreachable!(),
        }
    }

    pub fn span(&self) -> Range<usize> {
        match self {
            Self::Value(v) => v.span(),
            Self::Identifier(Identifier(_, span)) => span.to_owned(),
            Self::MemberExpression(_, _, span) => span.to_owned(),
            Self::FuncCall(FuncCall(_, _, span)) => span.to_owned(),
        }
    }
}

pub fn parse_expr<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklExpr<'a>> {
    while let Some(token) = lexer.next() {
        match token {
            Ok(PklToken::Bool(b)) => return Ok(AstPklValue::Bool(b, lexer.span()).into()),
            Ok(PklToken::Identifier(id)) | Ok(PklToken::IllegalIdentifier(id)) => {
                return Ok(PklExpr::Identifier(Identifier(id, lexer.span())))
            }
            Ok(PklToken::New) => return parse_class_instance(lexer),
            Ok(PklToken::FunctionCall(fn_name)) => {
                let fn_call = parse_fn_call(lexer, Identifier(fn_name, lexer.span()))?;

                return Ok(PklExpr::FuncCall(fn_call));
            }
            Ok(PklToken::Null) => return Ok(AstPklValue::Null(lexer.span()).into()),
            Ok(PklToken::Int(i))
            | Ok(PklToken::OctalInt(i))
            | Ok(PklToken::HexInt(i))
            | Ok(PklToken::BinaryInt(i)) => return Ok(AstPklValue::Int(i, lexer.span()).into()),
            Ok(PklToken::Float(f)) => return Ok(AstPklValue::Float(f, lexer.span()).into()),
            Ok(PklToken::String(s)) => return Ok(AstPklValue::String(s, lexer.span()).into()),
            Ok(PklToken::MultiLineString(s)) => {
                return Ok(AstPklValue::MultiLineString(s, lexer.span()).into())
            }
            Ok(PklToken::OpenParen) => return Ok(parse_amended_object(lexer)?.into()),
            Ok(PklToken::Space)
            | Ok(PklToken::NewLine)
            | Ok(PklToken::DocComment(_))
            | Ok(PklToken::LineComment(_))
            | Ok(PklToken::MultilineComment(_)) => continue,
            Err(e) => return Err((e.to_string(), lexer.span())),
            _ => return Err(("unexpected token here".to_owned(), lexer.span())),
        }
    }
    Err(("empty expressions are not allowed".to_owned(), lexer.span()))
}

impl<'a> From<AstPklValue<'a>> for PklExpr<'a> {
    fn from(value: AstPklValue<'a>) -> Self {
        PklExpr::Value(value)
    }
}
impl<'a> From<(&'a str, Range<usize>)> for PklExpr<'a> {
    fn from((value, indexes): (&'a str, Range<usize>)) -> Self {
        PklExpr::Identifier(Identifier(value, indexes))
    }
}
impl<'a> From<ExprHash<'a>> for PklExpr<'a> {
    fn from(value: ExprHash<'a>) -> Self {
        PklExpr::Value(value.into())
    }
}
