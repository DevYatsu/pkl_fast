use super::PklStatement;
use crate::lexer::PklToken;
use crate::parser::types::{parse_type_until, AstPklType};
use crate::parser::utils::{parse_id, parse_id_as_str, parse_multispaces_until, parse_open_brace};
use crate::PklResult;
use hashbrown::HashMap;
use logos::{Lexer, Span};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum ClassKind {
    #[default]
    Classical,
    Open,
    Abstract,
}

#[derive(Debug, Clone, Eq, Hash)]
pub struct ClassField<'a> {
    pub name: &'a str,
    pub kind: FieldKind,
    span: Span,
}

impl<'a> ClassField<'a> {
    pub fn new(name: &'a str, kind: FieldKind, span: Span) -> Self {
        Self { name, kind, span }
    }

    pub fn span(&self) -> Span {
        self.span.to_owned()
    }
}

impl<'a> PartialEq for ClassField<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum FieldKind {
    #[default]
    Classical,
    Hidden,
}

/// Parse a token stream into a Pkl class Statement.
///
/// Parser called after a 'class' keyword.
pub fn parse_class_declaration<'a>(
    lexer: &mut Lexer<'a, PklToken<'a>>,
    class_type: ClassKind,
) -> PklResult<PklStatement<'a>> {
    let start = lexer.span().start;
    let name = parse_id(lexer)?;
    let mut extends = None;

    let token = parse_open_brace_or_extends(lexer)?;

    match token {
        PklToken::OpenBrace => (),
        PklToken::Extends => {
            extends = Some(parse_id(lexer)?);
            parse_open_brace(lexer)?;
        }
        _ => unreachable!(),
    }

    let fields = parse_fields(lexer)?;
    let end = lexer.span().end;

    Ok(PklStatement::Class {
        name,
        _type: class_type,
        extends,
        fields,
        span: start..end,
    })
}

fn parse_open_brace_or_extends<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklToken<'a>> {
    parse_multispaces_until!(lexer, PklToken::OpenBrace, PklToken::Extends)
}

fn parse_fields<'a>(
    lexer: &mut Lexer<'a, PklToken<'a>>,
) -> PklResult<HashMap<ClassField<'a>, AstPklType<'a>>> {
    let mut hashmap = HashMap::new();

    let mut key: Option<ClassField<'a>> = None;
    let mut _type: Option<AstPklType<'a>> = None;

    loop {
        let token = lexer.next();

        if token.is_none() {
            return Err(("Unexpected end of input".to_owned(), lexer.span()));
        }

        match token.unwrap() {
            Ok(PklToken::Identifier(id)) | Ok(PklToken::IllegalIdentifier(id)) => {
                if let (Some(k), Some(t)) = (key.take(), _type.take()) {
                    hashmap.insert(k, t);
                }
                key = Some(ClassField::new(id, FieldKind::default(), lexer.span()))
            }
            Ok(PklToken::Hidden) if key.is_none() => {
                let id = parse_id_as_str(lexer)?;
                key = Some(ClassField::new(id, FieldKind::Hidden, lexer.span()))
            }

            Ok(PklToken::Colon) if key.is_some() & _type.is_none() => {
                let parsed_type = parse_type_until(lexer, PklToken::NewLine)?;
                _type = Some(parsed_type);
            }

            Ok(PklToken::Union) if _type.is_some() => {
                let other_type = parse_type_until(lexer, PklToken::NewLine)?;
                _type = Some(AstPklType::Union(
                    Box::new(_type.take().unwrap()),
                    Box::new(other_type),
                ))
            }

            Ok(PklToken::CloseBrace) => {
                if let (Some(k), Some(t)) = (key.take(), _type.take()) {
                    hashmap.insert(k, t);
                }
                break;
            }

            Ok(PklToken::Space)
            | Ok(PklToken::NewLine)
            | Ok(PklToken::DocComment(_))
            | Ok(PklToken::LineComment(_))
            | Ok(PklToken::MultilineComment(_)) => continue,
            Err(e) => return Err((e.to_string(), lexer.span())),
            _ => return Err(("unexpected token here".to_owned(), lexer.span())),
        }
    }

    Ok(hashmap)
}
