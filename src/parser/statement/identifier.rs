use crate::{
    parser::{types::parse_type, value::parse_value},
    prelude::{ParsingError, ParsingResult, PklLexer, PklToken},
};

use super::Statement;

pub fn parse_identifier_statement<'source>(
    lexer: &mut PklLexer<'source>,
    name: &'source str,
) -> ParsingResult<Statement<'source>> {
    let token = lexer.next();

    if token.is_none() {
        return Err(ParsingError::eof(lexer));
    }

    let statement = match token.unwrap() {
        Ok(PklToken::EqualSign) => {
            let value = parse_value(lexer)?;

            Statement::VariableDeclaration {
                name,
                value,
                optional_type: None,
            }
        }
        Ok(PklToken::OpenBracket) => {
            // object definition

            todo!()
        }
        Ok(PklToken::Colon) => {
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
        Err(e) => Err(ParsingError::lexing(lexer, e))?,
        _ => Err(ParsingError::unexpected(lexer))?,
    };

    Ok(statement)
}
