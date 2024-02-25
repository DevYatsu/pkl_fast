use logos::Logos;
use pkl_fast::lexer::PklToken;
use pkl_fast::parser::{import::ImportClause, parse, Statement};
use std::path::Path;

#[test]
fn import_as() {
    const IMPORT_STR: &str = "import \"test.pkl\" as test";
    let tokens = PklToken::lexer(IMPORT_STR);
    let statements = parse(tokens);

    assert_eq!(
        statements.unwrap(),
        vec![Statement::Import {
            clause: ImportClause::LocalFile(&Path::new("test.pkl")),
            imported_as: Some("test"),
        }]
    )
}

#[test]
fn import() {
    const IMPORT_STR: &str = "import \"test.pkl\"";
    let tokens = PklToken::lexer(IMPORT_STR);
    let statements = parse(tokens);

    assert_eq!(
        statements.unwrap(),
        vec![Statement::Import {
            clause: ImportClause::LocalFile(&Path::new("test.pkl")),
            imported_as: None,
        }]
    )
}
