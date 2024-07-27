use super::{member_expr::parse_member_expr_member, PklExpr};
use crate::{
    lexer::PklToken,
    parser::{expr::class::parse_class_instance, value::AstPklValue, Identifier},
    PklResult,
};
use logos::{Lexer, Span};

#[derive(Debug, PartialEq, Clone)]
pub struct FuncCall<'a>(pub Identifier<'a>, pub Vec<PklExpr<'a>>, pub Span);

impl<'a> FuncCall<'a> {
    pub fn span(&self) -> Span {
        self.2.to_owned()
    }
}

pub fn parse_fn_call<'a>(
    lexer: &mut Lexer<'a, PklToken<'a>>,
    id: Identifier<'a>,
) -> PklResult<FuncCall<'a>> {
    let start = lexer.span().start;
    let mut values: Vec<PklExpr> = Vec::with_capacity(5);
    let mut is_comma = true;

    loop {
        match lexer.next() {
            Some(Ok(token)) => match token {
                PklToken::Dot if !is_comma => {
                    if let Some(last) = values.last_mut() {
                        let expr_member = parse_member_expr_member(lexer)?;
                        let expr_start = last.span().start;
                        let expr_end = expr_member.span().end;

                        *last = PklExpr::MemberExpression(
                            Box::new(last.clone()),
                            expr_member,
                            expr_start..expr_end,
                        );
                    } else {
                        return Err(("unexpected token '.'".to_owned(), lexer.span()));
                    }
                }
                PklToken::Comma if !is_comma => {
                    is_comma = true;
                }
                PklToken::CloseParen => {
                    let end = lexer.span().end;
                    return Ok(FuncCall(id, values.into(), start..end));
                }
                PklToken::Space
                | PklToken::NewLine
                | PklToken::DocComment(_)
                | PklToken::LineComment(_)
                | PklToken::MultilineComment(_) => {}
                PklToken::Bool(b) if is_comma => {
                    values.push(AstPklValue::Bool(b, lexer.span()).into());
                    is_comma = false;
                }
                PklToken::Identifier(id) | PklToken::IllegalIdentifier(id) if is_comma => {
                    values.push(PklExpr::Identifier(Identifier(id, lexer.span())));
                    is_comma = false;
                }
                PklToken::New if is_comma => {
                    values.push(parse_class_instance(lexer)?);
                    is_comma = false;
                }
                PklToken::FunctionCall(fn_name) if is_comma => {
                    values.push(PklExpr::FuncCall(parse_fn_call(
                        lexer,
                        Identifier(fn_name, lexer.span()),
                    )?));

                    is_comma = false;
                }
                PklToken::Int(i)
                | PklToken::OctalInt(i)
                | PklToken::HexInt(i)
                | PklToken::BinaryInt(i)
                    if is_comma =>
                {
                    values.push(AstPklValue::Int(i, lexer.span()).into());
                    is_comma = false;
                }
                PklToken::Float(f) if is_comma => {
                    values.push(AstPklValue::Float(f, lexer.span()).into());
                    is_comma = false;
                }
                PklToken::String(s) if is_comma => {
                    values.push(AstPklValue::String(s, lexer.span()).into());
                    is_comma = false;
                }
                PklToken::MultiLineString(s) if is_comma => {
                    values.push(AstPklValue::MultiLineString(s, lexer.span()).into());
                    is_comma = false;
                }
                _ => return Err(("unexpected token here".to_owned(), lexer.span())),
            },
            Some(Err(e)) => return Err((e.to_string(), lexer.span())),
            None => return Err(("Missing list close parenthesis".to_owned(), lexer.span())),
        }
    }
}
