use winnow::{token::take_while, PResult, Parser};

pub fn identifier<'source>(input: &mut &'source str) -> PResult<&'source str> {
    take_while(0.., (('0'..='9'), ('A'..='F'), ('a'..='f'), '_')).parse_next(input)
}
