use crate::{
    parser::{
        operator::parse_equal,
        types::parse_type,
        utils::retrieve_next_token,
        value::{object::extract_amended_object, parse_object, parse_value},
    },
    prelude::{lex, ParsingError, ParsingResult, PklLexer, PklToken},
};

use super::Statement;

pub fn parse_var_statement<'source>(
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
            let value = parse_object(lexer, None)?;

            Statement::VariableDeclaration {
                name,
                value,
                optional_type: None,
            }
        }
        PklToken::Colon => {
            // expect a type

            let variable_type = parse_type(lexer)?;
            parse_equal(lexer)?;
            let value = parse_value(lexer)?;

            Statement::VariableDeclaration {
                name,
                value,
                optional_type: Some(variable_type),
            }
        }
        _ => Err(ParsingError::unexpected(lexer))?,
    };

    Ok(statement)
}
