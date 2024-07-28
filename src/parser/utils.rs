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

pub fn parse_equal<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklToken<'a>> {
    parse_multispaces_until!(lexer, PklToken::EqualSign)
}

fn id_token<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklToken<'a>> {
    parse_multispaces_until!(lexer, PklToken::Identifier(_))
}
pub fn parse_id<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<Identifier<'a>> {
    match id_token(lexer)? {
        PklToken::Identifier(id) => return Ok(Identifier(id, lexer.span())),
        _ => unreachable!(),
    }
}
