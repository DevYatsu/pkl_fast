use super::{Identifier, PklResult};
use crate::lexer::PklToken;
use logos::Lexer;

/// Macro to parse tokens from a lexer until one of the specified tokens is found.
///
/// # Arguments
///
/// * `$lexer` - The lexer instance from which tokens are parsed.
/// * `$($until_token:pat),+` - One or more patterns representing the tokens at which parsing should stop.
///
/// # Returns
///
/// * `Ok(PklToken<'a>)` - The first token that matches one of the specified patterns.
/// * `Err(ParseError)` - An error if an unexpected token is encountered or if the end of input is reached.
macro_rules! parse_multispaces_until {
    ($lexer:expr, $($until_token:pat),+ $(,)?) => {{
        let lexer: &mut Lexer<'_, PklToken<'_>> = $lexer;

        while let Some(token) = lexer.next() {
            match token {
                Ok(token) if $(matches!(token, $until_token))||+ => {
                    let token: PklToken<'_> = token;
                    return Ok(token);
                }
                Ok(PklToken::Space)
                | Ok(PklToken::DocComment(_))
                | Ok(PklToken::LineComment(_))
                | Ok(PklToken::MultilineComment(_))
                | Ok(PklToken::NewLine) => {
                    continue;
                }
                Err(e) => return Err((e.to_string(), lexer.span())),
                _ => return Err(("unexpected token here".to_owned(), lexer.span())),
            }
        }

        Err(("Unexpected end of input".to_owned(), lexer.span()))
    }};
}

pub(super) use parse_multispaces_until;

pub fn parse_equal<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklToken<'a>> {
    parse_multispaces_until!(lexer, PklToken::EqualSign)
}
pub fn parse_open_brace<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklToken<'a>> {
    parse_multispaces_until!(lexer, PklToken::OpenBrace)
}

fn id_token<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklToken<'a>> {
    parse_multispaces_until!(
        lexer,
        PklToken::Identifier(_) | PklToken::IllegalIdentifier(_)
    )
}
pub fn parse_id<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<Identifier<'a>> {
    match id_token(lexer)? {
        PklToken::Identifier(id) | PklToken::IllegalIdentifier(id) => {
            return Ok(Identifier(id, lexer.span()))
        }
        _ => unreachable!(),
    }
}
pub fn parse_id_as_str<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<&'a str> {
    match id_token(lexer)? {
        PklToken::Identifier(id) | PklToken::IllegalIdentifier(id) => return Ok(id),
        _ => unreachable!(),
    }
}

fn any_string_token<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklToken<'a>> {
    parse_multispaces_until!(lexer, PklToken::String(_), PklToken::MultiLineString(_))
}
pub fn parse_any_string<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<&'a str> {
    match any_string_token(lexer)? {
        PklToken::String(s) | PklToken::MultiLineString(s) => return Ok(s),
        _ => unreachable!(),
    }
}

fn simple_string_token<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklToken<'a>> {
    parse_multispaces_until!(lexer, PklToken::String(_))
}
pub fn parse_simple_string<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<&'a str> {
    match simple_string_token(lexer)? {
        PklToken::String(s) | PklToken::MultiLineString(s) => return Ok(s),
        _ => unreachable!(),
    }
}
