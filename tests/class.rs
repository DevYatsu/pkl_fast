use pkl_fast::prelude::{lex, parse};

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

    let tokens = lex(S);
    let statements = parse(tokens);

    assert_eq!(statements.is_ok(), true);
}
