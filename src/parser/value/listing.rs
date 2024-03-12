use std::fmt::Display;

use winnow::{combinator::alt, PResult, Parser};

use crate::parser::{expression::Expression, types::PklType, utils::var::local_variable};

use super::{amending::utils::default_field, object::ObjectField, PklValue};
#[derive(Debug, PartialEq, Clone)]
pub enum ListingField<'a> {
    Expression(Expression<'a>),
    LocalVariable {
        name: &'a str,
        _type: Option<PklType<'a>>,
        value: Expression<'a>,
    },
    DefaultObject(Vec<ObjectField<'a>>),

    /// A dynamic Expression is an expression defined in terms of another element
    /// (see [late binding](https://pkl-lang.org/main/current/language-reference/index.html#late-binding-2)),
    /// that is using `this` keyword.
    DynamicExpression {
        index: Expression<'a>,
        value: PklValue<'a>,
    },
}

pub fn parse_listing_field<'source>(input: &mut &'source str) -> PResult<ListingField<'source>> {
    alt((
        local_variable.map(|(name, _type, value)| ListingField::LocalVariable {
            name,
            _type,
            value,
        }),
        default_field.map(ListingField::DefaultObject),
    ))
    .parse_next(input)

    // match next_token {
    //     PklToken::Local => {
    //         let name = parse_identifier(lexer)?;

    //         match retrieve_next_token(lexer)? {
    //             PklToken::EqualSign => {
    //                 let (value, next) = parse_expr(lexer, None)?;
    //                 Ok((
    //                     ListingField::LocalVariable {
    //                         name,
    //                         _type: None,
    //                         value,
    //                     },
    //                     next,
    //                 ))
    //             }
    //             PklToken::Colon => {
    //                 let (_type, opt_token) = parse_type(lexer, None)?;
    //                 assert_token_eq(lexer, opt_token, PklToken::EqualSign)?;
    //                 let (value, next) = parse_expr(lexer, Some(next_token))?;
    //                 Ok((
    //                     ListingField::LocalVariable {
    //                         name,
    //                         _type: Some(_type),
    //                         value,
    //                     },
    //                     next,
    //                 ))
    //             }
    //             _ => Err(ParsingError::unexpected(lexer, "'=' or ':'".to_owned())),
    //         }
    //     }
    //     PklToken::Default => {
    //         expect_token(lexer, PklToken::OpenBracket)?;
    //         let (value, token) = parse_object(lexer, None)?;

    //         Ok((ListingField::DefaultObject(Expression::Value(value)), token))
    //     }
    //     PklToken::OpenParenthesis => {
    //         let (expr, opt_token) = parse_basic_expr(lexer, None)?;

    //         match opt_token {
    //             Some(PklToken::CloseParenthesis) => match expr {
    //                 Expression::ListIndexing { indexed, indexer } => {
    //                     if indexed == "this" {
    //                         expect_token(lexer, PklToken::OpenBracket)?;
    //                         let (value, token) = parse_object(lexer, None)?;

    //                         Ok((
    //                             ListingField::AmendingElement {
    //                                 index: *indexer,
    //                                 value,
    //                             },
    //                             token,
    //                         ))
    //                     } else {
    //                         let (expr, next) = parse_complex_expr(
    //                             lexer,
    //                             Expression::Parenthesised(Box::new(Expression::ListIndexing {
    //                                 indexed,
    //                                 indexer,
    //                             })),
    //                             None,
    //                         )?;
    //                         Ok((ListingField::Expression(expr), next))
    //                     }
    //                 }
    //                 _ => {
    //                     let (expr, next) = parse_complex_expr(
    //                         lexer,
    //                         Expression::Parenthesised(Box::new(expr)),
    //                         None,
    //                     )?;
    //                     Ok((ListingField::Expression(expr), next))
    //                 }
    //             },

    //             Some(_) => {
    //                 // first call to parse expr inside parenthesis
    //                 let (expr, next) = parse_complex_expr(lexer, expr, opt_token)?;
    //                 assert_token_eq(lexer, next, PklToken::CloseParenthesis)?;
    //                 // second call to parse following expr if there is one
    //                 let (expr, next) = parse_complex_expr(lexer, expr, None)?;

    //                 Ok((ListingField::Expression(expr), next))
    //             }
    //             _ => Err(ParsingError::eof(lexer, "a closing parenthesis")),
    //         }
    //     }
    //     _ => {
    //         let (expr, next) = parse_expr(lexer, Some(next_token))?;
    //         Ok((ListingField::Expression(expr), next))
    //     }
    // }
}

impl<'a> Display for ListingField<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListingField::Expression(expr) => write!(f, "{expr}"),
            ListingField::LocalVariable { name, value, _type } => {
                if _type.is_some() {
                    write!(f, "local {name}: {} = {value}", _type.clone().unwrap())
                } else {
                    write!(f, "local {name} = {value}")
                }
            }
            ListingField::DefaultObject(fields) => {
                write!(f, "default {{\n")?;
                for field in fields {
                    write!(f, "\t{field},\n");
                }
                write!(f, "}}")
            }
            ListingField::DynamicExpression { index, value } => {
                write!(f, "(this[{index}]) {value}")
            }
        }
    }
}

impl<'a> From<Expression<'a>> for ListingField<'a> {
    fn from(value: Expression<'a>) -> Self {
        ListingField::Expression(value)
    }
}
