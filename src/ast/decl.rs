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
}

#[derive(Debug)]
pub struct Parameter {
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

impl Decl {
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
