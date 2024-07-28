use super::PklExpr;
use crate::parser::expr::object::parse_object;
use crate::parser::value::AstPklValue;
use crate::parser::Identifier;
use crate::PklResult;
use crate::{lexer::PklToken, parser::utils::parse_multispaces_until};
use hashbrown::HashMap;
use logos::{Lexer, Span};

#[derive(Debug, PartialEq, Clone)]
pub struct ClassInstance<'a>(
    pub Option<Identifier<'a>>,
    pub (HashMap<&'a str, PklExpr<'a>>, Span),
    pub Span,
);

fn parse_id_or_open_brace<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklToken<'a>> {
    parse_multispaces_until!(
        lexer,
        PklToken::Identifier(_),
        PklToken::IllegalIdentifier(_),
        PklToken::OpenBrace
    )
}

/// Function called after 'new' keyword is found.
pub fn parse_class_instance<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklExpr<'a>> {
    let start = lexer.span().start;

    let class_name = match parse_id_or_open_brace(lexer)? {
        PklToken::OpenBrace => None,
        PklToken::Identifier(id) | PklToken::IllegalIdentifier(id) => {
            Some(Identifier(id, lexer.span()))
        }
        _ => unreachable!(),
    };

    loop {
        match lexer.next() {
            Some(Ok(PklToken::OpenBrace)) => {
                return Ok(AstPklValue::ClassInstance(ClassInstance(
                    class_name,
                    parse_object(lexer)?,
                    start..lexer.span().end,
                ))
                .into());
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
