use lexer::Token;
use logos::{Lexer, Logos};
use std::{env, fs, time::Instant};

mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let target_file = if args.len() > 1 {
        let first_argument = &args[1];
        println!("First argument: {}", first_argument);
        args[1].to_owned()
    } else {
        println!("No target file provided. Run `cargo run <file_name>`");
        return;
    };

    let pkl_code = fs::read_to_string(target_file).unwrap_or("".to_owned());
    let start = Instant::now();
    let mut lexer: Lexer<Token> = Token::lexer(&pkl_code);
    let end = Instant::now();

    while let Some(token) = lexer.next() {
        println!("{:?}", token);
        let statement = match token {
            Ok(Token::Import) => {
                parser::import::parse_import(&mut lexer)
                
            }
            Ok(Token::GlobbedImport) => {
                parser::import::parse_globbed_import(&mut lexer)
            }
            _ => continue,
        };

        println!("{:?}", statement);
    }

    println!("Total time: {} microseconds", (end - start).as_micros())
}
