use crate::parser::PklLexer;
use miette::{NamedSource, SourceSpan};

fn find_first_newline_after_index(input: &str, actual_index: usize) -> usize {
    let mut index = 0;

    for (i, c) in input.chars().enumerate().skip(actual_index) {
        if c == '\n' {
            index = i + 1;
            break;
        }
    }

    index
}

pub fn get_error_location<'source>(lexer: &mut PklLexer<'source>) -> SourceSpan {
    (
        lexer.span().start,
        find_first_newline_after_index(lexer.source(), lexer.span().end) + 1,
    )
        .into()
}

pub fn generate_source(file_name: &str, source: &str) -> NamedSource<String> {
    NamedSource::new(file_name, source.to_string())
}
