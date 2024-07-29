use crate::lexer::PklToken;
use expr::{member_expr::parse_member_expr_member, object::parse_object, PklExpr};
use hashbrown::HashMap;
use logos::{Lexer, Source, Span};
use statement::{
    amends::parse_amends_clause,
    class::{parse_class_declaration, ClassKind},
    constant::{parse_const, Constant},
    import::{parse_import, Import},
    module::{parse_module_clause, Module},
    typealias::{parse_typealias, TypeAlias},
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

/// Represents a parsing error in the PKL format.
///
/// A `ParseError` is a tuple consisting of:
///
/// * `String` - A message describing the error.
/// * `Span` - The span in the source where the error occurred.
pub type ParseError = (String, Span);

/// A result type for PKL parsing operations.
///
/// The `PklResult` type is a specialized `Result` type used throughout the PKL parsing code.
/// It represents either a successful result (`T`) or a `ParseError`.
pub type PklResult<T> = std::result::Result<T, ParseError>;

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
            Ok(PklToken::Identifier(id)) | Ok(PklToken::IllegalIdentifier(id)) => {
                if !is_newline {
                    return Err((
                        "unexpected token here (context: global), expected newline".to_owned(),
                        lexer.span(),
                    ));
                }
                let statement = parse_const(lexer, id)?;
                statements.push(statement);
                is_newline = false;
            }
            Ok(PklToken::TypeAlias) => {
                if !is_newline {
                    return Err((
                        "unexpected token here (context: global), expected newline".to_owned(),
                        lexer.span(),
                    ));
                }
                // parses until newline here
                let statement = parse_typealias(lexer)?;
                statements.push(statement);
                is_newline = true;
            }
            Ok(PklToken::Union) => {
                if let Some(PklStatement::TypeAlias(TypeAlias {
                    refering_type,
                    span,
                    ..
                })) = statements.last_mut()
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
                    ));
                }
            }
            Ok(PklToken::Module) => {
                if !is_newline {
                    return Err((
                        "unexpected token here (context: global), expected newline".to_owned(),
                        lexer.span(),
                    ));
                }
                let statement = parse_module_clause(lexer)?;
                statements.push(statement);
                is_newline = false;
            }
            Ok(PklToken::Amends) => {
                if !is_newline {
                    return Err((
                        "unexpected token here (context: global), expected newline".to_owned(),
                        lexer.span(),
                    ));
                }
                let statement = parse_amends_clause(lexer)?;
                statements.push(statement);
                is_newline = false;
            }
            Ok(PklToken::Import) => {
                if !is_newline {
                    return Err((
                        "unexpected token here (context: global), expected newline".to_owned(),
                        lexer.span(),
                    ));
                }
                let statement = parse_import(lexer)?;
                statements.push(statement);
                is_newline = false;
            }

            Ok(PklToken::As) => {
                if let Some(PklStatement::Import(Import {
                    local_name, span, ..
                })) = statements.last_mut()
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
                        ));
                    }
                } else {
                    return Err((
                        "unexpected token here (context: global)".to_owned(),
                        lexer.span(),
                    ));
                }
            }
            Ok(PklToken::AbstractClass) => {
                let stmt = parse_class_declaration(lexer, ClassKind::Abstract)?;
                statements.push(stmt)
            }
            Ok(PklToken::OpenClass) => {
                let stmt = parse_class_declaration(lexer, ClassKind::Open)?;
                statements.push(stmt)
            }
            Ok(PklToken::Class) => {
                let stmt = parse_class_declaration(lexer, ClassKind::default())?;
                statements.push(stmt)
            }

            Ok(PklToken::Dot) => {
                if let Some(PklStatement::Constant(Constant { value, .. })) = statements.last_mut()
                {
                    let expr_member = parse_member_expr_member(lexer)?;
                    let expr_start = value.span().start;
                    let expr_end = expr_member.span().end;

                    *value = PklExpr::MemberExpression(
                        Box::new(value.clone()),
                        expr_member,
                        expr_start..expr_end,
                    );
                } else if let Some(PklStatement::ModuleClause(Module { full_name, span })) =
                    statements.last_mut()
                {
                    let other_component = parse_id(lexer)?;
                    let new_span = span.start..other_component.1.end;
                    *full_name = lexer.source().slice(new_span.to_owned()).unwrap();
                    *span = new_span;
                } else {
                    return Err((
                        "unexpected token here (context: global)".to_owned(),
                        lexer.span(),
                    ));
                }
            }
            Ok(PklToken::OpenBrace) => {
                if let Some(PklStatement::Constant(Constant { value, span, .. })) =
                    statements.last_mut()
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
                            ));
                        }
                    }
                } else {
                    return Err((
                        "unexpected token here (context: global)".to_owned(),
                        lexer.span(),
                    ));
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
            Err(e) => return Err((e.to_string(), lexer.span())),
            _ => {
                return Err((
                    "unexpected token here (context: statement)".to_owned(),
                    lexer.span(),
                ));
            }
        }
    }

    Ok(statements)
}
