use std::{collections::HashMap, hash::Hash};

use crate::prelude::{ParsingError, ParsingResult, PklToken};
mod identifier;
mod string;

pub fn retrieve_next_token<'source>(
    lexer: &mut PklLexer<'source>,
) -> ParsingResult<PklToken<'source>> {
    let token = lexer.next();

    if token.is_none() {
        return Err(ParsingError::eof(lexer));
    }

    match token.unwrap() {
        Err(e) => Err(ParsingError::lexing(lexer, e)),
        token => Ok(token?),
    }
}

pub fn retrieve_opt_next_token<'source>(
    lexer: &mut PklLexer<'source>,
) -> ParsingResult<Option<PklToken<'source>>> {
    let token = lexer.next();

    match token {
        Some(Err(e)) => Err(ParsingError::lexing(lexer, e)),
        Some(token) => Ok(Some(token?)),
        None => Ok(None),
    }
}

pub fn expect_token<'source>(
    lexer: &mut PklLexer<'source>,
    target_token: PklToken<'source>,
) -> ParsingResult<()> {
    let token = lexer.next();

    if token.is_none() {
        return Err(ParsingError::eof(lexer));
    }

    match token.unwrap() {
        Err(e) => Err(ParsingError::lexing(lexer, e))?,
        Ok(token) if token == target_token => Ok(()),
        _ => Err(ParsingError::unexpected(lexer, target_token.to_string()))?,
    }
}

pub fn assert_token_eq<'source>(
    lexer: &mut PklLexer<'source>,
    token_option: Option<PklToken<'source>>,
    expected_token: PklToken<'source>,
) -> ParsingResult<()> {
    if token_option.is_none() {
        return Err(ParsingError::eof(lexer));
    }

    match token_option.unwrap() {
        token if token == expected_token => Ok(()),
        _ => Err(ParsingError::unexpected(lexer, expected_token.to_string()))?,
    }
}

pub fn expect_statement_end<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<()> {
    match lexer.next() {
        Some(Err(e)) => Err(ParsingError::lexing(lexer, e)),
        Some(Ok(token))
            if token == PklToken::NewLine
                || token == PklToken::LineComment
                || token == PklToken::BlockComment =>
        {
            Ok(())
        }
        Some(_) => Err(ParsingError::unexpected(lexer, "line ending".to_string())),
        None => Ok(()),
    }
}

/// This function creates a list out of a `predicate` that will be ran until one of the `end_token` is encountered.
/// The `separator_token` will be skipped after each time the `predicate` is ran.
///
/// *NOTE*: Contrary to `list_while_not_token1` function, this function does not skip the `separator_token` before running the predicate for the first time
pub fn list_while_not_token0<'source, R, F>(
    lexer: &mut PklLexer<'source>,
    separator_token: PklToken<'source>,
    end_token: PklToken<'source>,
    predicate: &F,
) -> ParsingResult<Vec<R>>
where
    F: Fn(&mut PklLexer<'source>) -> ParsingResult<R> + 'static,
{
    let mut result_vec = Vec::new();

    loop {
        result_vec.push(predicate(lexer)?);

        let token = retrieve_next_token(lexer)?;

        if end_token == token {
            break;
        }
        if separator_token == token {
            continue;
        }
        return Err(ParsingError::unexpected(
            lexer,
            format!("{} or {}", end_token, separator_token),
        ));
    }

    Ok(result_vec)
}

/// This function creates a list out of a `predicate` that will be ran the `end_token` is encountered.
/// The `separator_token` will be skipped **ONCE**, after each time the `predicate` is ran.
///
/// *NOTE*: This function also skips the `separator_token` before running the predicate for the first time
pub fn list_while_not_token1<'source, R, F>(
    lexer: &mut PklLexer<'source>,
    separator_token: PklToken<'source>,
    end_token: PklToken<'source>,
    predicate: &F,
) -> ParsingResult<Vec<R>>
where
    F: Fn(&mut PklLexer<'source>, PklToken<'source>) -> ParsingResult<R> + 'static,
{
    let mut result_vec = Vec::new();
    let mut was_separator_token_encountered = false;

    loop {
        let token = retrieve_next_token(lexer)?;

        if end_token == token {
            break;
        }
        if separator_token == token {
            if was_separator_token_encountered {
                return Err(ParsingError::unexpected(lexer, format!("{}", end_token)));
            }
            was_separator_token_encountered = true;
            continue;
        }
        was_separator_token_encountered = false;
        result_vec.push(predicate(lexer, token)?);
    }

    Ok(result_vec)
}

/// This function creates a list out of a `predicate` that will be ran until the `end_token` is encountered.
/// The `separator_token` will be skipped after each time the `predicate` is ran.
///
/// *NOTE*: This function is the same as `list_while_not_token0` except for one thing:
/// the `predicate` reads the next token and needs to return it.
pub fn list_while_not_token2<'source, R, F>(
    lexer: &mut PklLexer<'source>,
    separator_token: PklToken<'source>,
    end_token: PklToken<'source>,
    predicate: &F,
) -> ParsingResult<Vec<R>>
where
    F: Fn(&mut PklLexer<'source>) -> ParsingResult<(R, Option<PklToken<'source>>)> + 'static,
{
    let mut result_vec = Vec::new();

    loop {
        let (result, next_token) = predicate(lexer)?;
        result_vec.push(result);

        // if None, does not necessarily mean that there is no token next in the lexer
        if let Some(token) = next_token {
            if end_token == token {
                break;
            }
            if separator_token == token {
                continue;
            }
            return Err(ParsingError::unexpected(
                lexer,
                format!("{} or {}", end_token, separator_token),
            ));
        }
    }

    Ok(result_vec)
}

/// This function creates a list out of a `predicate` that will be ran until one of the `end_tokens` is encountered.
/// The `separator_token` will be skipped after each time the `predicate` is ran.
///
/// *NOTE*: This function is the same as `list_while_not_token2` except for one thing:
/// it can take several end_tokens and returns the ending one.
pub fn list_while_not_tokens<'source, R, F>(
    lexer: &mut PklLexer<'source>,
    separator_token: PklToken<'source>,
    end_tokens: &[PklToken<'source>],
    predicate: &F,
) -> ParsingResult<(Vec<R>, PklToken<'source>)>
where
    F: Fn(&mut PklLexer<'source>) -> ParsingResult<(R, Option<PklToken<'source>>)> + 'static,
{
    let mut result_vec = Vec::new();
    let mut _final_end_token = None;

    loop {
        let (result, next_token) = predicate(lexer)?;
        result_vec.push(result);

        // if None, does not necessarily mean that there is no token next in the lexer
        if let Some(token) = next_token {
            if end_tokens.contains(&token) {
                _final_end_token = Some(token);
                break;
            }
            if separator_token == token {
                continue;
            }
            return Err(ParsingError::unexpected(
                lexer,
                format!("One of {:?} or {}", end_tokens, separator_token),
            ));
        }
    }

    Ok((result_vec, _final_end_token.unwrap()))
}

/// This function creates a HashMap out of a `predicate` that will be ran until the `end_token` is encountered.
/// The `separator_token` will be skipped indefinitely, after each time the `predicate` is ran.
///
/// The predicate should return a tuple with the item to insert as a key and its value.
/// If your predicate needs to read the next token, and you want to return it, see `hashmap_while_not_token1`.
///
/// *NOTE*: This function also skips the `separator_token` before running the predicate for the first time
pub fn hashmap_while_not_token0<'source, K, V, F>(
    lexer: &mut PklLexer<'source>,
    separator_token: PklToken<'source>,
    end_token: PklToken<'source>,
    predicate: &F,
) -> ParsingResult<HashMap<K, V>>
where
    K: Eq + Hash,
    F: Fn(&mut PklLexer<'source>, PklToken<'source>) -> ParsingResult<(K, V)> + 'static,
{
    let mut result_vec = HashMap::new();

    loop {
        let token = retrieve_next_token(lexer)?;

        if end_token == token {
            break;
        }
        if separator_token == token {
            continue;
        }

        let (key, value) = predicate(lexer, token)?;
        result_vec.insert(key, value);
    }

    Ok(result_vec)
}

/// This function creates a HashMap out of a `predicate` that will be ran until the `end_token` is encountered.
/// The `separator_token` will be skipped indefinitely, after each time the `predicate` is ran.
///
/// The predicate should return a tuple with the item to insert as a key,
/// its value and the next token that was read by the predicate (and needs to be returned).
/// If your predicate does not need to read the next token, see `hashmap_while_not_token0`.
///
/// *NOTE*: This function also skips the `separator_token` before running the predicate for the first time
pub fn hashmap_while_not_token1<'source, K, V, F>(
    lexer: &mut PklLexer<'source>,
    separator_token: PklToken<'source>,
    end_token: PklToken<'source>,
    predicate: &F,
) -> ParsingResult<HashMap<K, V>>
where
    K: Eq + Hash,
    F: Fn(
            &mut PklLexer<'source>,
            PklToken<'source>,
        ) -> ParsingResult<(K, V, Option<PklToken<'source>>)>
        + 'static,
{
    let mut result_vec = HashMap::new();

    loop {
        let token = retrieve_next_token(lexer)?;

        if end_token == token {
            break;
        }
        if separator_token == token {
            continue;
        }

        let (key, value, next_token) = predicate(lexer, token)?;
        result_vec.insert(key, value);

        if let Some(token) = next_token {
            if token == end_token {
                break;
            }

            if separator_token == token {
                continue;
            }

            return Err(ParsingError::unexpected(
                lexer,
                format!("{} or {}", end_token, separator_token),
            ));
        }
    }

    Ok(result_vec)
}

/// This function creates a HashMap out of a `predicate` that will be ran until the `end_token` is encountered.
/// The `separator_token` will be skipped indefinitely, after each time the `predicate` is ran.
///
/// The predicate should return a tuple with the item to insert as a key and its value, as well as the next token.
///
/// *NOTE*: This function is the same as `hashmap_while_not_token0` except for one thing:
/// the `predicate` reads the next token and needs to return it.
pub fn hashmap_while_not_token2<'source, K, V, F>(
    lexer: &mut PklLexer<'source>,
    separator_token: PklToken<'source>,
    end_token: PklToken<'source>,
    predicate: &F,
) -> ParsingResult<HashMap<K, V>>
where
    K: Eq + Hash,
    F: Fn(
            &mut PklLexer<'source>,
            PklToken<'source>,
        ) -> ParsingResult<((K, V), Option<PklToken<'source>>)>
        + 'static,
{
    let mut result_vec = HashMap::new();

    loop {
        let token = retrieve_next_token(lexer)?;

        if end_token == token {
            break;
        }
        if separator_token == token {
            continue;
        }

        let ((key, value), next_token) = predicate(lexer, token)?;
        result_vec.insert(key, value);

        // if None, does not necessarily mean that there is no token next in the lexer
        if let Some(token) = next_token {
            if end_token == token {
                break;
            }
            if separator_token == token {
                continue;
            }
            return Err(ParsingError::unexpected(
                lexer,
                format!("{} or {}", end_token, separator_token),
            ));
        }
    }

    Ok(result_vec)
}

pub use identifier::parse_identifier;
pub use string::parse_string_literal;

use super::PklLexer;
