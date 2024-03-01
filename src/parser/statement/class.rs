use crate::{
    parser::{
        types::{parse_type, PklType},
        utils::{
            expect_token, hashmap_while_not_token0, list_while_not_token0, parse_identifier,
            retrieve_next_token,
        },
        value::parse_value,
    },
    prelude::{ParsingError, ParsingResult, PklLexer, PklToken, PklValue},
};

use super::Statement;

#[derive(Debug, PartialEq, Clone)]
pub enum ClassType {
    Abstract,
    Open,
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub enum FieldType {
    Fixed,
    Hidden,
    Local,
    None,
}

#[derive(Debug, PartialEq, Clone)]
/// A struct representing an field of a @ModuleInfo annotation
pub enum ClassArgument<'a> {
    Field {
        value: PklType<'a>,
        _type: FieldType,
    },

    Method {
        args: Vec<(&'a str, PklType<'a>)>,
        return_type: PklType<'a>,
        return_value: PklValue<'a>,
    },
}

pub fn parse_class_declaration<'source>(
    lexer: &mut PklLexer<'source>,
    _type: ClassType,
) -> ParsingResult<Statement<'source>> {
    let name = parse_identifier(lexer)?;

    let token = retrieve_next_token(lexer)?;

    let extends = match token {
        PklToken::Extends => {
            let value = parse_identifier(lexer)?;
            expect_token(lexer, PklToken::OpenBracket)?;

            Some(value)
        }
        PklToken::OpenBracket => None,
        _ => return Err(ParsingError::unexpected(lexer)),
    };

    let fields = hashmap_while_not_token0(
        lexer,
        PklToken::NewLine,
        PklToken::CloseBracket,
        &parse_class_field,
    )?;

    Ok(Statement::ClassDeclaration {
        name,
        extends,
        _type,
        fields,
    })
}

pub fn parse_class_field<'source>(
    lexer: &mut PklLexer<'source>,
    token: PklToken<'source>,
) -> ParsingResult<(&'source str, ClassArgument<'source>)> {
    match token {
        PklToken::Hidden => {
            let name = parse_identifier(lexer)?;
            expect_token(lexer, PklToken::Colon)?;
            let value = parse_type(lexer)?;

            Ok((
                name,
                ClassArgument::Field {
                    value,
                    _type: FieldType::Hidden,
                },
            ))
        }
        PklToken::Fixed => {
            let name = parse_identifier(lexer)?;
            expect_token(lexer, PklToken::Colon)?;
            let value = parse_type(lexer)?;

            Ok((
                name,
                ClassArgument::Field {
                    value,
                    _type: FieldType::Fixed,
                },
            ))
        }
        PklToken::Local => {
            let name = parse_identifier(lexer)?;
            expect_token(lexer, PklToken::Colon)?;
            let value = parse_type(lexer)?;

            Ok((
                name,
                ClassArgument::Field {
                    value,
                    _type: FieldType::Local,
                },
            ))
        }
        PklToken::Identifier(name) | PklToken::IllegalIdentifier(name) => {
            expect_token(lexer, PklToken::Colon)?;
            let value = parse_type(lexer)?;

            Ok((
                name,
                ClassArgument::Field {
                    value,
                    _type: FieldType::None,
                },
            ))
        }
        PklToken::Function => {
            let name = match retrieve_next_token(lexer)? {
                PklToken::FunctionCall(name) => name,
                _ => return Err(ParsingError::unexpected(lexer)),
            };

            let args = list_while_not_token0(
                lexer,
                PklToken::Comma,
                PklToken::CloseParenthesis,
                &parse_fn_arg,
            )?;
            expect_token(lexer, PklToken::Colon)?;

            let return_type = parse_type(lexer)?;

            expect_token(lexer, PklToken::EqualSign)?;
            let next_token = retrieve_next_token(lexer)?;
            let return_value = parse_value(lexer, next_token)?;

            Ok((
                name,
                ClassArgument::Method {
                    args,
                    return_type,
                    return_value,
                },
            ))
        }
        _ => Err(ParsingError::unexpected(lexer)),
    }
}

fn parse_fn_arg<'a>(lexer: &mut PklLexer<'a>) -> ParsingResult<(&'a str, PklType<'a>)> {
    let name = parse_identifier(lexer)?;
    expect_token(lexer, PklToken::Colon)?;
    let value = parse_type(lexer)?;

    Ok((name, value))
}
