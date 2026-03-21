use crate::lexer::span::Span;

use super::{expr::Expr, node::Node, node_id::NodeId, ty::Ty};

#[derive(Debug)]
pub struct Decl {
    pub id: NodeId,
    pub span: Span,
    pub kind: DeclKind,
}

#[derive(Debug)]
pub enum DeclKind {
    Variable {
        right: Box<Expr>,
        ty: Option<Ty>,
    },
    Function {
        parameters: Vec<Parameter>,
        body: Vec<Node>,
        return_ty: Option<Ty>,
    },
    Struct {
        fields: Vec<Field>,
    },
}

#[derive(Debug)]
pub struct Parameter {
    pub id: NodeId,
    pub span: Span,
    pub ty: Ty,
}

#[derive(Debug)]
pub struct Field {
    pub id: NodeId,
    pub span: Span,
    pub ty: Ty,
}

impl Parameter {
    pub fn new(id: NodeId, ty: Ty, span: Span) -> Parameter {
        Parameter { id, span, ty }
    }
}

impl Field {
    pub fn new(id: NodeId, ty: Ty, span: Span) -> Field {
        Field { id, span, ty }
    }
}

impl Decl {
    pub fn struct_(id: NodeId, fields: Vec<Field>, span: Span) -> Decl {
        Decl {
            id,
            span,

            kind: DeclKind::Struct { fields },
        }
    }

    pub fn variable(id: NodeId, right: Expr, ty: Option<Ty>, span: Span) -> Decl {
        Decl {
            id,
            span,

            kind: DeclKind::Variable {
                right: Box::new(right),
                ty,
            },
        }
    }

    pub fn function(
        id: NodeId,
        parameters: Vec<Parameter>,
        body: Vec<Node>,
        return_ty: Option<Ty>,
        span: Span,
    ) -> Decl {
        Decl {
            id,
            span,
            kind: DeclKind::Function {
                parameters,
                body,
                return_ty,
            },
        }
    }
}
