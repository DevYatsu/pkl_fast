#[test]
fn intro_docs_example() {
    const SOURCE: &str = "name = \"Pkl: Configure your Systems in New Ways\"
attendants = 100
isInteractive = true
amountLearned = 13.37
";
    use pkl_fast::{
        lexer::string::sanitize_code,
        prelude::{lex, parse},
    };
    let (code, updated_code, str_vec) = sanitize_code(SOURCE);
    let lexer = lex(&updated_code);
    let statements = parse(code, lexer, str_vec);
    assert_eq!(statements.is_ok(), true);
    assert_eq!(statements.unwrap().len() == 4, true)
}
