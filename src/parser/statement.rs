use std::thread::current;

use super::{expr::PklExpr, utils::parse_any_token, Identifier};
use crate::{lexer::PklToken, PklResult};
use amends::{parse_amends_clause, Amends};
use boxed::{parse_const, parse_fixed, parse_local};
use class::{parse_class_declaration, ClassDeclaration, ClassKind};
use extends::{parse_extends_clause, Extends};
use import::{parse_import, Import};
use logos::{Lexer, Span};
use module::{parse_module_clause, Module};
use property::{parse_property, Property};
use typealias::{parse_typealias, TypeAlias};

pub mod amends;
mod boxed;
pub mod class;
pub mod extends;
pub mod import;
pub mod module;
pub mod property;
pub mod typealias;

/// Represent any valid Pkl Statement.
#[derive(Debug, PartialEq, Clone)]
pub enum PklStatement<'a> {
    /// A constant/variable statement
    Property(Property<'a>),

    /// Am import statement
    Import(Import<'a>),

    /// A class declaration
    Class(ClassDeclaration<'a>),

    /// A typealias
    TypeAlias(TypeAlias<'a>),

    /// A module clause, used to declare a module name
    ModuleClause(Module<'a>),

    /// An amends clause, it's like extending
    /// but then you can't create any variable
    /// that is not declared in the amended
    /// module
    AmendsClause(Amends<'a>),

    /// An extends clause, literally it's like importing
    /// but directly in the main context,
    /// not in a variable creating in the context
    /// containing the import values.
    ExtendsClause(Extends<'a>),

    /// A local Statement
    Local(Box<PklStatement<'a>>, Span),
    /// A const Statement
    Const(Box<PklStatement<'a>>, Span),
    /// A fixed Statement
    Fixed(Box<PklStatement<'a>>, Span),
}

impl<'a> PklStatement<'a> {
    pub fn span(&self) -> Span {
        match self {
            PklStatement::Property(Property { span, .. }) => span.clone(),
            PklStatement::Import(Import { span, .. }) => span.clone(),
            PklStatement::Class(ClassDeclaration { span, .. }) => span.clone(),
            PklStatement::TypeAlias(TypeAlias { span, .. }) => span.clone(),
            PklStatement::ModuleClause(Module { span, .. }) => span.clone(),
            PklStatement::AmendsClause(Amends { span, .. }) => span.clone(),
            PklStatement::ExtendsClause(Extends { span, .. }) => span.clone(),
            PklStatement::Local(_, span) => span.clone(),
            PklStatement::Const(_, span) => span.clone(),
            PklStatement::Fixed(_, span) => span.clone(),
        }
    }

    pub fn inner(&self) -> &Self {
        match self {
            PklStatement::Local(x, _) => x.inner(),
            PklStatement::Const(x, _) => x.inner(),
            PklStatement::Fixed(x, _) => x.inner(),
            _ => self,
        }
    }
    pub fn inner_mut(&mut self) -> &mut Self {
        match self {
            PklStatement::Local(x, _) => x.inner_mut(),
            PklStatement::Const(x, _) => x.inner_mut(),
            PklStatement::Fixed(x, _) => x.inner_mut(),
            _ => self,
        }
    }
    pub fn is_import(&self) -> bool {
        matches!(self, &PklStatement::Import { .. })
    }
    pub fn is_constant(&self) -> bool {
        matches!(self, &PklStatement::Property { .. })
    }
    pub fn is_class_declaration(&self) -> bool {
        matches!(self, &PklStatement::Class { .. })
    }
}

/// Parses a `PklStatement`.
pub fn parse_stmt<'a>(
    lexer: &mut Lexer<'a, PklToken<'a>>,
    current_token: Option<PklToken<'a>>,
) -> PklResult<PklStatement<'a>> {
    let token = match current_token {
        Some(t) => t,
        None => parse_any_token(lexer)?,
    };

    match token {
        PklToken::TypeAlias => parse_typealias(lexer),
        PklToken::Import => parse_import(lexer),
        PklToken::Extends => parse_extends_clause(lexer),
        PklToken::Amends => parse_amends_clause(lexer),

        PklToken::Class => parse_class_declaration(lexer, ClassKind::default()),
        PklToken::OpenClass => parse_class_declaration(lexer, ClassKind::Open),
        PklToken::AbstractClass => parse_class_declaration(lexer, ClassKind::Abstract),

        PklToken::Module => parse_module_clause(lexer, false),
        PklToken::OpenModule => parse_module_clause(lexer, true),

        PklToken::Fixed => parse_fixed(lexer),
        PklToken::Const => parse_const(lexer),
        PklToken::Local => parse_local(lexer),

        PklToken::Identifier(id) | PklToken::IllegalIdentifier(id) => {
            parse_property(lexer, Identifier(id, lexer.span()))
        }

        _ => {
            return Err((
                "unexpected token here (context: global), expected statement".to_owned(),
                lexer.span(),
            )
                .into());
        }
    }
}
