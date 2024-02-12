use super::ParsingResult;
use logos::Lexer;
use pkl_fast::lexer::PklToken;

pub fn jump_spaces_and_then<'source, Output>(
    lexer: &mut Lexer<'source, PklToken>,
    predicate: &dyn Fn(&mut Lexer<'source, PklToken>) -> ParsingResult<Output>,
) -> ParsingResult<Output> {
    loop {
        if let Some(token) = lexer.next() {
            if let Ok(PklToken::Space) = token {
                continue;
            }

            return predicate(lexer);
        };
    }
}
