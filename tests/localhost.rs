use pkl_fast::{parser::parse, prelude::lex};

const SOURCE: &str = "amends \"Application.pkl\"

hostname = \"localhost\"

port = 3599

environment = \"dev\"

database {
  host = \"localhost\"
  port = 5786
  username = \"admin\"
  password = read(\"env:DATABASE_PASSWORD\") 
  dbName = \"myapp\"
}";

#[test]
fn localhost_doc_example() {
    let tokens = lex(SOURCE);
    let statements = parse(tokens);

    assert_eq!(statements.is_ok(), true);
}
