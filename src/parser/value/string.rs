use crate::{
    parser::{
        expression::{parse_expr, Expression},
        utils::expected,
    },
    prelude::{ParsingResult, PklParser},
};
use std::fmt;
use winnow::{
    ascii::{hex_digit1, newline, space0},
    combinator::{alt, cut_err, delimited, repeat_till},
    token::{one_of, take_while},
    PResult, Parser,
};

use super::PklValue;

#[derive(Debug, Clone, PartialEq)]
pub enum StringKind<'source> {
    Classic(&'source str),
    MultiLine(&'source str),
    Delimited {
        start_delimiter: &'source str,
        end_delimiter: &'source str,
        value: &'source str,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum StringFragment<'source> {
    Escaped(char),
    Interpolated(Expression<'source>),
    RawText(&'source str),
    UnicodeEscape(&'source str),
}

/// Parsing a string literal (aka a sequence of Unicode code points), optionnaly composed of:
/// - escape sequences: `\t`, `\n`, `\r`, `\"`, `\\`
/// - unicode escape: `\u{<codePoint>}` where `<codePoint>` is a hexadecimal number between 0 and 10FFFF
/// - string interpolation: `\(<expr>)` to embed the result of an `<expr>` in a string
///
/// *NOTE*: CUSTOM STRINGS DELIMITERS SUPPORT INCOMING!
pub fn string_value<'source>(input: &mut &'source str) -> PResult<PklValue<'source>> {
    '"'.parse_next(input)?;

    repeat_till(
        0..,
        alt((
            escape_sequence,
            take_while(0.., |c: char| '"' != c && '\\' != c)
                .map(StringFragment::RawText)
                .context(expected("string content")),
        )),
        '"',
    )
    .map(|(vec, _)| PklValue::String(vec))
    .parse_next(input)
}

/// Parsing a multiline string literal, starting with `"""`, and optionnaly composed of:
/// - escape sequences: `\t`, `\n`, `\r`, `\"`, `\\`
/// - unicode escape: `\u{<codePoint>}` where `<codePoint>` is a hexadecimal number between 0 and 10FFFF
/// - string interpolation: `\(<expr>)` to embed the result of an `<expr>` in a string
///
/// *NOTE*: Multiline string literals are delimited by three double quotes. String content and closing delimiter must each start on a new line.
pub fn multiline_string_value<'source>(input: &mut &'source str) -> PResult<PklValue<'source>> {
    r#"""""#.parse_next(input)?;
    // content starts on the first line after '"""' token
    space0.parse_next(input)?;
    cut_err(newline)
        .context(expected("new line after multiline string start"))
        .parse_next(input)?;

    cut_err(repeat_till(
        0..,
        alt((
            newline.map(StringFragment::Escaped),
            escape_sequence,
            take_while(1.., |c: char| '"' != c && '\\' != c && '\n' != c)
                .map(StringFragment::RawText),
        )),
        (newline, r#"""""#),
    ))
    .context(expected("valid multiline string"))
    .map(|(vec, _)| PklValue::String(vec))
    .parse_next(input)
}

fn escape_sequence<'source>(input: &mut &'source str) -> PResult<StringFragment<'source>> {
    "\\".parse_next(input)?;

    cut_err(alt((
        delimited(
            '(',
            parse_expr,
            cut_err(')').context(expected("closing parenthesis")),
        )
        .map(StringFragment::Interpolated),
        (
            'u',
            cut_err(delimited(
                cut_err('{').context(expected("opening bracket")),
                unicode,
                cut_err('}').context(expected("closing bracket")),
            )),
        )
            .map(|(_, value)| StringFragment::UnicodeEscape(value)),
        cut_err(one_of(['n', 't', '\\', 'r', '"']))
            .context(expected("valid escape char"))
            .map(StringFragment::Escaped),
    )))
    .parse_next(input)
}

fn unicode<'source>(input: &mut &'source str) -> PResult<&'source str> {
    cut_err(hex_digit1)
        .verify(|s: &str| s.len() <= 6)
        .context(expected("unicode"))
        .parse_next(input)
}

impl<'source> StringFragment<'source> {
    pub fn from_raw_string(
        parser: &mut PklParser<'source>,
        initial_string: &'source str,
    ) -> ParsingResult<Vec<Self>> {
        todo!()

        // let str_kind = &parser.strings_vec[initial_string.parse::<usize>().unwrap()];

        // // function used for parsing both classic strings and multiline ones
        // let mut fragments = Vec::new();

        // match str_kind {
        //     StringKind::Classic(initial_string) | StringKind::MultiLine(initial_string) => {
        //         let mut fragment_start_index = 0;
        //         for (char_index, c) in initial_string.char_indices() {
        //             if char_index != 0 && &initial_string[char_index - 1..char_index] == "\\" {
        //                 continue;
        //             }

        //             match c {
        //                 '\\' => {
        //                     if fragment_start_index != char_index {
        //                         fragments.push(StringFragment::RawText(
        //                             &initial_string[fragment_start_index..char_index],
        //                         ));
        //                     }

        //                     // Check the character after the backslash
        //                     if let Some(after_backslash) =
        //                         initial_string.chars().nth(char_index + 1)
        //                     {
        //                         match after_backslash {
        //                             '(' => {
        //                                 let rest_of_string = &initial_string[char_index + 2..];
        //                                 let mut open_paren_count = 1;

        //                                 for (i, c) in rest_of_string.chars().enumerate() {
        //                                     if c == '(' {
        //                                         open_paren_count += 1;
        //                                     }

        //                                     if c == ')' {
        //                                         open_paren_count -= 1;

        //                                         if open_paren_count == 0 {
        //                                             let expr_str = &rest_of_string[..i];
        //                                             let result = parse_expr(
        //                                                 &mut PklParser::new(
        //                                                     expr_str,
        //                                                     lex(expr_str),
        //                                                     vec![],
        //                                                 ),
        //                                                 None,
        //                                             );

        //                                             if result.is_err() {
        //                                                 let string_in_main_str = &initial_string
        //                                                     [char_index..char_index + 2 + i];
        //                                                 let index_in_main_str = initial_string
        //                                                     .find(string_in_main_str)
        //                                                     .unwrap()
        //                                                     + 1;

        //                                                 match result {
        //                                                     Err(e) => {
        //                                                         let at = e.get_at();

        //                                                         let offset_in_expr = at.offset();
        //                                                         let expr_start_index =
        //                                                             index_in_main_str + 2;

        //                                                         return Err(e.with_attributes(
        //                                                             generate_source(
        //                                                                 "main.pkl",
        //                                                                 parser.lexer.source(),
        //                                                             ),
        //                                                             set_error_location(
        //                                                                 &mut parser.lexer,
        //                                                                 expr_start_index
        //                                                                     + offset_in_expr,
        //                                                                 at.len(),
        //                                                             ),
        //                                                         ));
        //                                                     }
        //                                                     _ => unreachable!(),
        //                                                 }
        //                                             }

        //                                             let (expr, next_token) = result?;
        //                                             if next_token.is_some() {
        //                                                 let string_in_main_str = &initial_string
        //                                                     [char_index..char_index + 2 + i];
        //                                                 let index_in_main_str = initial_string
        //                                                     .find(string_in_main_str)
        //                                                     .unwrap()
        //                                                     + 1;
        //                                                 return Err(
        //                                                     ParsingError::invalid_interpolated_expr(
        //                                                         parser,
        //                                                         index_in_main_str + 2,
        //                                                         i,
        //                                                     ),
        //                                                 );
        //                                             }

        //                                             fragments
        //                                                 .push(StringFragment::Interpolated(expr));
        //                                             fragment_start_index = char_index + 2 + i + 1;
        //                                             break;
        //                                         }
        //                                     }
        //                                 }
        //                             }
        //                             'u' => {
        //                                 // Handle Unicode escape

        //                                 // no opening brace following u, raise an error
        //                                 if let Some(next_char) =
        //                                     initial_string.chars().nth(char_index + 2)
        //                                 {
        //                                     // we can unwrap safely as we are sure the sequence exists in the string
        //                                     if next_char != '{' {
        //                                         return Err(ParsingError::invalid_unicode(
        //                                             parser,
        //                                             char_index + 3,
        //                                             2,
        //                                         ));
        //                                     }
        //                                 }

        //                                 if let Some(close_index) =
        //                                     initial_string[char_index + 2..].find('}')
        //                                 {
        //                                     let hex_value = &initial_string
        //                                         [char_index + 3..char_index + 2 + close_index];

        //                                     if hex_value.len() > 6 || hex_value.is_empty() {
        //                                         // hex_value.len() for highlighting the entire hex_value and +1 to highlight the } following
        //                                         return Err(ParsingError::invalid_unicode(
        //                                             parser,
        //                                             char_index + 6,
        //                                             hex_value.len(),
        //                                         ));
        //                                     }

        //                                     fragments
        //                                         .push(StringFragment::UnicodeEscape(&hex_value));
        //                                     fragment_start_index = char_index + 2 + close_index + 1;
        //                                 } else {
        //                                     // No closing brace found, raise an error
        //                                     return Err(ParsingError::invalid_unicode(
        //                                         parser,
        //                                         char_index + 3,
        //                                         3,
        //                                     ));
        //                                 }
        //                             }
        //                             _ => {
        //                                 const ALLOWED_ESCAPE: [char; 5] =
        //                                     ['t', 'n', 'r', '"', '\\'];

        //                                 if !ALLOWED_ESCAPE.contains(&after_backslash) {
        //                                     return Err(ParsingError::invalid_char_escape(
        //                                         parser,
        //                                         char_index + 3,
        //                                     ));
        //                                 }

        //                                 // Handle other escaped characters
        //                                 fragments.push(StringFragment::Escaped(after_backslash));
        //                                 fragment_start_index = char_index + 2;
        //                             }
        //                         }
        //                     } else {
        //                         // Backslash is at the end of the string, raise an error
        //                         return Err(ParsingError::invalid_char_escape(
        //                             parser,
        //                             initial_string.len(),
        //                         ));
        //                     };
        //                 }

        //                 // for multiline strings
        //                 '\n' => {
        //                     if fragment_start_index != char_index {
        //                         fragments.push(StringFragment::RawText(
        //                             &initial_string[fragment_start_index..char_index],
        //                         ));
        //                     }

        //                     fragments.push(StringFragment::Escaped('n'));
        //                     fragment_start_index = char_index + 1;
        //                 }
        //                 _ => (),
        //             }
        //         }

        //         // Push the remaining raw text
        //         if fragment_start_index != initial_string.len() {
        //             fragments.push(StringFragment::RawText(
        //                 &initial_string[fragment_start_index..],
        //             ));
        //         }
        //     }
        //     StringKind::Delimited {
        //         start_delimiter,
        //         end_delimiter,
        //         value,
        //     } => todo!(),
        // };

        // Ok(fragments)
    }
}

impl<'source> fmt::Display for StringFragment<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StringFragment::Escaped(ch) => write!(f, "\\{}", ch),
            StringFragment::Interpolated(s) => write!(f, "\\({})", s),
            StringFragment::RawText(s) => write!(f, "{}", s),
            StringFragment::UnicodeEscape(s) => write!(f, "\\u{{{}}}", s),
        }
    }
}
