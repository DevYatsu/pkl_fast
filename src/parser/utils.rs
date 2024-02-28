use std::{collections::HashMap, hash::Hash};

use crate::prelude::{ParsingError, ParsingResult, PklLexer, PklToken};
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

pub fn expect_token<'source>(
    lexer: &mut PklLexer<'source>,
    target_token: PklToken<'source>,
) -> ParsingResult<()> {
    let token = lexer.next();

    if token.is_none() {
        return Err(ParsingError::eof(lexer));
    }
    println!("{}", lexer.slice());
    match token.unwrap() {
        Err(e) => Err(ParsingError::lexing(lexer, e))?,
        Ok(token) if token == target_token => Ok(()),
        _ => Err(ParsingError::unexpected(lexer))?,
    }
}

pub fn list_while_not_token<'source, R>(
    lexer: &mut PklLexer<'source>,
    separator_token: PklToken<'source>,
    end_token: PklToken<'source>,
    predicate: &dyn Fn(&mut PklLexer<'source>, PklToken<'source>) -> ParsingResult<R>,
) -> ParsingResult<Vec<R>> {
    let mut result_vec = Vec::new();

    loop {
        let token = retrieve_next_token(lexer)?;

        if token == end_token {
            break;
        }
        if token == separator_token {
            continue;
        }

        result_vec.push(predicate(lexer, token)?);
    }

    Ok(result_vec)
}

pub fn hashmap_while_not_token<'source, K, V>(
    lexer: &mut PklLexer<'source>,
    separator_token: PklToken<'source>,
    end_token: PklToken<'source>,
    predicate: &dyn Fn(&mut PklLexer<'source>, PklToken<'source>) -> ParsingResult<(K, V)>,
) -> ParsingResult<HashMap<K, V>>
where
    K: Eq + Hash,
{
    let mut result_vec = HashMap::new();

    loop {
        let token = retrieve_next_token(lexer)?;

        if token == end_token {
            break;
        }
        if token == separator_token {
            continue;
        }

        let (key, value) = predicate(lexer, token)?;
        result_vec.insert(key, value);
    }

    Ok(result_vec)
}

pub use identifier::parse_identifier;
pub use string::parse_string_literal;
