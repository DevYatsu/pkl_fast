#[test]
fn error_in_lexing() {
    const INVALID_STR: &str = "###"; // not valid pkl tokens
    use pkl_fast::{
        lexer::string::sanitize_code,
        prelude::{lex, parse},
    };
    let (code, updated_code, str_vec) = sanitize_code(INVALID_STR);
    let lexer = lex(&updated_code);
    let statements = parse(code, lexer, str_vec);

    assert_eq!(statements.is_err(), true)
}
