use crate::{lexer::PklToken, PklResult};
use expr::{member_expr::parse_member_expr_member, object::parse_object, PklExpr};
use hashbrown::HashMap;
use logos::{Lexer, Source};
use statement::{
    import::Import, module::Module, parse_stmt, property::Property, typealias::TypeAlias,
    PklStatement,
};
use std::ops::Range;
use types::{parse_type, AstPklType};
use utils::parse_id;
use value::AstPklValue;

pub mod expr;
pub mod statement;
pub mod types;
pub mod value;

mod utils;

pub type ExprHash<'a> = (HashMap<&'a str, PklExpr<'a>>, Range<usize>);

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier<'a>(pub &'a str, pub Range<usize>);

impl<'a> Identifier<'a> {
    pub fn span(&self) -> Range<usize> {
        self.1.to_owned()
    }
    pub fn value(&self) -> &str {
        self.0
    }
}

/// Parse a token stream into a Pkl statement.
pub fn parse_pkl<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<Vec<PklStatement<'a>>> {
    let mut statements = Vec::with_capacity(16); // Assuming typical file size for preallocation
    let mut is_newline = true;

    while let Some(token) = lexer.next() {
        match token {
            Ok(PklToken::Union) => {
                if let Some(PklStatement::TypeAlias(TypeAlias {
                    refering_type,
                    span,
                    ..
                })) = statements.last_mut().map(PklStatement::inner_mut)
                {
                    let second_type = parse_type(lexer)?;

                    span.end = second_type.span().end;
                    *refering_type = AstPklType::Union(
                        Box::new(refering_type.to_owned()),
                        Box::new(second_type),
                    );

                    is_newline = false;
                } else {
                    return Err((
                        "unexpected token here (context: global)".to_owned(),
                        lexer.span(),
                    )
                        .into());
                }
            }

            Ok(PklToken::As) => {
                if let Some(PklStatement::Import(Import {
                    local_name, span, ..
                })) = statements.last_mut().map(PklStatement::inner_mut)
                {
                    if local_name.is_none() {
                        let Identifier(other_name, other_rng) = parse_id(lexer)?;
                        *span = span.start..other_rng.end;
                        *local_name = Some(other_name);
                    } else {
                        return Err((
                            "Import statement already has an 'as' close (context: import)"
                                .to_owned(),
                            lexer.span(),
                        )
                            .into());
                    }
                } else {
                    return Err((
                        "unexpected token here (context: global)".to_owned(),
                        lexer.span(),
                    )
                        .into());
                }
            }

            Ok(PklToken::Dot) => {
                if let Some(PklStatement::Property(Property { value, .. })) =
                    statements.last_mut().map(PklStatement::inner_mut)
                {
                    let expr_member = parse_member_expr_member(lexer)?;
                    let expr_start = value.span().start;
                    let expr_end = expr_member.span().end;

                    *value = PklExpr::MemberExpression(
                        Box::new(value.clone()),
                        expr_member,
                        expr_start..expr_end,
                    );
                } else if let Some(PklStatement::ModuleClause(Module {
                    full_name, span, ..
                })) = statements.last_mut().map(PklStatement::inner_mut)
                {
                    let other_component = parse_id(lexer)?;
                    let new_span = full_name.1.start..other_component.1.end;
                    *full_name = Identifier(
                        lexer.source().slice(new_span.to_owned()).unwrap(),
                        new_span.to_owned(),
                    );
                    *span = new_span;
                } else {
                    return Err((
                        "unexpected token here (context: global)".to_owned(),
                        lexer.span(),
                    )
                        .into());
                }
            }
            Ok(PklToken::OpenBrace) => {
                if let Some(PklStatement::Property(Property { value, span, .. })) =
                    statements.last_mut().map(PklStatement::inner_mut)
                {
                    match value {
                        PklExpr::Value(AstPklValue::Object(_))
                        | PklExpr::Value(AstPklValue::AmendingObject(_, _, _))
                        | PklExpr::Value(AstPklValue::AmendedObject(_, _, _)) => {
                            let (new_object, object_span) = parse_object(lexer)?;
                            let end = object_span.end;
                            *value = AstPklValue::AmendedObject(
                                Box::new(value.clone().extract_value()),
                                (new_object, object_span),
                                span.start..end,
                            )
                            .into();
                        }
                        _ => {
                            return Err((
                                "unexpected token here (context: global)".to_owned(),
                                lexer.span(),
                            )
                                .into());
                        }
                    }
                } else {
                    return Err((
                        "unexpected token here (context: global)".to_owned(),
                        lexer.span(),
                    )
                        .into());
                }
            }
            Ok(PklToken::Space)
            | Ok(PklToken::DocComment(_))
            | Ok(PklToken::LineComment(_))
            | Ok(PklToken::MultilineComment(_)) => {
                // Skip spaces and comments
                continue;
            }
            Ok(PklToken::NewLine) => {
                is_newline = true;
                continue;
            }
            // parses any statement
            Ok(token) if is_newline => {
                let stmt = parse_stmt(lexer, Some(token))?;
                statements.push(stmt);
                is_newline = false;
            }
            Err(e) => return Err((e.to_string(), lexer.span()).into()),
            _ => {
                return Err((
                    "unexpected token here (context: global)".to_owned(),
                    lexer.span(),
                )
                    .into());
            }
        }
    }

    Ok(statements)
}
