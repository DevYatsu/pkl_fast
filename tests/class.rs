#[test]
fn class() {
    const S: &str = "abstract class Bird {
  fixed species: String 
}

class Osprey extends Bird {
  fixed species: \"Pandion haliaetus\" 
}

class Bird2 {
  name: String
  lifespan: Int
}
pigeon = new Bird2 {
  name = \"Pigeon\"
  lifespan = 8
}

class Bird3 {
  name: String
  lifespan: Int
  local separator: String
  hidden nameAndLifespanInIndex: String 
}
";
    use pkl_fast::{
        lexer::string::sanitize_code,
        prelude::{lex, parse},
    };
    let (code, str_vec) = sanitize_code(S);
    let tokens = lex(&code);
    let statements = parse(tokens, str_vec);

    assert_eq!(statements.is_ok(), true);
}
