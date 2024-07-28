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
    Union(Box<PklType<'a>>, Box<PklType<'a>>),
    Nullable(Box<PklType<'a>>),

    WithAttributes {
        name: &'a str,
        attributes: Vec<PklType<'a>>,
        span: Span,
    },

    WithRequirement {
        base_type: Box<PklType<'a>>,
        requirements: Box<PklExpr<'a>>,
        span: Span,
    },
}

impl<'a> PklType<'a> {
    pub fn span(&self) -> Span {
        match self {
            PklType::Basic(_, s) => s.to_owned(),
            PklType::StringLiteral(_, s) => s.to_owned(),
            PklType::Union(s1, s2) => s1.span().start..s2.span().end,
            PklType::Nullable(s) => s.span().to_owned(),
            PklType::WithAttributes { span, .. } => span.to_owned(),
            PklType::WithRequirement { span, .. } => span.to_owned(),
        }
    }
    pub fn is_last_with_attributes(&self) -> bool {
        match self {
            PklType::Basic(_, _) => false,
            PklType::StringLiteral(_, _) => false,
            PklType::Union(_, x) => x.is_last_with_attributes(),
            PklType::Nullable(_) => false,
            PklType::WithAttributes { .. } => true,
            PklType::WithRequirement { .. } => false,
        }
    }
}

pub fn parse_type<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklType<'a>> {
    while let Some(token) = lexer.next() {
        match token {
            Ok(PklToken::Identifier(id)) | Ok(PklToken::IllegalIdentifier(id)) => {
                return Ok(PklType::Basic(id, lexer.span()))
            }
            Ok(PklToken::TypeWithAttributes(fn_name)) => {
                let base_span = lexer.span();
                let start = base_span.start;

                let attributes = parse_attributes(lexer)?;
                let span = start..lexer.span().end;

                let _type = PklType::WithAttributes {
                    name: fn_name,
                    attributes,
                    span,
                };

                return Ok(_type);
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

pub fn parse_type_until<'a>(
    lexer: &mut Lexer<'a, PklToken<'a>>,
    until_token: PklToken<'a>,
) -> PklResult<PklType<'a>> {
    let mut _type = parse_type(lexer)?;

    while let Some(token) = lexer.next() {
        match token {
            Ok(token) if token == until_token => {
                break;
            }

            Ok(PklToken::QuestionMark) => {
                _type = PklType::Nullable(Box::new(_type));
            }
            Ok(PklToken::Union) => {
                let other_type = parse_type(lexer)?;
                _type = PklType::Union(Box::new(_type), Box::new(other_type));
            }
            Ok(PklToken::OpenParen) if _type.is_last_with_attributes() => {
                let base_span = _type.span();
                let start = base_span.start;

                let base_type = Box::new(_type);
                let base_expr = parse_expr(lexer)?;

                let requirements = Box::new(parse_long_expression_or(
                    lexer,
                    base_expr,
                    PklToken::CloseParen,
                )?);

                let span = start..lexer.span().end;

                _type = PklType::WithRequirement {
                    base_type,
                    requirements,
                    span,
                };
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

    Ok(_type)
}

fn parse_attributes<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<Vec<PklType<'a>>> {
    let mut result = Vec::new();
    let mut expect_type = true;

    loop {
        if expect_type {
            result.push(parse_type(lexer)?);
            expect_type = false;
        }

        match lexer.next() {
            Some(t)
                if matches!(
                    t,
                    Ok(PklToken::Space)
                        | Ok(PklToken::DocComment(_))
                        | Ok(PklToken::LineComment(_))
                        | Ok(PklToken::MultilineComment(_))
                        | Ok(PklToken::NewLine)
                ) =>
            {
                continue;
            }

            Some(Ok(PklToken::Comma)) => {
                expect_type = true;
                continue;
            }
            Some(Ok(PklToken::OperatorMoreThan)) => {
                break;
            }
            Some(Err(e)) => return Err((format!("Lexer error: {:?}", e), lexer.span())),
            None => {
                return Err((
                    "Unexpected end of input, did you mean to write ',' or '>'?".to_string(),
                    lexer.span(),
                ));
            }
            token => {
                return Err((
                    format!(
                        "Unexpected token '{token:?}' found, did you mean to write ',' or '>' ?"
                    ),
                    lexer.span(),
                ))
            }
        }
    }

    Ok(result)
}
