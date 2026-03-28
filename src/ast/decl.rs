use crate::lexer::span::Span;

use super::{expr::Expr, node::Node, node_id::NodeId};

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
    },
    Function {
        name: String,
        parameters: Vec<Parameter>,
        body: Vec<Node>,
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
}

#[derive(Debug)]
pub struct Field {
    pub id: NodeId,
    pub span: Span,
    pub name: String,
}

impl Parameter {
    pub fn new(name: String, span: Span) -> Parameter {
        Parameter {
            id: NodeId::default(),
            span,
            name,
        }
    }
}

impl Field {
    pub fn new(name: String, span: Span) -> Field {
        Field {
            id: NodeId::default(),
            span,
            name,
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

    pub fn variable(name: String, right: Expr, span: Span) -> Decl {
        Decl {
            id: NodeId::default(),
            span,

            kind: DeclKind::Variable { name, right },
        }
    }

    pub fn function(name: String, parameters: Vec<Parameter>, body: Vec<Node>, span: Span) -> Decl {
        Decl {
            id: NodeId::default(),
            span,
            kind: DeclKind::Function {
                name,
                parameters,
                body,
            },
        }
    }
}
