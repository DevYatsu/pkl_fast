use pkl_fast::{parser::statement::ImportClause, prelude::Statement};
use std::path::Path;

#[test]
fn import_as() {
    let source: &str = "import \"test.pkl\" as test";
    use pkl_fast::{
        lexer::string::sanitize_code,
        prelude::{lex, parse},
    };
    let (code, updated_code, str_vec) = sanitize_code(source);
    let lexer = lex(&updated_code);
    let statements = parse(code, lexer, str_vec);

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
    let source: &str = "import \"test.pkl\"";
    use pkl_fast::{
        lexer::string::sanitize_code,
        prelude::{lex, parse},
    };
    let (code, updated_code, str_vec) = sanitize_code(source);
    let lexer = lex(&updated_code);
    let statements = parse(code, lexer, str_vec);

    assert_eq!(
        statements.unwrap(),
        vec![Statement::Import {
            clause: ImportClause::LocalFile(&Path::new("test.pkl")),
            imported_as: None,
        }]
    )
}
