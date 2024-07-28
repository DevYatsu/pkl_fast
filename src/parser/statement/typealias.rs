use crate::parser::statement::PklStatement;
use crate::parser::types::parse_type_until;
use crate::parser::utils::{parse_equal, parse_id};
use crate::parser::Identifier;
use crate::{lexer::PklToken, PklResult};
use logos::Lexer;

/// Function called after 'typealias' keyword.
pub fn parse_typealias<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklStatement<'a>> {
    let start = lexer.span().start;
    let (name, attributes) = parse_typealias_name(lexer)?;

    parse_equal(lexer)?;

    let refering_type = parse_type_until(lexer, PklToken::NewLine)?;

    let span = start..lexer.span().end;
    return Ok(PklStatement::TypeAlias {
        name,
        attributes,
        refering_type,
        span,
    });
}

pub fn parse_typealias_name<'a>(
    lexer: &mut Lexer<'a, PklToken<'a>>,
) -> PklResult<(Identifier<'a>, Vec<Identifier<'a>>)> {
    while let Some(token) = lexer.next() {
        match token {
            Ok(PklToken::Identifier(id)) | Ok(PklToken::IllegalIdentifier(id)) => {
                return Ok((Identifier(id, lexer.span()), vec![]))
            }
            Ok(PklToken::TypeWithAttributes(fn_name)) => {
                let start = lexer.span().start;
                let end = start + fn_name.len();
                let attributes = parse_attributes(lexer)?;
                return Ok((Identifier(fn_name, start..end), attributes));
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

    Err(("empty typealiases not allowed".to_owned(), lexer.span()))
}

/// Parses a typealias attributes
fn parse_attributes<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<Vec<Identifier<'a>>> {
    let mut result = Vec::new();
    let mut expect_type = true;

    loop {
        if expect_type {
            let id = parse_id(lexer)?;
            result.push(id);
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
