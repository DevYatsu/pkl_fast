use pkl_fast::{
    parser::statement::ImportClause,
    prelude::{lex, parse, Statement},
};
use std::path::Path;

#[test]
fn import_as() {
    const IMPORT_STR: &str = "import \"test.pkl\" as test";
    let tokens = lex(IMPORT_STR);
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
    let tokens = lex(IMPORT_STR);
    let statements = parse(tokens);

    assert_eq!(
        statements.unwrap(),
        vec![Statement::Import {
            clause: ImportClause::LocalFile(&Path::new("test.pkl")),
            imported_as: None,
        }]
    )
}
