use pkl_core::ModuleSource;

#[test]
fn test_module_source_from_text() {
    let src = ModuleSource::from_text("name = 42");
    assert!(src.uri.starts_with("repl:"), "uri should start with repl:, got: {}", src.uri);
    assert_eq!(src.contents.unwrap(), "name = 42");
}

#[test]
fn test_module_source_from_text_unique_uri() {
    let src1 = ModuleSource::from_text("a = 1");
    let src2 = ModuleSource::from_text("a = 2");
    assert_ne!(src1.uri, src2.uri, "each from_text should generate a unique URI");
}

#[test]
fn test_module_source_from_file() {
    let src = ModuleSource::from_file("/usr/local/config.pkl");
    assert_eq!(src.uri, "file:///usr/local/config.pkl");
    assert!(src.contents.is_none());
}

#[test]
fn test_module_source_from_uri() {
    let src = ModuleSource::from_uri("https://example.com/config.pkl");
    assert_eq!(src.uri, "https://example.com/config.pkl");
    assert!(src.contents.is_none());
}
