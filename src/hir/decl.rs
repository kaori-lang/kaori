use crate::lexer::span::Span;

use super::{expr::Expr, node_id::NodeId, stmt::Stmt};

#[derive(Debug)]
pub struct Decl {
    pub id: NodeId,
    pub span: Span,
    pub kind: DeclKind,
}

#[derive(Debug)]
pub enum DeclKind {
    Function {
        parameters: Vec<Expr>,
        body: Vec<Stmt>,
    },
}

impl Decl {
    pub fn function(id: NodeId, parameters: Vec<Expr>, body: Vec<Stmt>, span: Span) -> Decl {
        Decl {
            id,
            span,
            kind: DeclKind::Function { parameters, body },
        }
    }
}
