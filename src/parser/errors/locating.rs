use miette::{NamedSource, SourceSpan};

use crate::prelude::PklParser;

pub fn get_error_location<'source>(parser: &PklParser) -> SourceSpan {
    SourceSpan::new((parser.source_input().len() - parser.input().len()).into(), 5)
}

pub fn generate_source(file_name: &str, source: &str) -> NamedSource<String> {
    NamedSource::new(file_name, source.to_string())
}

pub fn set_error_location<'source>(start_index: usize, length: usize) -> SourceSpan {
    todo!()
    // let offset: SourceOffset = lexer.span().start.into();

    // SourceSpan::new((offset.offset() + start_index).into(), length)
}
