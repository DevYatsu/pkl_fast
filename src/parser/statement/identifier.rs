use crate::{
    parser::{types::PklType, value::parse_value},
    prelude::{ParsingError, ParsingResult, PklLexer, PklToken},
};

use super::Statement;

pub fn parse_identifier_statement<'source>(
    lexer: &mut PklLexer<'source>,
    name: &'source str,
) -> ParsingResult<Statement<'source>> {
    let token = lexer.next();

    if token.is_none() {
        return Err(ParsingError::unexpected(lexer));
    }

    let value = match token.unwrap() {
        Ok(PklToken::EqualSign) => parse_value(lexer)?,
        Ok(PklToken::OpenBracket) => {
            // object definition

            todo!()
        }
        Ok(PklToken::Colon) => {
            // expect a type
            todo!()
        }
        Err(e) => Err(ParsingError::lexing(lexer, e))?,
        _ => Err(ParsingError::unexpected(lexer))?,
    };

    Ok(Statement::VariableDeclaration { name, value, optional_type: None })
}
