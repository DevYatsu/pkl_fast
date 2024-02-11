use std::{fs, time::Instant};

use lexer::Token;
use logos::{Lexer, Logos};

mod lexer;

fn main() {
    let pkl_code = fs::read_to_string("test.pkl").unwrap_or("".to_owned());
    let start = Instant::now();
    let lexer: Lexer<Token> = Token::lexer(&pkl_code);
    let end = Instant::now();

    println!("lexer {:?}", lexer);
    // Print the parsed data
    for token in lexer {
        println!("{:?}", token);
    }

    println!("Total time: {} microseconds", (end-start).as_micros())
}
