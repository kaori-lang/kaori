use crate::compiler::lexer::span::Span;

use super::{expression::Expr, r#type::Type};

#[derive(Debug)]
pub struct Decl {
    pub span: Span,
    pub kind: DeclKind,
}

#[derive(Debug)]
pub enum DeclKind {
    Variable {
        name: String,
        right: Box<Expr>,
        ty: Type,
    },
}

impl Decl {
    pub fn variable(name: String, right: Box<Expr>, ty: Type, span: Span) -> Decl {
        Decl {
            span,
            kind: DeclKind::Variable { name, right, ty },
        }
    }
}
