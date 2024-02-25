use pkl_fast::lexer::PklToken;
use logos::Logos;
use pkl_fast::parser::{parse, ParsingError};

#[test]
fn error_in_lexing() {
    const empty_str: &str = "###"; // not valid pkl tokens
    let mut tokens = PklToken::lexer(empty_str);

    let mut statements = parse(tokens);

    assert_eq!(statements.ok(), None)
}