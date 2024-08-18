use pest::{
    iterators::Pairs,
    pratt_parser::{Assoc, Op, PrattParser},
    Parser,
};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "pkl.pest"]
pub struct PklParser;

pub fn parse(src: &str) -> Result<Pairs<Rule>, pest::error::Error<Rule>> {
    let result = PklParser::parse(Rule::file, src)?;

    Ok(result)
}

pub fn pratt() -> PrattParser<Rule> {
    PrattParser::new()
        .op(Op::infix(Rule::null_coalescing, Assoc::Left))
        .op(Op::infix(Rule::comp_equal, Assoc::Left)
            | Op::infix(Rule::and, Assoc::Left)
            | Op::infix(Rule::or, Assoc::Left)
            | Op::infix(Rule::comp_not_equal, Assoc::Left)
            | Op::infix(Rule::comp_greater, Assoc::Left)
            | Op::infix(Rule::comp_greater_equal, Assoc::Left)
            | Op::infix(Rule::comp_less, Assoc::Left)
            | Op::infix(Rule::comp_less_equal, Assoc::Left))
        .op(Op::infix(Rule::is_op, Assoc::Left) | Op::infix(Rule::as_op, Assoc::Left))
        .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::sub, Assoc::Left))
        .op(Op::infix(Rule::mul, Assoc::Left)
            | Op::infix(Rule::modulo, Assoc::Left)
            | Op::infix(Rule::div, Assoc::Left)
            | Op::infix(Rule::div_r, Assoc::Left))
        .op(Op::infix(Rule::pow, Assoc::Right))
        .op(Op::postfix(Rule::non_null))
        .op(Op::prefix(Rule::neg) | Op::prefix(Rule::logical_not))
}
