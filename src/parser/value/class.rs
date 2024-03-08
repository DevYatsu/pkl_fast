use std::fmt::Display;

use crate::{
    parser::{
        expression::{parse_expr, Expression},
        utils::{expect_token, list_while_not_token3, retrieve_next_token},
    },
    prelude::{ParsingError, ParsingResult, PklLexer, PklToken, PklValue},
};

use super::{
    listing::parse_listing_field,
    mapping::{parse_mapping_field, parse_mapping_variable, MappingField},
    parse_object,
};

#[derive(Debug, PartialEq, Clone)]
pub enum ClassField<'a> {
    Expression(Expression<'a>),
    VariableDeclaration {
        name: &'a str,
        value: Expression<'a>,
    },
    MappingField(MappingField<'a>),
}

/// Function called to parse a class instance, we assume that 'new' was already found
pub fn parse_class_instance<'source>(
    lexer: &mut PklLexer<'source>,
) -> ParsingResult<PklValue<'source>> {
    let next_token = retrieve_next_token(lexer)?;

    let name = match next_token {
        PklToken::Identifier(value) => {
            expect_token(lexer, PklToken::OpenBracket)?;
            match value {
                "Listing" => {
                    let values = list_while_not_token3(
                        lexer,
                        PklToken::NewLine,
                        PklToken::CloseBracket,
                        &parse_listing_field,
                    )?;

                    return Ok(PklValue::Listing(values));
                }
                "Mapping" => {
                    let values = list_while_not_token3(
                        lexer,
                        PklToken::NewLine,
                        PklToken::CloseBracket,
                        &parse_mapping_field,
                    )?;

                    return Ok(PklValue::Mapping(values));
                }
                _ => (),
            }

            Some(value)
        }
        PklToken::OpenBracket => None,
        _ => return Err(ParsingError::unexpected(lexer, "class instance".to_owned())),
    };

    let arguments = list_while_not_token3(
        lexer,
        PklToken::NewLine,
        PklToken::CloseBracket,
        &parse_class_instance_field,
    )?;

    Ok(PklValue::ClassInstance { name, arguments })
}

fn parse_class_instance_field<'source>(
    lexer: &mut PklLexer<'source>,
    token: PklToken<'source>,
) -> ParsingResult<(ClassField<'source>, Option<PklToken<'source>>)> {
    match token {
        PklToken::Identifier(name) | PklToken::IllegalIdentifier(name) => {
            let next_token = retrieve_next_token(lexer)?;

            match next_token {
                PklToken::EqualSign => {
                    let (value, next_token) = parse_expr(lexer, None)?;
                    Ok((ClassField::VariableDeclaration { name, value }, next_token))
                }
                PklToken::OpenBracket => {
                    let value = parse_object(lexer, None)?;

                    Ok((
                        ClassField::VariableDeclaration {
                            name,
                            value: Expression::Value(value),
                        },
                        None,
                    ))
                }
                _ => Err(ParsingError::unexpected(lexer, "'=' or '{'".to_owned())),
            }
        }

        PklToken::OpenBrace => {
            let (field, next) = parse_mapping_variable(lexer)?;

            Ok((ClassField::MappingField(field), next))
        }
        token => {
            // try parsing an expression, the instance might be a listing
            let (expr, next) = parse_expr(lexer, Some(token))?;

            Ok((ClassField::Expression(expr), next))
        }
    }
}

impl<'a> Display for ClassField<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClassField::Expression(e) => write!(f, "{e}"),
            ClassField::VariableDeclaration { name, value } => write!(f, "{name} = {value}"),
            ClassField::MappingField(m) => write!(f, "{m}"),
        }
    }
}
