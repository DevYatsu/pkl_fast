use winnow::{
    ascii::multispace0,
    ascii::{line_ending, multispace1, space0},
    combinator::cut_err,
    combinator::delimited,
    error::ParserError,
    error::{StrContext, StrContextValue},
    PResult, Parser,
};

pub mod id;
pub mod string;
pub mod var;

pub const GLOBAL_KEYWORDS: [&str; 30] = [
    "amends",
    "extends",
    "import",
    "module",
    "open",
    "abstract",
    "hidden",
    "local",
    "class",
    "typealias",
    "let",
    "for",
    "when",
    "default",
    "true",
    "false",
    // reserved for future use
    "protected",
    "override",
    "record",
    "delete",
    "match",
    "case",
    "switch",
    "vararg",
    "const",
    //types
    "number",
    "Int",
    "Float",
    "unknown",
    "any",
];

pub fn line_ending_or_end<'source>(input: &mut &'source str) -> PResult<&'source str> {
    space0.parse_next(input)?;

    if input.len() > 0 {
        line_ending.parse_next(input)
    } else {
        Ok("")
    }
}
pub fn expected(what: &'static str) -> StrContext {
    StrContext::Expected(StrContextValue::Description(what))
}

pub fn cut_multispace1<'source>(input: &mut &'source str) -> PResult<&'source str> {
    cut_err(multispace1)
        .context(expected("space"))
        .parse_next(input)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, F, O, E: ParserError<&'a str>>(inner: F) -> impl Parser<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

// pub fn retrieve_next_token<'source>(
//     parser: &mut PklParser<'source>,
// ) -> ParsingResult<PklToken<'source>> {
//     let token = parser.lexer.next();

//     if token.is_none() {
//         return Err(ParsingError::eof(parser, "something"));
//     }

//     match token.unwrap() {
//         Err(e) => Err(ParsingError::lexing(parser, e)),
//         token => Ok(token?),
//     }
// }

// pub fn retrieve_opt_next_token<'source>(
//     parser: &mut PklParser<'source>,
// ) -> ParsingResult<Option<PklToken<'source>>> {
//     let token = parser.lexer.next();

//     match token {
//         Some(Err(e)) => Err(ParsingError::lexing(parser, e)),
//         Some(token) => Ok(Some(token?)),
//         None => Ok(None),
//     }
// }

// pub fn expect_token<'source>(
//     parser: &mut PklParser<'source>,
//     target_token: PklToken<'source>,
// ) -> ParsingResult<()> {
//     let token = parser.lexer.next();

//     if token.is_none() {
//         return Err(ParsingError::eof(parser, &format!("a {target_token}")));
//     }

//     match token.unwrap() {
//         Err(e) => Err(ParsingError::lexing(parser, e))?,
//         Ok(token) if token == target_token => Ok(()),
//         _ => Err(ParsingError::unexpected(parser, target_token.to_string()))?,
//     }
// }

// pub fn expect_token_with_opt_newlines<'source>(
//     parser: &mut PklParser<'source>,
//     opt_current_token: Option<PklToken<'source>>,
//     target_token: PklToken<'source>,
// ) -> ParsingResult<()> {
//     match opt_current_token {
//         Some(PklToken::NewLine) => (),
//         Some(token) if token == target_token => return Ok(()),
//         None => (),
//         _ => Err(ParsingError::unexpected(parser, target_token.to_string()))?,
//     }

//     loop {
//         let token = parser.lexer.next();

//         if token.is_none() {
//             return Err(ParsingError::eof(parser, &format!("{target_token}")));
//         }

//         match token.unwrap() {
//             Err(e) => Err(ParsingError::lexing(parser, e))?,
//             Ok(token) if token == PklToken::NewLine => continue,
//             Ok(token) if token == target_token => return Ok(()),
//             _ => Err(ParsingError::unexpected(parser, target_token.to_string()))?,
//         }
//     }
// }

// pub fn parse_opt_newlines<'source, F, R>(
//     parser: &mut PklParser<'source>,
//     predicate: &F,
// ) -> ParsingResult<R>
// where
//     F: Fn(&mut PklParser<'source>, Option<PklToken<'source>>) -> ParsingResult<R>,
// {
//     loop {
//         let token = retrieve_next_token(parser)?;

//         if PklToken::NewLine != token {
//             return predicate(parser, Some(token));
//         }
//     }
// }

// pub fn assert_token_eq<'source>(
//     parser: &mut PklParser<'source>,
//     token_option: Option<PklToken<'source>>,
//     expected_token: PklToken<'source>,
// ) -> ParsingResult<()> {
//     if token_option.is_none() {
//         return Err(ParsingError::eof(parser, &expected_token.to_string()));
//     }

//     match token_option.unwrap() {
//         token if token == expected_token => Ok(()),
//         _ => Err(ParsingError::unexpected(parser, expected_token.to_string()))?,
//     }
// }

// pub fn expect_statement_end<'source>(parser: &mut PklParser<'source>) -> ParsingResult<()> {
//     match parser.lexer.next() {
//         Some(Err(e)) => Err(ParsingError::lexing(parser, e)),
//         Some(Ok(token))
//             if token == PklToken::NewLine
//                 || token == PklToken::LineComment
//                 || token == PklToken::DocComment =>
//         {
//             Ok(())
//         }
//         Some(_) => Err(ParsingError::unexpected(parser, "line ending".to_string())),
//         None => Ok(()),
//     }
// }

// /// This function creates a list out of a `predicate` that will be ran until one of the `end_token` is encountered.
// /// The `separator_token` will be skipped after each time the `predicate` is ran.
// ///
// /// *NOTE*: Contrary to `list_while_not_token1` function, this function does not skip the `separator_token` before running the predicate for the first time
// pub fn list_while_not_token0<'source, R, F>(
//     parser: &mut PklParser<'source>,
//     separator_token: PklToken<'source>,
//     end_token: PklToken<'source>,
//     predicate: &F,
// ) -> ParsingResult<Vec<R>>
// where
//     F: Fn(&mut PklParser<'source>) -> ParsingResult<R> + 'static,
// {
//     let mut result_vec = Vec::new();

//     loop {
//         result_vec.push(predicate(parser)?);

//         let token = retrieve_next_token(parser)?;

//         if end_token == token {
//             break;
//         }
//         if separator_token == token {
//             continue;
//         }
//         return Err(ParsingError::unexpected(
//             parser,
//             format!("{} or {}", end_token, separator_token),
//         ));
//     }

//     Ok(result_vec)
// }

// /// This function creates a list out of a `predicate` that will be ran the `end_token` is encountered.
// /// The `separator_token` will be skipped **ONCE**, after each time the `predicate` is ran.
// ///
// /// *NOTE*: This function also skips the `separator_token` before running the predicate for the first time
// pub fn list_while_not_token1<'source, R, F>(
//     parser: &mut PklParser<'source>,
//     separator_token: PklToken<'source>,
//     end_token: PklToken<'source>,
//     predicate: &F,
// ) -> ParsingResult<Vec<R>>
// where
//     F: Fn(&mut PklParser<'source>, PklToken<'source>) -> ParsingResult<R> + 'static,
// {
//     let mut result_vec = Vec::new();
//     let mut was_separator_token_encountered = false;

//     loop {
//         let token = retrieve_next_token(parser)?;

//         if end_token == token {
//             break;
//         }
//         if separator_token == token {
//             if was_separator_token_encountered {
//                 return Err(ParsingError::unexpected(parser, format!("{}", end_token)));
//             }
//             was_separator_token_encountered = true;
//             continue;
//         }
//         was_separator_token_encountered = false;
//         result_vec.push(predicate(parser, token)?);
//     }

//     Ok(result_vec)
// }

// /// This function creates a list out of a `predicate` that will be ran until the `end_token` is encountered.
// /// The `separator_token` will be skipped after each time the `predicate` is ran.
// ///
// /// *NOTE*: This function is the same as `list_while_not_token0` except for one thing:
// /// the `predicate` reads the next token and needs to return it.
// pub fn list_while_not_token2<'source, R, F>(
//     parser: &mut PklParser<'source>,
//     separator_token: PklToken<'source>,
//     end_token: PklToken<'source>,
//     predicate: &F,
// ) -> ParsingResult<Vec<R>>
// where
//     F: Fn(
//             &mut PklParser<'source>,
//             Option<PklToken<'source>>,
//         ) -> ParsingResult<(R, Option<PklToken<'source>>)>
//         + 'static,
// {
//     let mut result_vec = Vec::new();

//     loop {
//         let opt_token = retrieve_opt_next_token(parser)?;

//         if let Some(t) = &opt_token {
//             if t == &end_token {
//                 break;
//             }
//         }

//         let (result, next_token) = predicate(parser, opt_token)?;
//         result_vec.push(result);

//         // if None, does not necessarily mean that there is no token next in the parser
//         if let Some(token) = next_token {
//             if end_token == token {
//                 break;
//             }
//             if separator_token == token {
//                 continue;
//             }
//             return Err(ParsingError::unexpected(
//                 parser,
//                 format!("{} or {}", end_token, separator_token),
//             ));
//         }
//     }

//     Ok(result_vec)
// }

// /// This function creates a list out of a `predicate` that will be ran until the `end_token` is encountered.
// /// The `separator_token` will be skipped after each time the `predicate` is ran.
// ///
// /// *NOTE*: This function is the same as `list_while_not_token2` except for one thing:
// /// the separator tokens are jumped at the start and the predicate is sure to receive a valid token as parameter.
// pub fn list_while_not_token3<'source, R, F>(
//     parser: &mut PklParser<'source>,
//     separator_tokens: &[PklToken<'source>],
//     end_token: PklToken<'source>,
//     predicate: &F,
// ) -> ParsingResult<Vec<R>>
// where
//     F: Fn(
//             &mut PklParser<'source>,
//             PklToken<'source>,
//         ) -> ParsingResult<(R, Option<PklToken<'source>>)>
//         + 'static,
// {
//     let mut result_vec = Vec::new();

//     loop {
//         let token = retrieve_next_token(parser)?;

//         if separator_tokens.contains(&token) {
//             continue;
//         }
//         if token == end_token {
//             break;
//         }

//         let (result, next_token) = predicate(parser, token)?;
//         result_vec.push(result);

//         // if None, does not necessarily mean that there is no token next in the parser
//         if let Some(token) = next_token {
//             if token == end_token {
//                 break;
//             }
//             if separator_tokens.contains(&token) {
//                 continue;
//             }
//             return Err(ParsingError::unexpected(
//                 parser,
//                 format!(
//                     "one of {} or {}",
//                     separator_tokens
//                         .iter()
//                         .map(|x| x.to_string())
//                         .collect::<Vec<String>>()
//                         .join(","),
//                     end_token
//                 ),
//             ));
//         }
//     }

//     Ok(result_vec)
// }

// /// This function creates a list out of a `predicate` that will be ran until one of the `end_tokens` is encountered.
// /// The `separator_token` will be skipped after each time the `predicate` is ran.
// ///
// /// *NOTE*: This function is the same as `list_while_not_token2` except for one thing:
// /// it can take several end_tokens and returns the ending one.
// pub fn list_while_not_tokens<'source, R, F>(
//     parser: &mut PklParser<'source>,
//     separator_token: PklToken<'source>,
//     end_tokens: &[PklToken<'source>],
//     predicate: &F,
// ) -> ParsingResult<(Vec<R>, PklToken<'source>)>
// where
//     F: Fn(
//             &mut PklParser<'source>,
//             Option<PklToken<'source>>,
//         ) -> ParsingResult<(R, Option<PklToken<'source>>)>
//         + 'static,
// {
//     let mut result_vec = Vec::new();
//     let mut _final_end_token = None;

//     loop {
//         let (result, next_token) = predicate(parser, None)?;
//         result_vec.push(result);

//         // if None, does not necessarily mean that there is no token next in the parser
//         if let Some(token) = next_token {
//             if end_tokens.contains(&token) {
//                 _final_end_token = Some(token);
//                 break;
//             }
//             if separator_token == token {
//                 continue;
//             }
//             return Err(ParsingError::unexpected(
//                 parser,
//                 format!("One of {:?} or {}", end_tokens, separator_token),
//             ));
//         }
//     }

//     Ok((result_vec, _final_end_token.unwrap()))
// }

// /// This function creates a HashMap out of a `predicate` that will be ran until the `end_token` is encountered.
// /// The `separator_token` will be skipped indefinitely, after each time the `predicate` is ran.
// ///
// /// The predicate should return a tuple with the item to insert as a key and its value.
// /// If your predicate needs to read the next token, and you want to return it, see `hashmap_while_not_token1`.
// ///
// /// *NOTE*: This function also skips the `separator_token` before running the predicate for the first time
// pub fn hashmap_while_not_token0<'source, K, V, F>(
//     parser: &mut PklParser<'source>,
//     separator_token: PklToken<'source>,
//     end_token: PklToken<'source>,
//     predicate: &F,
// ) -> ParsingResult<HashMap<K, V>>
// where
//     K: Eq + Hash,
//     F: Fn(&mut PklParser<'source>, PklToken<'source>) -> ParsingResult<(K, V)> + 'static,
// {
//     let mut result_vec = HashMap::new();

//     loop {
//         let token = retrieve_next_token(parser)?;

//         if end_token == token {
//             break;
//         }
//         if separator_token == token {
//             continue;
//         }

//         let (key, value) = predicate(parser, token)?;
//         result_vec.insert(key, value);
//     }

//     Ok(result_vec)
// }

// /// This function creates a HashMap out of a `predicate` that will be ran until the `end_token` is encountered.
// /// The `separator_token` will be skipped indefinitely, after each time the `predicate` is ran.
// ///
// /// The predicate should return a tuple with the item to insert as a key,
// /// its value and the next token that was read by the predicate (and needs to be returned).
// /// If your predicate does not need to read the next token, see `hashmap_while_not_token0`.
// ///
// /// *NOTE*: This function also skips the `separator_token` before running the predicate for the first time
// pub fn hashmap_while_not_token1<'source, K, V, F>(
//     parser: &mut PklParser<'source>,
//     separator_token: PklToken<'source>,
//     end_token: PklToken<'source>,
//     predicate: &F,
// ) -> ParsingResult<HashMap<K, V>>
// where
//     K: Eq + Hash,
//     F: Fn(
//             &mut PklParser<'source>,
//             PklToken<'source>,
//         ) -> ParsingResult<(K, V, Option<PklToken<'source>>)>
//         + 'static,
// {
//     let mut result_vec = HashMap::new();

//     loop {
//         let token = retrieve_next_token(parser)?;

//         if separator_token == token {
//             continue;
//         }
//         if end_token == token {
//             break;
//         }

//         let (key, value, next_token) = predicate(parser, token)?;
//         result_vec.insert(key, value);

//         if let Some(token) = next_token {
//             if token == end_token {
//                 break;
//             }

//             if separator_token == token {
//                 continue;
//             }

//             return Err(ParsingError::unexpected(
//                 parser,
//                 format!("{} or {}", end_token, separator_token),
//             ));
//         }
//     }

//     Ok(result_vec)
// }

// /// This function creates a HashMap out of a `predicate` that will be ran until the `end_token` is encountered.
// /// The `separator_token` will be skipped indefinitely, after each time the `predicate` is ran.
// ///
// /// The predicate should return a tuple with the item to insert as a key and its value, as well as the next token.
// ///
// /// *NOTE*: This function is the same as `hashmap_while_not_token0` except for one thing:
// /// the `predicate` reads the next token and needs to return it.
// pub fn hashmap_while_not_token2<'source, K, V, F>(
//     parser: &mut PklParser<'source>,
//     separator_token: PklToken<'source>,
//     end_token: PklToken<'source>,
//     predicate: &F,
// ) -> ParsingResult<HashMap<K, V>>
// where
//     K: Eq + Hash,
//     F: Fn(
//             &mut PklParser<'source>,
//             PklToken<'source>,
//         ) -> ParsingResult<((K, V), Option<PklToken<'source>>)>
//         + 'static,
// {
//     let mut result_vec = HashMap::new();

//     loop {
//         let token = retrieve_next_token(parser)?;

//         if end_token == token {
//             break;
//         }
//         if separator_token == token {
//             continue;
//         }

//         let ((key, value), next_token) = predicate(parser, token)?;
//         result_vec.insert(key, value);

//         // if None, does not necessarily mean that there is no token next in the parser
//         if let Some(token) = next_token {
//             if end_token == token {
//                 break;
//             }
//             if separator_token == token {
//                 continue;
//             }
//             return Err(ParsingError::unexpected(
//                 parser,
//                 format!("{} or {}", end_token, separator_token),
//             ));
//         }
//     }

//     Ok(result_vec)
// }
