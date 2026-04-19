use crate::lexer::span::Span;

use super::{expr::Expr, node_id::NodeId};

#[derive(Debug)]
pub struct Stmt {
    pub id: NodeId,
    pub span: Span,
    pub kind: StmtKind,
}

#[derive(Debug)]
pub enum StmtKind {
    Print(Box<Expr>),
    Branch {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Loop {
        init: Option<Expr>,
        condition: Expr,
        block: Box<Stmt>,
        increment: Option<Box<Stmt>>,
    },
    Block(Vec<Stmt>),
    UnsafeBlock(Vec<Stmt>),
    Expression(Box<Expr>),
    Break,
    Continue,
    Return(Option<Box<Expr>>),
}

impl Stmt {
    pub fn print(expr: Expr, span: Span) -> Stmt {
        Stmt {
            id: NodeId::default(),
            span,
            kind: StmtKind::Print(Box::new(expr)),
        }
    }

    pub fn branch_(
        condition: Expr,
        then_branch: Stmt,
        else_branch: Option<Stmt>,
        span: Span,
    ) -> Stmt {
        Stmt {
            id: NodeId::default(),
            span,
            kind: StmtKind::Branch {
                condition: Box::new(condition),
                then_branch: Box::new(then_branch),
                else_branch: else_branch.map(Box::new),
            },
        }
    }

    pub fn loop_(
        init: Option<Expr>,
        condition: Expr,
        block: Stmt,
        increment: Option<Stmt>,
        span: Span,
    ) -> Stmt {
        Stmt {
            id: NodeId::default(),
            span,
            kind: StmtKind::Loop {
                init,
                condition,
                block: Box::new(block),
                increment: increment.map(Box::new),
            },
        }
    }

    pub fn block(statements: Vec<Stmt>, span: Span) -> Stmt {
        Stmt {
            id: NodeId::default(),
            span,
            kind: StmtKind::Block(statements),
        }
    }

    pub fn unsafe_block(statements: Vec<Stmt>, span: Span) -> Stmt {
        Stmt {
            id: NodeId::default(),
            span,
            kind: StmtKind::UnsafeBlock(statements),
        }
    }

    pub fn expression(expr: Expr, span: Span) -> Stmt {
        Stmt {
            id: NodeId::default(),
            span,
            kind: StmtKind::Expression(Box::new(expr)),
        }
    }

    pub fn break_(span: Span) -> Stmt {
        Stmt {
            id: NodeId::default(),
            span,
            kind: StmtKind::Break,
        }
    }

    pub fn continue_(span: Span) -> Stmt {
        Stmt {
            id: NodeId::default(),
            span,
            kind: StmtKind::Continue,
        }
    }

    pub fn return_(expr: Option<Expr>, span: Span) -> Stmt {
        Stmt {
            id: NodeId::default(),
            span,
            kind: StmtKind::Return(expr.map(Box::new)),
        }
    }
}
