use std::fmt::Display;

use crate::{
    parser::{
        expression::{
            basic::parse_basic_expr, complex::parse_complex_expr, parse_expr, Expression,
        },
        types::{parse_type, PklType},
        utils::{assert_token_eq, expect_token, parse_identifier, retrieve_next_token},
    },
    prelude::{ParsingError, ParsingResult, PklLexer, PklToken},
};

use super::{parse_object, PklValue};
#[derive(Debug, PartialEq, Clone)]
pub enum MappingField<'a> {
    Expression(Expression<'a>),
    LocalVariable {
        name: &'a str,
        _type: Option<PklType<'a>>,
        value: Expression<'a>,
    },
    DefaultObject(Expression<'a>),
    AmendingElement {
        index: Expression<'a>,
        value: PklValue<'a>,
    },
    Pair {
        key: Expression<'a>,
        value: Expression<'a>,
    },
}

pub fn parse_mapping_field<'source>(
    lexer: &mut PklLexer<'source>,
    next_token: PklToken<'source>,
) -> ParsingResult<(MappingField<'source>, Option<PklToken<'source>>)> {
    match next_token {
        PklToken::Local => {
            let name = parse_identifier(lexer)?;

            match retrieve_next_token(lexer)? {
                PklToken::EqualSign => {
                    let (value, next) = parse_expr(lexer, None)?;
                    Ok((
                        MappingField::LocalVariable {
                            name,
                            _type: None,
                            value,
                        },
                        next,
                    ))
                }
                PklToken::Colon => {
                    let (_type, opt_token) = parse_type(lexer, None)?;
                    assert_token_eq(lexer, opt_token, PklToken::EqualSign)?;
                    let (value, next) = parse_expr(lexer, Some(next_token))?;
                    Ok((
                        MappingField::LocalVariable {
                            name,
                            _type: Some(_type),
                            value,
                        },
                        next,
                    ))
                }
                _ => Err(ParsingError::unexpected(lexer, "'=' or ':'".to_owned())),
            }
        }
        PklToken::Default => {
            expect_token(lexer, PklToken::OpenBracket)?;
            let (value, token) = parse_object(lexer, None)?;

            Ok((MappingField::DefaultObject(Expression::Value(value)), token))
        }
        PklToken::OpenBrace => parse_mapping_variable(lexer),
        _ => {
            let (expr, next) = parse_expr(lexer, Some(next_token))?;
            Ok((MappingField::Expression(expr), next))
        }
    }
}

// parser called whenever a '[' was found
pub fn parse_mapping_variable<'source>(
    lexer: &mut PklLexer<'source>,
) -> ParsingResult<(MappingField<'source>, Option<PklToken<'source>>)> {
    let (key, next_token) = parse_expr(lexer, None)?;
    assert_token_eq(lexer, next_token, PklToken::CloseBrace)?;

    match retrieve_next_token(lexer)? {
        PklToken::OpenBracket => {
            let (value, token) = parse_object(lexer, None)?;
            Ok((
                MappingField::Pair {
                    value: Expression::Value(value),
                    key,
                },
                token,
            ))
        }
        PklToken::EqualSign => {
            match retrieve_next_token(lexer)? {
                PklToken::OpenParenthesis => {
                    let (expr, opt_token) = parse_basic_expr(lexer, None)?;

                    match opt_token {
                        Some(PklToken::CloseParenthesis) => match expr {
                            Expression::ListIndexing { indexed, indexer } => {
                                if indexed == "this" {
                                    expect_token(lexer, PklToken::OpenBracket)?;
                                    let (value, token) = parse_object(lexer, None)?;

                                    Ok((
                                        MappingField::AmendingElement {
                                            index: *indexer,
                                            value,
                                        },
                                        token,
                                    ))
                                } else {
                                    let (expr, next) = parse_complex_expr(
                                        lexer,
                                        Expression::Parenthesised(Box::new(
                                            Expression::ListIndexing { indexed, indexer },
                                        )),
                                        None,
                                    )?;
                                    Ok((MappingField::Expression(expr), next))
                                }
                            }
                            _ => {
                                let (expr, next) = parse_complex_expr(
                                    lexer,
                                    Expression::Parenthesised(Box::new(expr)),
                                    None,
                                )?;
                                Ok((MappingField::Expression(expr), next))
                            }
                        },

                        Some(_) => {
                            // first call to parse expr inside parenthesis
                            let (expr, next) = parse_complex_expr(lexer, expr, opt_token)?;
                            assert_token_eq(lexer, next, PklToken::CloseParenthesis)?;
                            // second call to parse following expr if there is one
                            let (expr, next) = parse_complex_expr(lexer, expr, None)?;

                            Ok((MappingField::Expression(expr), next))
                        }
                        _ => Err(ParsingError::eof(lexer)),
                    }
                }
                token => {
                    let (value, next) = parse_expr(lexer, Some(token))?;
                    Ok((MappingField::Pair { value, key }, next))
                }
            }
        }
        _ => Err(ParsingError::unexpected(lexer, "'=' or '{'".to_owned())),
    }
}

impl<'a> Display for MappingField<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MappingField::Expression(expr) => write!(f, "{expr}"),
            MappingField::LocalVariable { name, value, _type } => {
                if _type.is_some() {
                    write!(f, "local {name}: {} = {value}", _type.clone().unwrap())
                } else {
                    write!(f, "local {name} = {value}")
                }
            }
            MappingField::DefaultObject(x) => write!(f, "default {x}"),
            MappingField::AmendingElement { index, value } => write!(f, "(this[{index}]) {value}"),
            MappingField::Pair { key, value } => match value {
                Expression::Value(v) => match v {
                    PklValue::Object { .. } => write!(f, "[{key}] {value}"),
                    _ => write!(f, "[{key}] = {value}"),
                },
                _ => write!(f, "[{key}] = {value}"),
            },
        }
    }
}
