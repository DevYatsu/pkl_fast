#[macro_export]
macro_rules! expect_tokens {
    // Base case: no tokens left to match
    ($lexer:expr,) => { Ok(()) };

    // Recursive case: match the first token and continue with the rest
    ($lexer:expr, $($tokens:expr),+) => {
        match $lexer.next() {
            Some(Err(e)) => Err(ParsingError::lexing($lexer, e)),
            Some(Ok(token)) if $( token == $tokens )||+ => Ok(()),
            Some(_) => Err(ParsingError::unexpected($lexer)),
            None => Err(ParsingError::eof($lexer)),
        }
    };
}
