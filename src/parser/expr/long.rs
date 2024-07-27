use logos::Lexer;

use crate::{lexer::PklToken, PklResult};

use super::{member_expr::parse_member_expr_member, PklExpr};

/// Keep parsing a longer expressions until the 'or_token'
/// is found.
///
/// This parser skips spaces, comments and newlines.
pub fn parse_long_expression_or<'a>(
    lexer: &mut Lexer<'a, PklToken<'a>>,
    mut base_expr: PklExpr<'a>,
    or_token: PklToken<'a>,
) -> PklResult<PklExpr<'a>> {
    loop {
        let next_token = lexer.next();

        if next_token.is_none() {
            return Err((String::from("Unexpected end of input"), lexer.span()));
        }

        match next_token.unwrap() {
            Ok(token) if or_token == token => {
                break;
            }

            Ok(PklToken::Dot) => {
                let member_expr = parse_member_expr_member(lexer)?;
                let start = base_expr.span().start;

                base_expr = PklExpr::MemberExpression(
                    Box::new(base_expr),
                    member_expr,
                    start..lexer.span().end,
                );
            }

            Ok(PklToken::Space)
            | Ok(PklToken::DocComment(_))
            | Ok(PklToken::LineComment(_))
            | Ok(PklToken::MultilineComment(_))
            | Ok(PklToken::NewLine) => {
                // skip spaces, comments, newlines
                continue;
            }

            // add operators in the future
            t => {
                return Err((
                    format!("Expected token '{or_token:?}' found '{t:?}'"),
                    lexer.span(),
                ))
            }
        };
    }

    Ok(base_expr)
}
