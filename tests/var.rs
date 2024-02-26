use pkl_fast::prelude::{lex, parse};

#[test]
fn floats_var() {
    const S: &str = "num1 = .23
num2 = 1.23
num3 = 1.23e2 
num4 = 1.23e-2
notANumber = NaN
positiveInfinity = Infinity
negativeInfinity = -Infinity";

    let tokens = lex(S);
    let statements = parse(tokens);

    assert_eq!(statements.is_ok(), true);
    assert_eq!(statements.unwrap().len() == S.lines().count(), true)
}

#[test]
fn ints_var() {
    const S: &str = "num1 = 123
num2 = 0x012AFF 
num3 = 0b00010111 
num4 = 0o755 
num1 = 1_000_000 
num2 = 0x0134_64DE 
num3 = 0b0001_0111 
num4 = 0o0134_6475";

    let tokens = lex(S);
    let statements = parse(tokens);

    assert_eq!(statements.is_ok(), true);
    assert_eq!(statements.unwrap().len() == S.lines().count(), true)
}
