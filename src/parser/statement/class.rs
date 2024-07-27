use super::PklExpr;
use crate::lexer::PklToken;
use crate::parser::expr::object::parse_object;
use crate::parser::value::AstPklValue;
use crate::PklResult;
use logos::Lexer;

pub struct ClassField<'a> {
    pub name: &'a str,
    pub field_type: &'a str,
}

pub fn parse_class_instance<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklExpr<'a>> {
    let start = lexer.span().start;

    let class_name = loop {
        match lexer.next() {
            Some(Ok(PklToken::Identifier(id))) | Some(Ok(PklToken::IllegalIdentifier(id))) => {
                break id
            }
            Some(Ok(PklToken::Space))
            | Some(Ok(PklToken::NewLine))
            | Some(Ok(PklToken::DocComment(_)))
            | Some(Ok(PklToken::LineComment(_)))
            | Some(Ok(PklToken::MultilineComment(_))) => continue,
            Some(Err(e)) => return Err((e.to_string(), lexer.span())),
            Some(_) => {
                return Err((
                    "unexpected token here (context: class_instance), expected identifier"
                        .to_owned(),
                    lexer.span(),
                ));
            }
            None => return Err(("Expected identifier".to_owned(), lexer.span())),
        }
    };

    loop {
        match lexer.next() {
            Some(Ok(PklToken::OpenBrace)) => {
                return Ok(AstPklValue::ClassInstance(
                    class_name,
                    parse_object(lexer)?,
                    start..lexer.span().end,
                )
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
