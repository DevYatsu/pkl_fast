use pkl_fast::{
    parser::statement::ImportClause,
    prelude::{parse, Statement},
};
use std::path::Path;

#[test]
fn import_as() {
    let source: &str = "import \"test.pkl\" as test";
    let statements = parse("main.pkl", source);

    assert_eq!(
        statements.unwrap(),
        vec![Statement::Import {
            clause: ImportClause::LocalFile(&Path::new("test.pkl")),
            imported_as: Some("test"),
            is_globbed: false
        }]
    )
}

#[test]
fn import() {
    let source: &str = "import* \"test.pkl\"";
    let statements = parse("main.pkl", source);

    assert_eq!(
        statements.unwrap(),
        vec![Statement::Import {
            clause: ImportClause::LocalFile(&Path::new("test.pkl")),
            imported_as: None,
            is_globbed: true
        }]
    )
}
