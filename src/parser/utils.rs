use super::{
    errors::{
        locating::{generate_source, get_error_location},
        UnexpectedError,
    },
    ParsingError, ParsingResult, PklLexer,
};
use pkl_fast::lexer::{LexingError, PklToken};

pub fn jump_spaces_and_then<'source, Output, F>(
    lexer: &mut PklLexer<'source>,
    predicate: F,
) -> ParsingResult<Output>
where
    F: Fn(Option<Result<PklToken, LexingError>>, &mut PklLexer<'source>) -> ParsingResult<Output>,
{
    loop {
        if let Some(token) = lexer.next() {
            if let Ok(PklToken::Space) = token {
                continue;
            }

            return predicate(Some(token), lexer);
        } else {
            return predicate(None, lexer);
        };
    }
}
pub fn jump_spaces_with_peek<'source>(lexer: &mut PklLexer<'source>) -> () {
    loop {
        let mut peekable = lexer.peekable();
        if let Some(token) = peekable.peek() {
            if let Ok(PklToken::Space) = token {
                lexer.next();
                continue;
            }

            break;
        } else {
            break;
        }
    }
}

pub fn parse_object_name<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<&'source str> {
    let start_index: usize = lexer.span().start;
    let mut _end_index: Option<usize> = None;
    // i came up with the indexes approach to avoid uneccessary string creation
    // if you have any better approach please share it with me

    loop {
        let mut peekable = lexer.peekable();

        let peeked = peekable.peek();
        if let Some(token) = peeked {
            match token {
                Ok(PklToken::Identifier) => {
                    lexer.next();
                }
                Ok(PklToken::Dot) => {
                    lexer.next();
                }
                _ => {
                    _end_index = Some(lexer.span().start);
                    break;
                }
            }
        } else {
            break;
        }
    }

    let source = lexer.source();
    if (&source[start_index..start_index + 1]) == "." {
        return Err(ParsingError::UnexpectedToken(UnexpectedError {
            src: generate_source("main.pkl", lexer.source()),
            at: get_error_location(lexer).into(),
        }));
    }

    Ok(&source[start_index.._end_index.unwrap()])
}
