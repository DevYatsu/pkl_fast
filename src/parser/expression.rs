use crate::{
    parser::utils::retrieve_next_token,
    prelude::{ParsingError, PklToken},
};

use self::fn_call::parse_fn_call_arguments;

use super::{operator::Operator, value::PklValue, ParsingResult, PklLexer};

mod fn_call;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    Value(PklValue<'a>),
    Identifier(&'a str),
    FunctionCall {
        func_name: &'a str,
        args: Vec<Expression<'a>>,
    },

    Operation {
        operator: Operator,
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },
}

pub fn parse_expr<'source>(lexer: &mut PklLexer<'source>) -> ParsingResult<Expression<'source>> {
    let token = retrieve_next_token(lexer)?;

    match token {
        PklToken::Identifier(ident) => Ok(Expression::Identifier(ident)),
        PklToken::FunctionCall(func_name) => {
            let args = parse_fn_call_arguments(lexer)?;
            Ok(Expression::FunctionCall { func_name, args })
        }

        _ => return Err(ParsingError::unexpected(lexer)),
    }
}
