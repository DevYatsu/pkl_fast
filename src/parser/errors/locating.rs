use crate::parser::PklLexer;
use miette::SourceSpan;

pub fn find_last_newline_after_index(input: &str, index: usize) -> usize {
    let mut last_newline_index = 0;

    for (i, c) in input.chars().enumerate().skip(index) {
        if c == '\n' {
            last_newline_index = i;
        }
    }

    last_newline_index
}

pub fn get_error_location<'source>(lexer: &mut PklLexer<'source>) -> SourceSpan {
    (
        lexer.span().start,
        find_last_newline_after_index(lexer.source(), lexer.span().end),
    )
        .into()
}
