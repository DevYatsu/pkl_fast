#[test]
fn error_in_lexing() {
    const INVALID_STR: &str = "###"; // not valid pkl tokens
    use pkl_fast::{
        lexer::string::sanitize_code,
        prelude::{lex, parse},
    };
    let (code, str_vec) = sanitize_code(INVALID_STR);
    let tokens = lex(&code);
    let statements = parse(tokens, str_vec);

    assert_eq!(statements.is_err(), true)
}
