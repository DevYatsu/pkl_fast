use pkl_fast::lexer::PklToken;
use pkl_fast::Logos;

#[test]
fn empty() {
    const EMPTY_STR: &str = "";
    let mut tokens = PklToken::lexer(EMPTY_STR);

    // checks if tokens iterator is empty
    assert_eq!(tokens.next(), None)
}
