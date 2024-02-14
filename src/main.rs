use logos::{Lexer, Logos};
use std::{env, fs, time::Instant};

use pkl_fast::lexer::PklToken;

use crate::parser::parse;
mod parser;

fn main() -> miette::Result<()> {
    let args: Vec<String> = env::args().collect();
    let target_file = match args.get(1) {
        Some(value) => {
            println!("Last argument: {}", value);
            value.to_owned()
        }
        None => {
            println!("No target file provided. Run the command `cargo run <file_name>`");
            return Ok(());
        }
    };

    let pkl_code = fs::read_to_string(target_file).unwrap_or("".to_owned());
    let start = Instant::now();
    let lexer: Lexer<PklToken> = PklToken::lexer(&pkl_code);

    let statements = parse(lexer)?;
    println!("{:?}", statements);

    let end = Instant::now();
    println!("Total time: {} microseconds", (end - start).as_micros());

    Ok(())
}
