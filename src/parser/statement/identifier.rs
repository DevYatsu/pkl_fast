use crate::{
    parser::{types::parse_type, utils::retrieve_next_token, value::parse_value},
    prelude::{ParsingError, ParsingResult, PklLexer, PklToken},
};

use super::Statement;

pub fn parse_identifier_statement<'source>(
    lexer: &mut PklLexer<'source>,
    name: &'source str,
) -> ParsingResult<Statement<'source>> {
    let token = retrieve_next_token(lexer)?;

    let statement = match token {
        PklToken::EqualSign => {
            let value = parse_value(lexer)?;

            Statement::VariableDeclaration {
                name,
                value,
                optional_type: None,
            }
        }
        PklToken::OpenBracket => {
            // object definition

            todo!()
        }
        PklToken::Colon => {
            // expect a type

            let variable_type = parse_type(lexer)?;
            let mut statement = parse_identifier_statement(lexer, name)?;

            if let Statement::VariableDeclaration {
                ref mut optional_type,
                ..
            } = statement
            {
                // statement is certain to be a variable declaration
                *optional_type = Some(variable_type);
            }

            statement
        }
        _ => Err(ParsingError::unexpected(lexer))?,
    };

    Ok(statement)
}
