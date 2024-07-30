use super::{PklExpr, PklStatement};
use crate::lexer::PklToken;
use crate::parser::expr::object::parse_object;
use crate::parser::expr::parse_expr;
use crate::parser::types::{parse_type_until, AstPklType};
use crate::parser::Identifier;
use crate::PklResult;
use logos::{Lexer, Span};

#[derive(Debug, Clone, PartialEq)]
pub struct Property<'a> {
    pub name: Identifier<'a>,
    pub kind: PropertyKind,
    pub _type: Option<AstPklType<'a>>,
    pub value: PklExpr<'a>,
    pub span: Span,
}
#[derive(Debug, Clone, PartialEq, Default)]
pub enum PropertyKind {
    #[default]
    Classical,
    Local,
    Fixed,
    Const,
}

/// Parse a token stream into a Pkl const Statement.
pub fn parse_property<'a>(
    lexer: &mut Lexer<'a, PklToken<'a>>,
    name: Identifier<'a>,
    kind: PropertyKind,
) -> PklResult<PklStatement<'a>> {
    let start = name.1.start;
    let (_type, value) = parse_property_expr(lexer)?;
    let end = lexer.span().end;

    Ok(PklStatement::Property(Property {
        name,
        kind,
        _type,
        value,
        span: start..end,
    }))
}

/// Parse a token stream into a Pkl Expr after an identifier with a possible type.
pub fn parse_property_expr<'a>(
    lexer: &mut Lexer<'a, PklToken<'a>>,
) -> PklResult<(Option<AstPklType<'a>>, PklExpr<'a>)> {
    loop {
        match lexer.next() {
            Some(Ok(PklToken::EqualSign)) => {
                return Ok((None, parse_expr(lexer)?));
            }
            Some(Ok(PklToken::Colon)) => {
                let _type = parse_type_until(lexer, PklToken::EqualSign)?;
                return Ok((Some(_type), parse_expr(lexer)?));
            }
            Some(Ok(PklToken::OpenBrace)) => {
                return Ok((None, parse_object(lexer)?.into()));
            }
            Some(Ok(PklToken::Space))
            | Some(Ok(PklToken::NewLine))
            | Some(Ok(PklToken::DocComment(_)))
            | Some(Ok(PklToken::LineComment(_)))
            | Some(Ok(PklToken::MultilineComment(_))) => {
                // Continue the loop to process the next token
                continue;
            }
            Some(Err(e)) => {
                return Err((e.to_string(), lexer.span()));
            }
            Some(_) => {
                return Err((
                    "unexpected token here (context: property)".to_owned(),
                    lexer.span(),
                ));
            }
            None => {
                return Err(("Expected '='".to_owned(), lexer.span()));
            }
        }
    }
}

/// Parse a token stream into a Pkl Expr after an identifier.
pub fn parse_property_expr_without_type<'a>(
    lexer: &mut Lexer<'a, PklToken<'a>>,
) -> PklResult<PklExpr<'a>> {
    loop {
        match lexer.next() {
            Some(Ok(PklToken::EqualSign)) => {
                return Ok(parse_expr(lexer)?);
            }
            Some(Ok(PklToken::OpenBrace)) => {
                return Ok(parse_object(lexer)?.into());
            }
            Some(Ok(PklToken::Space))
            | Some(Ok(PklToken::NewLine))
            | Some(Ok(PklToken::DocComment(_)))
            | Some(Ok(PklToken::LineComment(_)))
            | Some(Ok(PklToken::MultilineComment(_))) => {
                // Continue the loop to process the next token
                continue;
            }
            Some(Err(e)) => {
                return Err((e.to_string(), lexer.span()));
            }
            Some(_) => {
                return Err((
                    "unexpected token here (context: property)".to_owned(),
                    lexer.span(),
                ));
            }
            None => {
                return Err(("Expected '='".to_owned(), lexer.span()));
            }
        }
    }
}
