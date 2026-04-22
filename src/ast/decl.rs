use crate::lexer::span::Span;

use super::{Expr, Stmt, node_id::NodeId};

#[derive(Debug)]
pub struct Decl {
    pub id: NodeId,
    pub span: Span,
    pub kind: DeclKind,
}

#[derive(Debug)]
pub enum DeclKind {
    Function {
        name: String,
        parameters: Vec<(String, Span)>,
        body: Vec<Stmt>,
    },
}

impl Decl {
    pub fn function(
        name: String,
        parameters: Vec<(String, Span)>,
        body: Vec<Stmt>,
        span: Span,
    ) -> Decl {
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
