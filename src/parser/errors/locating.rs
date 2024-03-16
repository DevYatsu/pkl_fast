use miette::{NamedSource, SourceSpan};

use crate::prelude::PklParser;

pub fn get_error_location<'source>(parser: &PklParser, length: usize) -> SourceSpan {
    SourceSpan::new(
        (parser.source_input().len() - parser.input().len()).into(),
        length,
    )
}

pub fn generate_source(file_name: &str, source: &str) -> NamedSource<String> {
    NamedSource::new(file_name, source.to_string())
}
pub fn get_next_element_length(parser: &PklParser) -> usize {
    let input = parser.input();

    let mut length = 0;

    for c in input.chars() {
        if !c.is_whitespace() {
            length += 1;
            if c.is_alphanumeric() || c == '_' || c == '$' {
                while let Some(next) = input.chars().nth(length) {
                    if next.is_alphanumeric() || next == '_' {
                        length += 1;
                    } else {
                        break;
                    }
                }
                break;
            }
            if c == '"' {
                while let Some(next) = input.chars().nth(length) {
                    length += 1;
                    if next == '"' && input.chars().nth(length - 2) != Some('\\') {
                        break;
                    }
                }
                break;
            }
        }
    }

    length
}

pub fn get_next_element_until_inclusive(parser: &PklParser, pattern: &str) -> usize {
    let input = parser.input();

    input
        .find(pattern)
        .map(|x| x + 1)
        .unwrap_or_else(|| input.len() - 1)
}

pub fn get_next_element_until_exclusive(parser: &PklParser, pattern: &str) -> usize {
    let input = parser.input();

    input.find(pattern).unwrap_or_else(|| input.len() - 1)
}
