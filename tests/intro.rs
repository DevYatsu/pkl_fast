use pkl_fast::{parser::parse, prelude::lex};

#[test]
fn intro_docs_example() {
    const SOURCE: &str = "name = \"Pkl: Configure your Systems in New Ways\"
attendants = 100
isInteractive = true
amountLearned = 13.37
";

    let tokens = lex(SOURCE);
    let statements = parse(tokens);

    assert_eq!(statements.is_ok(), true);
    assert_eq!(statements.unwrap().len() == 4, true)
}
