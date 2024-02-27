use crate::{
    parser::{operator::parse_equal, types::parse_type},
    prelude::{ParsingError, ParsingResult, PklLexer, PklToken},
};

use super::Statement;
pub fn parse_typealias<'source>(
    lexer: &mut PklLexer<'source>,
) -> ParsingResult<Statement<'source>> {
    let token = lexer.next();

    if let Some(Ok(PklToken::Identifier)) = token {
        let alias: &str = lexer.slice();
        parse_equal(lexer)?;
        let equivalent_type = parse_type(lexer)?;

        Ok(Statement::TypeAlias {
            alias,
            equivalent_type,
        })
    } else {
        if token.is_some() {
            Err(ParsingError::invalid_id(lexer))
        } else {
            Err(ParsingError::eof(lexer))
        }
    }
}
