use crate::{
    parser::{
        types::{parse_type, PklType},
        utils::{
            assert_token_eq, expect_token, hashmap_while_not_token2, list_while_not_token2,
            parse_identifier, retrieve_next_token,
        },
        value::parse_value,
    },
    prelude::{ParsingError, ParsingResult, PklLexer, PklToken, PklValue},
};

use super::Statement;

#[derive(Debug, PartialEq, Clone)]
/// A struct representing the type of a `ClassDeclaration`.
pub enum ClassType {
    Abstract,
    Open,
    None,
}

#[derive(Debug, PartialEq, Clone)]
/// A struct representing the type of a `ClassField`.
pub enum FieldType {
    Fixed,
    Hidden,
    Local,
    None,
}

#[derive(Debug, PartialEq, Clone)]
/// A struct representing an argument, that is a field or a method, of a `ClassDeclaration`.
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
        _ => return Err(ParsingError::unexpected(lexer, "'{'".to_owned())),
    };

    let fields = hashmap_while_not_token2(
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
) -> ParsingResult<(
    (&'source str, ClassArgument<'source>),
    Option<PklToken<'source>>,
)> {
    match token {
        PklToken::Hidden => {
            let name = parse_identifier(lexer)?;
            expect_token(lexer, PklToken::Colon)?;
            let (value, next_token) = parse_type(lexer, None)?;

            Ok((
                (
                    name,
                    ClassArgument::Field {
                        value,
                        _type: FieldType::Hidden,
                    },
                ),
                next_token,
            ))
        }
        PklToken::Fixed => {
            let name = parse_identifier(lexer)?;
            expect_token(lexer, PklToken::Colon)?;
            let (value, next_token) = parse_type(lexer, None)?;

            Ok((
                (
                    name,
                    ClassArgument::Field {
                        value,
                        _type: FieldType::Hidden,
                    },
                ),
                next_token,
            ))
        }
        PklToken::Local => {
            let name = parse_identifier(lexer)?;
            expect_token(lexer, PklToken::Colon)?;
            let (value, next_token) = parse_type(lexer, None)?;

            Ok((
                (
                    name,
                    ClassArgument::Field {
                        value,
                        _type: FieldType::Local,
                    },
                ),
                next_token,
            ))
        }
        PklToken::Identifier(name) | PklToken::IllegalIdentifier(name) => {
            expect_token(lexer, PklToken::Colon)?;
            let (value, next_token) = parse_type(lexer, None)?;

            Ok((
                (
                    name,
                    ClassArgument::Field {
                        value,
                        _type: FieldType::None,
                    },
                ),
                next_token,
            ))
        }
        PklToken::Function => {
            let name = match retrieve_next_token(lexer)? {
                PklToken::FunctionCall(name) => name,
                _ => return Err(ParsingError::unexpected(lexer, "function name".to_owned())),
            };

            let args = list_while_not_token2(
                lexer,
                PklToken::Comma,
                PklToken::CloseParenthesis,
                &parse_fn_arg,
            )?;
            expect_token(lexer, PklToken::Colon)?;

            let (return_type, next_token) = parse_type(lexer, None)?;

            assert_token_eq(lexer, next_token, PklToken::EqualSign)?;

            let next_token = retrieve_next_token(lexer)?;
            let return_value = parse_value(lexer, next_token)?;

            Ok((
                (
                    name,
                    ClassArgument::Method {
                        args,
                        return_type,
                        return_value,
                    },
                ),
                None,
            ))
        }
        _ => Err(ParsingError::unexpected(
            lexer,
            "field or method definition".to_string(),
        )),
    }
}

fn parse_fn_arg<'a>(
    lexer: &mut PklLexer<'a>,
    opt_token: Option<PklToken<'a>>,
) -> ParsingResult<((&'a str, PklType<'a>), Option<PklToken<'a>>)> {
    let name = if opt_token.is_some() {
        match opt_token.unwrap() {
            PklToken::Identifier(id) => id,
            _ => return Err(ParsingError::expected_identifier(lexer)),
        }
    } else {
        parse_identifier(lexer)?
    };
    expect_token(lexer, PklToken::Colon)?;
    let (value, next_token) = parse_type(lexer, None)?;

    Ok(((name, value), next_token))
}
