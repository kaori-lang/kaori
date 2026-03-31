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
        right: Box<Expr>,
    },
    Function {
        parameters: Vec<Parameter>,
        body: Vec<Node>,
    },
}

#[derive(Debug)]
pub struct Parameter {
    pub id: NodeId,
    pub span: Span,
}

#[derive(Debug)]
pub struct Field {
    pub id: NodeId,
    pub span: Span,
}

impl Parameter {
    pub fn new(id: NodeId, span: Span) -> Parameter {
        Parameter { id, span }
    }
}

impl Field {
    pub fn new(id: NodeId, span: Span) -> Field {
        Field { id, span }
    }
}

impl Decl {
    pub fn variable(id: NodeId, right: Expr, span: Span) -> Decl {
        Decl {
            id,
            span,

            kind: DeclKind::Variable {
                right: Box::new(right),
            },
        }
    }

    pub fn function(id: NodeId, parameters: Vec<Parameter>, body: Vec<Node>, span: Span) -> Decl {
        Decl {
            id,
            span,
            kind: DeclKind::Function { parameters, body },
        }
    }
}
