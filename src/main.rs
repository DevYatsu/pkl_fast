use logos::{Lexer, Logos};
use lexer::Token;

mod parser;
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
    "#;

    let lexer: Lexer<Token> = Token::lexer(pkl_code);

    // Print the parsed data
    for token in lexer {
        println!("{:?}", token);
    }
}
