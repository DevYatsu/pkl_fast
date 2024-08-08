use super::{parse_stmt, PklStatement};
use crate::{lexer::PklToken, PklResult};
use logos::Lexer;

pub fn parse_fixed<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklStatement<'a>> {
    let start = lexer.span().start;
    let stmt = parse_stmt(lexer, None)?;
    let end = lexer.span().end;
    Ok(PklStatement::Fixed(Box::new(stmt), start..end))
}
pub fn parse_const<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklStatement<'a>> {
    let start = lexer.span().start;
    let stmt = parse_stmt(lexer, None)?;
    let end = lexer.span().end;
    Ok(PklStatement::Const(Box::new(stmt), start..end))
}
pub fn parse_local<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklStatement<'a>> {
    let start = lexer.span().start;
    let stmt = parse_stmt(lexer, None)?;
    let end = lexer.span().end;
    Ok(PklStatement::Local(Box::new(stmt), start..end))
}
