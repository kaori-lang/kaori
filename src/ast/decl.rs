use crate::lexer::span::Span;

use super::{ast_node::AstNode, expr::Expr, node_id::NodeId, ty::Ty};

#[derive(Debug)]
pub struct Decl {
    pub id: NodeId,
    pub span: Span,
    pub kind: DeclKind,
}

#[derive(Debug)]
pub enum DeclKind {
    Variable {
        name: String,
        right: Expr,
        ty: Option<Ty>,
    },
    Function {
        name: String,
        parameters: Vec<Parameter>,
        body: Vec<AstNode>,
        return_ty: Option<Ty>,
    },
    Struct {
        name: String,
        fields: Vec<Field>,
    },
}

#[derive(Debug)]
pub struct Parameter {
    pub id: NodeId,
    pub span: Span,
    pub name: String,
    pub ty: Ty,
}

#[derive(Debug)]
pub struct Field {
    pub id: NodeId,
    pub span: Span,
    pub name: String,
    pub ty: Ty,
}

impl Parameter {
    pub fn new(name: String, ty: Ty, span: Span) -> Parameter {
        Parameter {
            id: NodeId::default(),
            span,
            name,
            ty,
        }
    }
}

impl Field {
    pub fn new(name: String, ty: Ty, span: Span) -> Field {
        Field {
            id: NodeId::default(),
            span,
            name,
            ty,
        }
    }
}

impl Decl {
    pub fn struct_(name: String, fields: Vec<Field>, span: Span) -> Decl {
        Decl {
            id: NodeId::default(),
            span,

            kind: DeclKind::Struct { name, fields },
        }
    }

    pub fn variable(name: String, right: Expr, ty: Option<Ty>, span: Span) -> Decl {
        Decl {
            id: NodeId::default(),
            span,

            kind: DeclKind::Variable { name, right, ty },
        }
    }

    pub fn function(
        name: String,
        parameters: Vec<Parameter>,
        body: Vec<AstNode>,
        return_ty: Option<Ty>,
        span: Span,
    ) -> Decl {
        Decl {
            id: NodeId::default(),
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
