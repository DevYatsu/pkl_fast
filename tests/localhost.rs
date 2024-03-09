const SOURCE: &str = "amends \"Application.pkl\"

hostname = \"localhost\"

port = 3599

environment = \"dev\"

/*
database {
  host = \"localhost\"
  port = 5786
  username = \"admin\"
  password = read(\"env:DATABASE_PASSWORD\") 
  dbName = \"myapp\"
}
*/";

#[test]
fn localhost_doc_example() {
    use pkl_fast::{
        lexer::string::sanitize_code,
        prelude::{lex, parse},
    };
    let (code, str_vec) = sanitize_code(SOURCE);
    let tokens = lex(&code);
    let statements = parse(tokens, str_vec);

    assert_eq!(statements.is_ok(), true);
}
