use crate::compiler::lexer::span::Span;

use super::{expression::Expr, type_ast::TypeAST};

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
        ty: TypeAST,
    },
}

impl Decl {
    pub fn variable(name: String, right: Box<Expr>, ty: TypeAST, span: Span) -> Decl {
        Decl {
            span,
            kind: DeclKind::Variable { name, right, ty },
        }
    }
}
