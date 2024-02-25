use pkl_fast::lexer::PklToken;
use logos::Logos;

#[test]
fn empty() {
    const empty_str: &str = "";
    let mut tokens = PklToken::lexer(empty_str);

    // checks if tokens iterator is empty
    assert_eq!(tokens.next(), None)
}