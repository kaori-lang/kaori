use crate::frontend::scanner::span::Span;

use super::{ast_node::AstNode, expr::Expr, ty::Ty};

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
        ty: Ty,
    },
    Function {
        name: String,
        parameters: Vec<Decl>,
        body: Vec<AstNode>,
        return_ty: Ty,
    },
    Struct {
        name: String,
        fields: Vec<Decl>,
    },
    Parameter {
        name: String,
        ty: Ty,
    },
    Field {
        name: String,
        ty: Ty,
    },
}

impl Decl {
    pub fn struct_(name: String, fields: Vec<Decl>, span: Span) -> Decl {
        Decl {
            span,
            kind: DeclKind::Struct { name, fields },
        }
    }

    pub fn variable(name: String, right: Expr, ty: Ty, span: Span) -> Decl {
        Decl {
            span,
            kind: DeclKind::Variable {
                name,
                right: Box::new(right),
                ty,
            },
        }
    }

    pub fn parameter(name: String, ty: Ty, span: Span) -> Decl {
        Decl {
            span,
            kind: DeclKind::Parameter { name, ty },
        }
    }

    pub fn field(name: String, ty: Ty, span: Span) -> Decl {
        Decl {
            span,
            kind: DeclKind::Field { name, ty },
        }
    }

    pub fn function(
        name: String,
        parameters: Vec<Decl>,
        body: Vec<AstNode>,
        return_ty: Ty,
        span: Span,
    ) -> Decl {
        Decl {
            span,
            kind: DeclKind::Function {
                name,
                parameters,
                body,
                return_ty,
            },
        }
    }
}
