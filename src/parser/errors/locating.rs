use miette::{NamedSource, SourceSpan};

pub fn get_error_location<'source>() -> SourceSpan {
    todo!()
    // SourceSpan::new(lexer.span().start.into(), lexer.span().len())
}

pub fn generate_source(file_name: &str, source: &str) -> NamedSource<String> {
    NamedSource::new(file_name, source.to_string())
}

pub fn set_error_location<'source>(start_index: usize, length: usize) -> SourceSpan {
    todo!()
    // let offset: SourceOffset = lexer.span().start.into();

    // SourceSpan::new((offset.offset() + start_index).into(), length)
}
