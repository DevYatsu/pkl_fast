use std::fmt;

use crate::{
    parser::{
        expression::{parse_expr, Expression},
        utils::{
            assert_token_eq, expect_token, list_while_not_token3, parse_identifier,
            parse_opt_newlines, retrieve_next_token,
        },
    },
    prelude::{ParsingError, ParsingResult, PklLexer, PklToken, PklValue},
};

#[derive(Debug, PartialEq, Clone)]
pub enum ObjectField<'a> {
    Variable {
        name: &'a str,
        value: Expression<'a>,
    },

    WhenGenerator {
        condition: Expression<'a>,
        members: Vec<ObjectField<'a>>,
        _else: Option<Vec<ObjectField<'a>>>,
    },

    ForGenerator {
        key: Option<&'a str>,
        value: &'a str,
        iterable: &'a str,
        members: Vec<ObjectField<'a>>,
    },

    Spread(&'a str),
    NullableSpread(&'a str),

    AmendedValue {
        index: Expression<'a>,
        value: PklValue<'a>,
    },
}

/// Function called to parse an object, we assume that '{' was already found
pub fn parse_object<'source>(
    lexer: &mut PklLexer<'source>,
    opt_amended_object: Option<&'source str>,
) -> ParsingResult<PklValue<'source>> {
    let values = list_while_not_token3(
        lexer,
        PklToken::NewLine,
        PklToken::CloseBracket,
        &parse_block,
    )?;

    Ok(PklValue::Object {
        values,
        amended_by: opt_amended_object,
    })
}

pub fn parse_block<'source>(
    lexer: &mut PklLexer<'source>,
    token: PklToken<'source>,
) -> ParsingResult<(ObjectField<'source>, Option<PklToken<'source>>)> {
    match token {
        PklToken::Identifier(name) | PklToken::IllegalIdentifier(name) => {
            let next_token = retrieve_next_token(lexer)?;

            let (value, next_token) = match next_token {
                PklToken::EqualSign => {
                    let (value, next_token) = parse_expr(lexer, None)?;

                    (value, next_token)
                }
                PklToken::OpenBracket => {
                    // we sould see whether or not we add to this object that its parent object is amended
                    let value = parse_object(lexer, None)?;

                    (Expression::Value(value), None)
                }
                _ => return Err(ParsingError::unexpected(lexer, "'='".to_owned())),
            };

            Ok((ObjectField::Variable { name, value }, next_token))
        }
        PklToken::OpenBrace => {
            let (expr, next_token) = parse_expr(lexer, None)?;
            assert_token_eq(lexer, next_token, PklToken::CloseBrace)?;
            expect_token(lexer, PklToken::OpenBracket)?;
            let value = parse_object(lexer, None)?;

            Ok((ObjectField::AmendedValue { index: expr, value }, None))
        }
        PklToken::SpreadSyntax => {
            let ident = parse_identifier(lexer)?;
            Ok((ObjectField::Spread(ident), None))
        }
        PklToken::NullableSpreadSyntax => {
            let ident = parse_identifier(lexer)?;
            Ok((ObjectField::NullableSpread(ident), None))
        }
        PklToken::When => {
            expect_token(lexer, PklToken::OpenParenthesis)?;
            let (condition, next) = parse_opt_newlines(lexer, &parse_expr)?;
            assert_token_eq(lexer, next, PklToken::CloseParenthesis)?;
            expect_token(lexer, PklToken::OpenBracket)?;

            let members = list_while_not_token3(
                lexer,
                PklToken::NewLine,
                PklToken::CloseBracket,
                &parse_block,
            )?;

            let next_token = retrieve_next_token(lexer)?;

            match next_token {
                PklToken::Else => {
                    expect_token(lexer, PklToken::OpenBracket)?;
                    let _else = list_while_not_token3(
                        lexer,
                        PklToken::NewLine,
                        PklToken::CloseBracket,
                        &parse_block,
                    )?;

                    Ok((
                        ObjectField::WhenGenerator {
                            condition,
                            members,
                            _else: Some(_else),
                        },
                        None,
                    ))
                }
                _ => Ok((
                    ObjectField::WhenGenerator {
                        condition,
                        members,
                        _else: None,
                    },
                    Some(next_token),
                )),
            }
        }
        PklToken::For => {
            expect_token(lexer, PklToken::OpenParenthesis)?;
            let ident1 = parse_identifier(lexer)?;
            let next = retrieve_next_token(lexer)?;

            match next {
                PklToken::Comma => {
                    let value = parse_identifier(lexer)?;
                    expect_token(lexer, PklToken::In)?;
                    let iterable = parse_identifier(lexer)?;
                    expect_token(lexer, PklToken::CloseParenthesis)?;
                    expect_token(lexer, PklToken::OpenBracket)?;

                    let members = list_while_not_token3(
                        lexer,
                        PklToken::NewLine,
                        PklToken::CloseBracket,
                        &parse_block,
                    )?;

                    Ok((
                        ObjectField::ForGenerator {
                            key: Some(ident1),
                            value,
                            iterable,
                            members,
                        },
                        None,
                    ))
                }
                PklToken::In => {
                    let iterable = parse_identifier(lexer)?;
                    expect_token(lexer, PklToken::CloseParenthesis)?;
                    expect_token(lexer, PklToken::OpenBracket)?;

                    let members = list_while_not_token3(
                        lexer,
                        PklToken::NewLine,
                        PklToken::CloseBracket,
                        &parse_block,
                    )?;

                    Ok((
                        ObjectField::ForGenerator {
                            key: None,
                            value: ident1,
                            iterable,
                            members,
                        },
                        None,
                    ))
                }
                _ => Err(ParsingError::unexpected(lexer, "'in' or ','".to_owned())),
            }
        }
        _ => Err(ParsingError::expected_identifier(lexer)),
    }
}

impl<'a> fmt::Display for ObjectField<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjectField::Variable { name, value } => write!(f, "{name} = {value}"),
            ObjectField::WhenGenerator {
                condition,
                members,
                _else,
            } => {
                write!(f, "when ({condition}) {{\n")?;

                for member in members {
                    write!(f, "{member}\n")?;
                }

                match _else {
                    Some(else_members) => {
                        write!(f, "}} else {{\n")?;

                        for member in else_members {
                            write!(f, "{member}\n")?;
                        }
                        write!(f, "}}")
                    }
                    None => write!(f, "}}"),
                }
            }
            ObjectField::ForGenerator {
                key,
                value,
                iterable,
                members,
            } => {
                write!(f, "for (")?;
                match key {
                    Some(key) => write!(f, "{key}, {value}")?,
                    None => write!(f, "{value}")?,
                };

                write!(f, " in {iterable}) {{\n")?;

                for member in members {
                    write!(f, "{member}\n")?;
                }

                write!(f, "}}")
            }
            ObjectField::Spread(value) => write!(f, "...{value}"),
            ObjectField::NullableSpread(value) => write!(f, "...?{value}"),
            ObjectField::AmendedValue { index, value } => write!(f, "[{index}] {value}"),
        }
    }
}
