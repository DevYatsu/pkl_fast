use super::PklExpr;
use crate::{
    lexer::PklToken,
    parser::{statement::constant::parse_const_expr, value::AstPklValue, ExprHash},
    PklResult,
};
use hashbrown::HashMap;
use logos::Lexer;

pub fn parse_object<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<ExprHash<'a>> {
    let start = lexer.span().start;
    let mut hashmap = HashMap::with_capacity(8); // Assuming typical small object size
    let mut expect_new_entry = true;

    while let Some(token) = lexer.next() {
        match token {
            Ok(PklToken::Identifier(id)) | Ok(PklToken::IllegalIdentifier(id)) => {
                if !expect_new_entry {
                    return Err((
                        "unexpected token here (context: object), expected newline or comma"
                            .to_owned(),
                        lexer.span(),
                    ));
                }

                let value = parse_const_expr(lexer)?;
                expect_new_entry = matches!(value, PklExpr::Value(AstPklValue::Object((_, _))));
                hashmap.insert(id, value);
            }
            Ok(PklToken::NewLine) | Ok(PklToken::Comma) => {
                expect_new_entry = true;
            }
            Ok(PklToken::Space) => {}
            Ok(PklToken::CloseBrace) => {
                let end = lexer.span().end;
                return Ok((hashmap, start..end));
            }
            Err(e) => return Err((e.to_string(), lexer.span())),
            _ => {
                return Err((
                    "unexpected token here (context: object)".to_owned(),
                    lexer.span(),
                ));
            }
        }
    }

    Err(("Missing object close brace".to_owned(), lexer.span()))
}

pub fn parse_amended_object<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<AstPklValue<'a>> {
    let start = lexer.span().start;

    let amended_object_name = match lexer.next() {
        Some(Ok(PklToken::Identifier(id))) | Some(Ok(PklToken::IllegalIdentifier(id))) => {
            if let Some(Ok(PklToken::CloseParen)) = lexer.next() {
                id
            } else {
                return Err((
                    "expected close parenthesis (context: amended_object)".to_owned(),
                    lexer.span(),
                ));
            }
        }
        Some(Err(e)) => return Err((e.to_string(), lexer.span())),
        _ => {
            return Err((
                "expected identifier here (context: amended_object)".to_owned(),
                lexer.span(),
            ));
        }
    };

    while let Some(token) = lexer.next() {
        match token {
            Ok(PklToken::Space) | Ok(PklToken::NewLine) => continue,
            Ok(PklToken::OpenBrace) => {
                let object = parse_object(lexer)?;
                let end = lexer.span().end;
                return Ok(AstPklValue::AmendingObject(
                    amended_object_name,
                    object,
                    start..end,
                ));
            }
            Err(e) => return Err((e.to_string(), lexer.span())),
            _ => {
                return Err((
                    "expected open brace here (context: amended_object)".to_owned(),
                    lexer.span(),
                ));
            }
        }
    }

    Err((
        "expected open brace (context: amended_object)".to_owned(),
        lexer.span(),
    ))
}
