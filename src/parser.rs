use crate::{lexer::PklToken, parse_identifier, parse_string};
use std::ops::{Deref, DerefMut, Range};

#[cfg(feature = "hashbrown_support")]
use hashbrown::Hashmap as HashMap;
use logos::{Lexer, Span};
#[cfg(not(feature = "hashbrown_support"))]
use std::collections::HashMap;

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

/* ANCHOR: statements */
/// Represent any valid Pkl value.
#[derive(Debug, PartialEq, Clone)]
pub enum PklStatement<'a> {
    /// A constant/variable statement
    Constant(&'a str, PklExpr<'a>, Range<usize>),

    /// Am import statement:
    /// - name: &str
    /// - local name: Option<&str>
    Import(&'a str, Option<&'a str>, Range<usize>),
}
/* ANCHOR_END: statements */

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

#[derive(Debug, PartialEq, Clone)]
pub struct FuncCall<'a>(pub Identifier<'a>, pub Vec<PklExpr<'a>>, pub Range<usize>);

impl<'a> FuncCall<'a> {
    pub fn span(&self) -> Range<usize> {
        self.2.to_owned()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExprMember<'a> {
    Identifier(Identifier<'a>),
    FuncCall(FuncCall<'a>),
}

impl<'a> ExprMember<'a> {
    pub fn span(&self) -> Range<usize> {
        match self {
            ExprMember::Identifier(id) => id.span(),
            ExprMember::FuncCall(fn_call) => fn_call.span(),
        }
    }
}

impl<'a> From<Identifier<'a>> for ExprMember<'a> {
    fn from(value: Identifier<'a>) -> Self {
        ExprMember::Identifier(value)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PklExpr<'a> {
    Identifier(Identifier<'a>),
    Value(AstPklValue<'a>),
    MemberExpression(Box<PklExpr<'a>>, ExprMember<'a>, Range<usize>),
    FuncCall(FuncCall<'a>),
}

impl<'a> PklExpr<'a> {
    /// This function MUST be called only when we are sure `PklExpr` is a `AstPklValue`
    pub fn extract_value(self) -> AstPklValue<'a> {
        match self {
            Self::Value(v) => v,
            _ => unreachable!(),
        }
    }

    pub fn span(&self) -> Range<usize> {
        match self {
            Self::Value(v) => v.span(),
            Self::Identifier(Identifier(_, span)) => span.to_owned(),
            Self::MemberExpression(_, _, span) => span.to_owned(),
            Self::FuncCall(FuncCall(_, _, span)) => span.to_owned(),
        }
    }
}

impl<'a> From<AstPklValue<'a>> for PklExpr<'a> {
    fn from(value: AstPklValue<'a>) -> Self {
        PklExpr::Value(value)
    }
}
impl<'a> From<(&'a str, Range<usize>)> for PklExpr<'a> {
    fn from((value, indexes): (&'a str, Range<usize>)) -> Self {
        PklExpr::Identifier(Identifier(value, indexes))
    }
}

/// Represent any valid Pkl value.
#[derive(Debug, PartialEq, Clone)]
pub enum AstPklValue<'a> {
    Null(Range<usize>),

    /// true or false.
    Bool(bool, Range<usize>),
    /// Any floating point number.
    Float(f64, Range<usize>),
    /// Any Integer.
    Int(i64, Range<usize>),

    /// Any quoted string.
    String(&'a str, Range<usize>),
    /// Any multiline string.
    MultiLineString(&'a str, Range<usize>),

    /// An object.
    Object(ExprHash<'a>),

    /// An object.
    List(Vec<PklExpr<'a>>, Range<usize>),

    /// A Class instance.
    ClassInstance(&'a str, ExprHash<'a>, Range<usize>),

    /// ### An object amending another object:
    /// - First comes the name of the amended object,
    /// - Then the additional values
    /// - Finally the range
    ///
    /// **Corresponds to:**
    /// ```pkl
    /// x = (other_object) {
    ///     prop = "attribute"
    /// }
    /// ```
    AmendingObject(&'a str, ExprHash<'a>, Range<usize>),

    /// ### An amended object.
    /// Different from `AmendingObject`
    ///
    /// **Corresponds to:**
    /// ```pkl
    /// x = {
    ///    prop = "attribute"
    /// } {
    ///    other_prop = "other_attribute"
    /// }
    /// ```
    AmendedObject(Box<AstPklValue<'a>>, ExprHash<'a>, Range<usize>),
}

impl<'a> Deref for PklStatement<'a> {
    type Target = PklExpr<'a>;

    fn deref(&self) -> &Self::Target {
        match self {
            PklStatement::Constant(_, value, _) => value,
            PklStatement::Import(_, _, _) => unreachable!(),
        }
    }
}
impl<'a> DerefMut for PklStatement<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            PklStatement::Constant(_, value, _) => value,
            PklStatement::Import(_, _, _) => unreachable!(),
        }
    }
}
impl<'a> PklStatement<'a> {
    pub fn span(&self) -> Range<usize> {
        match self {
            PklStatement::Constant(_, _, rng) => rng.clone(),
            PklStatement::Import(_, _, rng) => rng.clone(),
        }
    }
    pub fn is_import(&self) -> bool {
        match self {
            PklStatement::Import(_, _, _) => true,
            _ => false,
        }
    }
    pub fn is_constant(&self) -> bool {
        match self {
            PklStatement::Constant(_, _, _) => true,
            _ => false,
        }
    }
}

impl<'a> From<ExprHash<'a>> for AstPklValue<'a> {
    fn from(value: ExprHash<'a>) -> Self {
        AstPklValue::Object(value)
    }
}
impl<'a> From<ExprHash<'a>> for PklExpr<'a> {
    fn from(value: ExprHash<'a>) -> Self {
        PklExpr::Value(value.into())
    }
}

impl<'a> AstPklValue<'a> {
    pub fn span(&self) -> Range<usize> {
        match self {
            AstPklValue::Int(_, rng)
            | AstPklValue::Bool(_, rng)
            | AstPklValue::Float(_, rng)
            | AstPklValue::Object((_, rng))
            | AstPklValue::AmendingObject(_, _, rng)
            | AstPklValue::AmendedObject(_, _, rng)
            | AstPklValue::ClassInstance(_, _, rng)
            | AstPklValue::String(_, rng)
            | AstPklValue::List(_, rng)
            | AstPklValue::MultiLineString(_, rng)
            | AstPklValue::Null(rng) => rng.clone(),
        }
    }
}

fn parse_basic_id<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<Identifier<'a>> {
    parse_identifier!(lexer)
}

fn parse_member_expr_member<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<ExprMember<'a>> {
    let start = lexer.span().end;

    while let Some(token) = lexer.next() {
        match token {
            Ok(PklToken::Identifier(id)) | Ok(PklToken::IllegalIdentifier(id)) => {
                return Ok(Identifier(id, start..lexer.span().end).into())
            }
            Ok(PklToken::FunctionCall(id)) => {
                return Ok(ExprMember::FuncCall(parse_fn_call(
                    lexer,
                    Identifier(id, lexer.span()),
                )?))
            }
            Ok(PklToken::NewLine) | Ok(PklToken::Space) => {
                // Skip spaces and newlines
            }
            Err(e) => {
                return Err((e.to_string(), lexer.span()));
            }
            _ => {
                return Err((
                    "unexpected token, expected identifier".to_owned(),
                    lexer.span(),
                ));
            }
        }
    }

    Err((
        "expected identifier but got nothing".to_owned(),
        lexer.span(),
    ))
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
                if let Some(PklStatement::Import(_, optional_name, rng)) = statements.last_mut() {
                    if optional_name.is_none() {
                        fn optional_id<'a>(
                            lexer: &mut Lexer<'a, PklToken<'a>>,
                        ) -> PklResult<Identifier<'a>> {
                            parse_identifier!(
                                lexer,
                                "unexpected token here, expected an identifier (context: import)"
                            )
                        }

                        let Identifier(other_name, other_rng) = optional_id(lexer)?;
                        *rng = rng.start..other_rng.end;
                        *optional_name = Some(other_name);
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
            Ok(PklToken::Dot) => {
                if let Some(PklStatement::Constant(_, value, _)) = statements.last_mut() {
                    let expr_member = parse_member_expr_member(lexer)?;
                    let expr_start = value.span().start;
                    let expr_end = expr_member.span().end;

                    *value = PklExpr::MemberExpression(
                        Box::new(value.clone()),
                        expr_member.into(),
                        expr_start..expr_end,
                    );
                } else {
                    return Err((
                        "unexpected token here (context: global)".to_owned(),
                        lexer.span(),
                    ));
                }
            }
            Ok(PklToken::OpenBrace) => {
                if let Some(PklStatement::Constant(_, value, rng)) = statements.last_mut() {
                    match value {
                        PklExpr::Value(AstPklValue::Object(_))
                        | PklExpr::Value(AstPklValue::AmendingObject(_, _, _))
                        | PklExpr::Value(AstPklValue::AmendedObject(_, _, _)) => {
                            let (new_object, object_span) = parse_object(lexer)?;
                            let end = object_span.end;
                            *value = AstPklValue::AmendedObject(
                                Box::new(value.clone().extract_value()),
                                (new_object, object_span),
                                rng.start..end,
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

fn parse_expr<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklExpr<'a>> {
    while let Some(token) = lexer.next() {
        match token {
            Ok(PklToken::Bool(b)) => return Ok(AstPklValue::Bool(b, lexer.span()).into()),
            Ok(PklToken::Identifier(id)) | Ok(PklToken::IllegalIdentifier(id)) => {
                return Ok(PklExpr::Identifier(Identifier(id, lexer.span())))
            }
            Ok(PklToken::New) => return parse_class_instance(lexer),
            Ok(PklToken::FunctionCall(fn_name)) => {
                let fn_call = parse_fn_call(lexer, Identifier(fn_name, lexer.span()))?;

                return Ok(PklExpr::FuncCall(fn_call));
            }
            Ok(PklToken::Null) => return Ok(AstPklValue::Null(lexer.span()).into()),
            Ok(PklToken::Int(i))
            | Ok(PklToken::OctalInt(i))
            | Ok(PklToken::HexInt(i))
            | Ok(PklToken::BinaryInt(i)) => return Ok(AstPklValue::Int(i, lexer.span()).into()),
            Ok(PklToken::Float(f)) => return Ok(AstPklValue::Float(f, lexer.span()).into()),
            Ok(PklToken::String(s)) => return Ok(AstPklValue::String(s, lexer.span()).into()),
            Ok(PklToken::MultiLineString(s)) => {
                return Ok(AstPklValue::MultiLineString(s, lexer.span()).into())
            }
            Ok(PklToken::OpenParen) => return Ok(parse_amended_object(lexer)?.into()),
            Ok(PklToken::Space)
            | Ok(PklToken::NewLine)
            | Ok(PklToken::DocComment(_))
            | Ok(PklToken::LineComment(_))
            | Ok(PklToken::MultilineComment(_)) => continue,
            Err(e) => return Err((e.to_string(), lexer.span())),
            _ => return Err(("unexpected token here".to_owned(), lexer.span())),
        }
    }
    Err(("empty expressions are not allowed".to_owned(), lexer.span()))
}

fn parse_fn_call<'a>(
    lexer: &mut Lexer<'a, PklToken<'a>>,
    id: Identifier<'a>,
) -> PklResult<FuncCall<'a>> {
    let start = lexer.span().start;
    let mut values: Vec<PklExpr> = Vec::with_capacity(5);
    let mut is_comma = true;

    loop {
        match lexer.next() {
            Some(Ok(token)) => match token {
                PklToken::Dot if !is_comma => {
                    if let Some(last) = values.last_mut() {
                        let expr_member = parse_member_expr_member(lexer)?;
                        let expr_start = last.span().start;
                        let expr_end = expr_member.span().end;

                        *last = PklExpr::MemberExpression(
                            Box::new(last.clone()),
                            expr_member.into(),
                            expr_start..expr_end,
                        );
                    } else {
                        return Err(("unexpected token '.'".to_owned(), lexer.span()));
                    }
                }
                PklToken::Comma if !is_comma => {
                    is_comma = true;
                }
                PklToken::CloseParen => {
                    let end = lexer.span().end;
                    return Ok(FuncCall(id, values.into(), start..end));
                }
                PklToken::Space
                | PklToken::NewLine
                | PklToken::DocComment(_)
                | PklToken::LineComment(_)
                | PklToken::MultilineComment(_) => {}
                PklToken::Bool(b) if is_comma => {
                    values.push(AstPklValue::Bool(b, lexer.span()).into());
                    is_comma = false;
                }
                PklToken::Identifier(id) | PklToken::IllegalIdentifier(id) if is_comma => {
                    values.push(PklExpr::Identifier(Identifier(id, lexer.span())));
                    is_comma = false;
                }
                PklToken::New if is_comma => {
                    values.push(parse_class_instance(lexer)?);
                    is_comma = false;
                }
                PklToken::FunctionCall(fn_name) if is_comma => {
                    values.push(PklExpr::FuncCall(parse_fn_call(
                        lexer,
                        Identifier(fn_name, lexer.span()),
                    )?));

                    is_comma = false;
                }
                PklToken::Int(i)
                | PklToken::OctalInt(i)
                | PklToken::HexInt(i)
                | PklToken::BinaryInt(i)
                    if is_comma =>
                {
                    values.push(AstPklValue::Int(i, lexer.span()).into());
                    is_comma = false;
                }
                PklToken::Float(f) if is_comma => {
                    values.push(AstPklValue::Float(f, lexer.span()).into());
                    is_comma = false;
                }
                PklToken::String(s) if is_comma => {
                    values.push(AstPklValue::String(s, lexer.span()).into());
                    is_comma = false;
                }
                PklToken::MultiLineString(s) if is_comma => {
                    values.push(AstPklValue::MultiLineString(s, lexer.span()).into());
                    is_comma = false;
                }
                _ => return Err(("unexpected token here".to_owned(), lexer.span())),
            },
            Some(Err(e)) => return Err((e.to_string(), lexer.span())),
            None => return Err(("Missing list close parenthesis".to_owned(), lexer.span())),
        }
    }
}

fn parse_object<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<ExprHash<'a>> {
    let start = lexer.span().start;
    let mut hashmap = HashMap::with_capacity(8); // Assuming typical small object size
    let mut expect_new_entry = true;

    while let Some(token) = lexer.next() {
        match token {
            Ok(PklToken::Identifier(id)) | Ok(PklToken::IllegalIdentifier(id)) => {
                if !expect_new_entry {
                    return Err((
                        "unexpected token here (context: object), expected newline or comma"
                            .to_owned(),
                        lexer.span(),
                    ));
                }

                let value = parse_const_expr(lexer)?;
                expect_new_entry = matches!(value, PklExpr::Value(AstPklValue::Object((_, _))));
                hashmap.insert(id, value);
            }
            Ok(PklToken::NewLine) | Ok(PklToken::Comma) => {
                expect_new_entry = true;
            }
            Ok(PklToken::Space) => {}
            Ok(PklToken::CloseBrace) => {
                let end = lexer.span().end;
                return Ok((hashmap, start..end));
            }
            Err(e) => return Err((e.to_string(), lexer.span())),
            _ => {
                return Err((
                    "unexpected token here (context: object)".to_owned(),
                    lexer.span(),
                ));
            }
        }
    }

    Err(("Missing object close brace".to_owned(), lexer.span()))
}

fn parse_amended_object<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<AstPklValue<'a>> {
    let start = lexer.span().start;

    let amended_object_name = match lexer.next() {
        Some(Ok(PklToken::Identifier(id))) | Some(Ok(PklToken::IllegalIdentifier(id))) => {
            if let Some(Ok(PklToken::CloseParen)) = lexer.next() {
                id
            } else {
                return Err((
                    "expected close parenthesis (context: amended_object)".to_owned(),
                    lexer.span(),
                ));
            }
        }
        Some(Err(e)) => return Err((e.to_string(), lexer.span())),
        _ => {
            return Err((
                "expected identifier here (context: amended_object)".to_owned(),
                lexer.span(),
            ));
        }
    };

    while let Some(token) = lexer.next() {
        match token {
            Ok(PklToken::Space) | Ok(PklToken::NewLine) => continue,
            Ok(PklToken::OpenBrace) => {
                let object = parse_object(lexer)?;
                let end = lexer.span().end;
                return Ok(AstPklValue::AmendingObject(
                    amended_object_name,
                    object,
                    start..end,
                ));
            }
            Err(e) => return Err((e.to_string(), lexer.span())),
            _ => {
                return Err((
                    "expected open brace here (context: amended_object)".to_owned(),
                    lexer.span(),
                ));
            }
        }
    }

    Err((
        "expected open brace (context: amended_object)".to_owned(),
        lexer.span(),
    ))
}

/* ANCHOR: const */
/// Parse a token stream into a Pkl const Statement.
fn parse_const<'a>(
    lexer: &mut Lexer<'a, PklToken<'a>>,
    name: &'a str,
) -> PklResult<PklStatement<'a>> {
    let start = lexer.span().start;
    let value = parse_const_expr(lexer)?;
    let end = lexer.span().end;

    Ok(PklStatement::Constant(name, value, start..end))
}
/* ANCHOR_END: const */

/// Function called after 'import' keyword.
fn parse_import<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklStatement<'a>> {
    let start = lexer.span().start;

    fn parse_value<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<(&'a str, Range<usize>)> {
        parse_string!(
            lexer,
            "unexpected token here, expected an import value (context: import)",
            "Missing import value"
        )
    }

    let (value, rng) = parse_value(lexer)?;

    return Ok(PklStatement::Import(value, None, start..rng.end));
}

/* ANCHOR: const_expr */
/// Parse a token stream into a Pkl Expr after an identifier.
fn parse_const_expr<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklExpr<'a>> {
    loop {
        match lexer.next() {
            Some(Ok(PklToken::EqualSign)) => {
                return parse_expr(lexer);
            }
            Some(Ok(PklToken::OpenBrace)) => {
                return Ok(parse_object(lexer)?.into());
            }
            Some(Ok(PklToken::Space))
            | Some(Ok(PklToken::NewLine))
            | Some(Ok(PklToken::DocComment(_)))
            | Some(Ok(PklToken::LineComment(_)))
            | Some(Ok(PklToken::MultilineComment(_))) => {
                // Continue the loop to process the next token
                continue;
            }
            Some(Err(e)) => {
                return Err((e.to_string(), lexer.span()));
            }
            Some(_) => {
                return Err((
                    "unexpected token here (context: constant)".to_owned(),
                    lexer.span(),
                ));
            }
            None => {
                return Err(("Expected '='".to_owned(), lexer.span()));
            }
        }
    }
}
/* ANCHOR_END: const_expr */

fn parse_class_instance<'a>(lexer: &mut Lexer<'a, PklToken<'a>>) -> PklResult<PklExpr<'a>> {
    let start = lexer.span().start;

    let class_name = loop {
        match lexer.next() {
            Some(Ok(PklToken::Identifier(id))) | Some(Ok(PklToken::IllegalIdentifier(id))) => {
                break id
            }
            Some(Ok(PklToken::Space))
            | Some(Ok(PklToken::NewLine))
            | Some(Ok(PklToken::DocComment(_)))
            | Some(Ok(PklToken::LineComment(_)))
            | Some(Ok(PklToken::MultilineComment(_))) => continue,
            Some(Err(e)) => return Err((e.to_string(), lexer.span())),
            Some(_) => {
                return Err((
                    "unexpected token here (context: class_instance), expected identifier"
                        .to_owned(),
                    lexer.span(),
                ));
            }
            None => return Err(("Expected identifier".to_owned(), lexer.span())),
        }
    };

    loop {
        match lexer.next() {
            Some(Ok(PklToken::OpenBrace)) => {
                return Ok(AstPklValue::ClassInstance(
                    class_name,
                    parse_object(lexer)?,
                    start..lexer.span().end,
                )
                .into());
            }
            Some(Ok(PklToken::Space))
            | Some(Ok(PklToken::NewLine))
            | Some(Ok(PklToken::DocComment(_)))
            | Some(Ok(PklToken::LineComment(_)))
            | Some(Ok(PklToken::MultilineComment(_))) => {
                // Continue the loop to process the next token
                continue;
            }
            Some(Err(e)) => {
                return Err((e.to_string(), lexer.span()));
            }
            Some(_) => {
                return Err((
                    "unexpected token here (context: constant)".to_owned(),
                    lexer.span(),
                ));
            }
            None => {
                return Err(("Expected '='".to_owned(), lexer.span()));
            }
        }
    }
}
