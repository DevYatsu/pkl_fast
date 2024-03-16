use pkl_fast::parser::parse;

#[test]
fn empty() {
    const EMPTY_STR: &str = "";
    let statements = parse("main.pkl", EMPTY_STR);

    // checks if tokens iterator is empty
    assert_eq!(statements.is_ok(), true);
    assert_eq!(statements.unwrap().len(), 0);
}
