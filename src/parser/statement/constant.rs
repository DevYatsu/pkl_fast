use super::{PklExpr, PklStatement};
use crate::lexer::PklToken;
use crate::parser::expr::object::parse_object;
use crate::parser::expr::parse_expr;
use crate::parser::types::{parse_type, PklType};
use crate::PklResult;
use logos::Lexer;

/// Parse a token stream into a Pkl const Statement.
pub fn parse_const<'a>(
    lexer: &mut Lexer<'a, PklToken<'a>>,
    name: &'a str,
) -> PklResult<PklStatement<'a>> {
    let start = lexer.span().start;
    let (_type, value) = parse_const_expr(lexer)?;
    let end = lexer.span().end;

    Ok(PklStatement::Constant {
        name,
        _type,
        value,
        span: start..end,
    })
}

/// Parse a token stream into a Pkl Expr after an identifier with a possible type.
pub fn parse_const_expr<'a>(
    lexer: &mut Lexer<'a, PklToken<'a>>,
) -> PklResult<(Option<PklType<'a>>, PklExpr<'a>)> {
    let mut _type = None;
    loop {
        match lexer.next() {
            Some(Ok(PklToken::EqualSign)) => {
                return Ok((_type, parse_expr(lexer)?));
            }
            Some(Ok(PklToken::Colon)) => {
                _type = Some(parse_type(lexer)?);
            }
            Some(Ok(PklToken::OpenBrace)) if _type.is_none() => {
                return Ok((_type, parse_object(lexer)?.into()));
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
                    "unexpected token here (context: constant)".to_owned(),
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
pub fn parse_const_expr_without_type<'a>(
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
                    "unexpected token here (context: constant)".to_owned(),
                    lexer.span(),
                ));
            }
            None => {
                return Err(("Expected '='".to_owned(), lexer.span()));
            }
        }
    }
}
