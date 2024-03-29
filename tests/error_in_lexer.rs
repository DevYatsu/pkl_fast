use pkl_fast::prelude::{lex, parse};

#[test]
fn error_in_lexing() {
    const INVALID_STR: &str = "###"; // not valid pkl tokens
    let tokens = lex(INVALID_STR);
    let statements = parse(tokens);

    assert_eq!(statements.is_err(), true)
}
