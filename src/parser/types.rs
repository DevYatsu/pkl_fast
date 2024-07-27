use super::{
    expr::{parse_expr, PklExpr},
    PklResult,
};
use crate::{lexer::PklToken, parser::expr::long::parse_long_expression_or};
use logos::{Lexer, Span};

#[derive(Debug, Clone, PartialEq)]
/// Representation of a Pkl Type
pub enum PklType<'a> {
    Basic(&'a str, Span),
    StringLiteral(&'a str, Span),
    Or(Box<PklType<'a>>, Box<PklType<'a>>),

    ListType {
        name: &'a str,
        attributes: Vec<&'a str>,
        span: Span,
    },

    WithRequirement {
        base_type: Box<PklType<'a>>,
        requirements: Box<PklExpr<'a>>,
        span: Span,
    },
}

pub fn parse_type<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklType<'a>> {
    while let Some(token) = lexer.next() {
        match token {
            Ok(PklToken::Identifier(id)) | Ok(PklToken::IllegalIdentifier(id)) => {
                return Ok(PklType::Basic(id, lexer.span()))
            }
            Ok(PklToken::FunctionCall(fn_name)) => {
                let base_span = lexer.span();
                let start = base_span.start;

                let base_type = Box::new(PklType::Basic(fn_name, base_span));

                let base_expr = parse_expr(lexer)?;

                let requirements = Box::new(parse_long_expression_or(
                    lexer,
                    base_expr,
                    PklToken::CloseParen,
                )?);

                let span = start..lexer.span().end;

                return Ok(PklType::WithRequirement {
                    base_type,
                    requirements,
                    span,
                });
            }
            Ok(PklToken::String(s)) | Ok(PklToken::MultiLineString(s)) => {
                return Ok(PklType::StringLiteral(s, lexer.span()))
            }
            Ok(PklToken::Space)
            | Ok(PklToken::NewLine)
            | Ok(PklToken::DocComment(_))
            | Ok(PklToken::LineComment(_))
            | Ok(PklToken::MultilineComment(_)) => continue,
            Err(e) => return Err((e.to_string(), lexer.span())),
            _ => return Err(("unexpected token here".to_owned(), lexer.span())),
        }
    }

    Err(("empty types are not allowed".to_owned(), lexer.span()))
}
