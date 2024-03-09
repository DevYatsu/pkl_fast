use miette::Diagnostic;
use pkl_fast::parser::parse;
use pkl_fast::{lexer::string::sanitize_code, prelude::lex};
use std::{env, fs, path::PathBuf, time::Instant};
use thiserror::Error;

fn main() -> miette::Result<()> {
    let args: Vec<String> = env::args().collect();
    let target_path = get_target_file(&args)?;
    let pkl_code = fs::read_to_string(&target_path).map_err(|_| ProgramError::InvalidFilePath)?;
    let (pkl_code, updated_code, strings_vec) = sanitize_code(&pkl_code);

    let start = Instant::now();

    let _file_name = target_path
        .file_name()
        .ok_or_else(|| ProgramError::InvalidFilePath)?;
    let _statements = parse(pkl_code, lex(&updated_code), strings_vec)?;

    for s in _statements {
        println!("{:?}", s)
    }

    let end = Instant::now();
    println!("Total time: {} microseconds", (end - start).as_micros());

    Ok(())
}

fn get_target_file(args: &[String]) -> miette::Result<PathBuf> {
    Ok(args
        .get(1)
        .map(|file_path| PathBuf::from(file_path))
        .ok_or_else(|| ProgramError::NoFileArgument)?)
}

#[derive(Error, Diagnostic, Debug)]
pub enum ProgramError {
    #[error("No target file provided")]
    #[diagnostic(
        code(program_error::target_file),
        help("Run the command `cargo run <file_path>`")
    )]
    NoFileArgument,

    #[error("Invalid file path")]
    #[diagnostic(
        code(program_error::file_path),
        help("Ensure the existence of a file at the specified path")
    )]
    InvalidFilePath,
}
