use pkl_fast::{parser::statement::ImportClause, prelude::Statement};
use std::path::Path;

#[test]
fn import_as() {
    const IMPORT_STR: &str = "import \"test.pkl\" as test";
    use pkl_fast::{
        lexer::string::sanitize_code,
        prelude::{lex, parse},
    };
    let (code, str_vec) = sanitize_code(IMPORT_STR);
    let tokens = lex(&code);
    let statements = parse(tokens, str_vec);

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
    use pkl_fast::{
        lexer::string::sanitize_code,
        prelude::{lex, parse},
    };
    let (code, str_vec) = sanitize_code(IMPORT_STR);
    let tokens = lex(&code);
    let statements = parse(tokens, str_vec);

    assert_eq!(
        statements.unwrap(),
        vec![Statement::Import {
            clause: ImportClause::LocalFile(&Path::new("test.pkl")),
            imported_as: None,
        }]
    )
}
