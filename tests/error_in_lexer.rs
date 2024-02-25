use pkl_fast::{lexer::PklToken, parser::parse, Logos};

#[test]
fn error_in_lexing() {
    const INVALID_STR: &str = "###"; // not valid pkl tokens
    let tokens = PklToken::lexer(INVALID_STR);
    let statements = parse(tokens);

    assert_eq!(statements.is_err(), true)
}
