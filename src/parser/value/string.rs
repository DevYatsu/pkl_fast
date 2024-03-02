use crate::prelude::{ParsingError, PklLexer};
use logos::Logos;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum StringFragment<'source> {
    Escaped(char),
    Interpolated(&'source str),
    RawText(&'source str),
    UnicodeEscape(&'source str),
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum StringLexer<'source> {
    #[regex(r"\\[a-mo-zA-MO-Z]", |lex| lex.slice().chars().nth(1).unwrap())]
    EscapedValue(char),

    #[regex(r"\\\(\w+\)", |lex| let val=lex.slice();&val[2..val.len()-1])]
    EscapeForInterpolated(&'source str),

    #[regex(r"\\u\{[0-9A-Fa-f]{1,6}\}", |lex| {let val=lex.slice();&val[3..val.len()-1]})]
    UnicodeEscape(&'source str),
}

impl<'source> StringFragment<'source> {
    pub fn from_raw_string(
        lexer: &mut PklLexer<'source>,
        initial_string: &'source str,
    ) -> Result<Vec<Self>, ParsingError> {
        let mut fragments = Vec::new();
        let mut raw_string = initial_string;

        while let Some(index) = raw_string.find("\\") {
            // Push raw text before the backslash
            if index > 0 {
                fragments.push(StringFragment::RawText(&raw_string[..index]));
            }

            // Check the character after the backslash
            if let Some(after_backslash) = raw_string.chars().nth(index + 1) {
                let new_index = if after_backslash == '(' {
                    todo!()
                } else if after_backslash == 'u' {
                    // Handle Unicode escape

                    // no opening brace following u, raise an error
                    if let Some(next_char) = raw_string.chars().nth(index + 2) {
                        // we can unwrap safely as we are sure the sequence exists in the string
                        let index = initial_string.find(&format!("\\u{}", next_char)).unwrap();
                        if next_char != '{' {
                            return Err(ParsingError::invalid_unicode(lexer, index + 1, 2));
                        }
                    }

                    if let Some(close_index) = raw_string[index + 2..].find('}') {
                        let hex_value = &raw_string[index + 3..index + 2 + close_index];

                        if hex_value.len() > 6 || hex_value.is_empty() {
                            // we can unwrap safely as we are sure the sequence exists in the string
                            let index = initial_string
                                .find(&format!("\\u{{{}}}", hex_value))
                                .unwrap();

                            // hex_value.len() for highlighting the entire hex_value and +1 to highlight the } following
                            return Err(ParsingError::invalid_unicode(
                                lexer,
                                index + 1,
                                3 + hex_value.len() + 1,
                            ));
                        }

                        fragments.push(StringFragment::UnicodeEscape(&hex_value));
                        index + 2 + close_index + 1
                    } else {
                        // No closing brace found, raise an error
                        return Err(ParsingError::invalid_unicode(lexer, index + 1, 3));
                    }
                } else {
                    const ALLOWED_ESCAPE: [char; 5] = ['t', 'n', 'r', '"', '\\'];

                    if !ALLOWED_ESCAPE.contains(&after_backslash) {
                        // we can unwrap safely as we are sure the sequence exists in the string
                        let index = initial_string
                            .find(&format!("\\{}", after_backslash))
                            .unwrap();
                        return Err(ParsingError::invalid_char_escape(lexer, index + 1));
                    }

                    // Handle other escaped characters
                    fragments.push(StringFragment::Escaped(after_backslash));
                    index + 2
                };

                raw_string = &raw_string[new_index..];
            } else {
                // Backslash is at the end of the string, raise an error
                return Err(ParsingError::invalid_char_escape(
                    lexer,
                    initial_string.len(),
                ));
            }
        }

        // Push the remaining raw text
        if !raw_string.is_empty() {
            fragments.push(StringFragment::RawText(raw_string));
        }

        Ok(fragments)
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
