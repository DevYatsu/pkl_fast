use pkl_fast::lexer::PklToken;
use pkl_fast::parser::{parse, Statement, import::ImportClause};
use logos::Logos;
use std::path::Path;

#[test]
fn import_as() {
    const import_str: &str = "import \"test.pkl\" as test";
    let mut tokens = PklToken::lexer(import_str);
    let mut statements = parse(tokens);

    assert_eq!(statements.unwrap(), vec![Statement::Import {
        clause: ImportClause::LocalFile(&Path::new("test.pkl")),
        imported_as: Some("test"),
    }])
}


#[test]
fn import() {
    const import_str: &str = "import \"test.pkl\"";
    let mut tokens = PklToken::lexer(import_str);
    let mut statements = parse(tokens);

    assert_eq!(statements.unwrap(), vec![Statement::Import {
        clause: ImportClause::LocalFile(&Path::new("test.pkl")),
        imported_as: None,
    }])
}