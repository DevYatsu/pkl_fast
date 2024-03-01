use pkl_fast::{parser::parse, prelude::lex};

#[test]
fn expr() {
    let source = "x=1
test = 2 ** 3 / (3 + 1) + add(1 + 2*x,2**x)

comparison1 = 5 == 2
comparison2 = 5 < 2
comparison3 = 5 > 2
comparison4 = 5 <= 2
comparison5 = 5 >= 2

num1 = 5 + 2   
num2 = 5 - 2   
num3 = 5 * 2   
num4 = 5 / 2   
num5 = 5 ~/ 2  
num6 = 5 % 2   
num7 = 5 ** 2  

res1 = true && false 
res2 = true || false 
res3 = !false 
res4 = true.xor(false) 
res5 = true.implies(false) 
";

    let lexer = lex(source);
    let parser = parse(lexer);

    assert_eq!(parser.is_ok(), true)
}

#[test]
fn other() {
    let src = "name = \"Dodo\"
greeting = \"Hi, \\(name)!\" ";

    let lexer = lex(src);
    let parser = parse(lexer);

    assert_eq!(parser.is_ok(), true)
}
