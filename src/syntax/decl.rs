use crate::lexer::span::Span;

use super::{ast_id::AstId, ast_node::AstNode, expr::Expr, ty::Ty};

#[derive(Debug)]
pub struct Decl {
    pub id: AstId,
    pub span: Span,
    pub kind: DeclKind,
    pub ty: Ty,
}

#[derive(Debug)]
pub enum DeclKind {
    Variable {
        name: String,
        right: Expr,
    },
    Function {
        name: String,
        parameters: Vec<Decl>,
        body: Vec<AstNode>,
    },
    Struct {
        name: String,
        fields: Vec<Decl>,
    },
    Parameter {
        name: String,
    },
    Field {
        name: String,
    },
}

impl Decl {
    pub fn struct_(name: String, fields: Vec<Decl>, ty: Ty, span: Span) -> Decl {
        Decl {
            id: AstId::default(),
            span,
            ty,
            kind: DeclKind::Struct { name, fields },
        }
    }

    pub fn variable(name: String, right: Expr, ty: Ty, span: Span) -> Decl {
        Decl {
            id: AstId::default(),
            span,
            ty,
            kind: DeclKind::Variable { name, right },
        }
    }

    pub fn parameter(name: String, ty: Ty, span: Span) -> Decl {
        Decl {
            id: AstId::default(),
            span,
            ty,
            kind: DeclKind::Parameter { name },
        }
    }

    pub fn field(name: String, ty: Ty, span: Span) -> Decl {
        Decl {
            id: AstId::default(),
            span,
            ty,
            kind: DeclKind::Field { name },
        }
    }

    pub fn function(
        name: String,
        parameters: Vec<Decl>,
        body: Vec<AstNode>,
        ty: Ty,
        span: Span,
    ) -> Decl {
        Decl {
            id: AstId::default(),
            span,
            ty,
            kind: DeclKind::Function {
                name,
                parameters,
                body,
            },
        }
    }
}
