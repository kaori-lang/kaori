use crate::compiler::lexer::span::Span;

use super::{expression::Expr, type_ast::TypeAST};

#[derive(Debug)]
pub struct Decl {
    span: Span,
    kind: DeclKind,
}

#[derive(Debug)]
pub enum DeclKind {
    Variable {
        identifier: Box<Expr>,
        right: Box<Expr>,
        ty: TypeAST,
    },
}

impl Decl {
    pub fn variable(identifier: Box<Expr>, right: Box<Expr>, ty: TypeAST, span: Span) -> Decl {
        Decl {
            span,
            kind: DeclKind::Variable {
                identifier,
                right,
                ty,
            },
        }
    }
}
