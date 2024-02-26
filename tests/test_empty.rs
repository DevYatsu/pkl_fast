use pkl_fast::prelude::lex;

#[test]
fn empty() {
    const EMPTY_STR: &str = "";
    let mut tokens = lex(EMPTY_STR);

    // checks if tokens iterator is empty
    assert_eq!(tokens.next(), None)
}
