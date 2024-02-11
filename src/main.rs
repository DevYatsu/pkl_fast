use logos::{Lexer, Logos};
use lexer::Token;

mod lexer;

fn main() {
    let pkl_code = r#"
        /// Designates whether it is zebra party time.
        // TODO: Add constraints here?
        /* Let's have a zebra party */
        name = "Pkl: Configure your Systems in New Ways"
        attendants = 100
        /* isInteractive = true */
        amountLearned = 13.37

        if (bar)
            bar
        else
        if (baz)
            baz
        else
            foo

        typealias Foo = "foo"|"bar"|"baz"
        res1 = new { bar = "bar"; baz = "baz" }
        res2 = new { 1; 2; 3; 4; 5; 6 }
    "#;

    let lexer: Lexer<Token> = Token::lexer(pkl_code);

    // Print the parsed data
    for token in lexer {
        println!("{:?}", token);
    }
}
