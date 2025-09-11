use crate::frontend::lexer::span::Span;

use super::{ast_id::AstId, ast_node::AstNode, decl::Decl, expr::Expr};

#[derive(Debug)]
pub struct Stmt {
    pub id: AstId,
    pub span: Span,
    pub kind: StmtKind,
}

#[derive(Debug)]
pub enum StmtKind {
    Print(Box<Expr>),
    If {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    WhileLoop {
        condition: Box<Expr>,
        block: Box<Stmt>,
    },
    ForLoop {
        init: Decl,
        condition: Expr,
        increment: Box<Stmt>,
        block: Box<Stmt>,
    },
    Block(Vec<AstNode>),
    Expression(Box<Expr>),
    Break,
    Continue,
    Return(Option<Box<Expr>>),
}

impl Stmt {
    pub fn print(expression: Expr, span: Span) -> Stmt {
        Stmt {
            id: AstId::default(),
            span,
            kind: StmtKind::Print(Box::new(expression)),
        }
    }

    pub fn if_(condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>, span: Span) -> Stmt {
        Stmt {
            id: AstId::default(),
            span,
            kind: StmtKind::If {
                condition: Box::new(condition),
                then_branch: Box::new(then_branch),
                else_branch: else_branch.map(Box::new),
            },
        }
    }

    pub fn while_loop(condition: Expr, block: Stmt, span: Span) -> Stmt {
        Stmt {
            id: AstId::default(),
            span,
            kind: StmtKind::WhileLoop {
                condition: Box::new(condition),
                block: Box::new(block),
            },
        }
    }

    pub fn for_loop(init: Decl, condition: Expr, increment: Stmt, block: Stmt, span: Span) -> Stmt {
        Stmt {
            id: AstId::default(),
            span,
            kind: StmtKind::ForLoop {
                init,
                condition,
                increment: Box::new(increment),
                block: Box::new(block),
            },
        }
    }

    pub fn block(nodes: Vec<AstNode>, span: Span) -> Stmt {
        Stmt {
            id: AstId::default(),
            span,
            kind: StmtKind::Block(nodes),
        }
    }

    pub fn expression(expr: Expr, span: Span) -> Stmt {
        Stmt {
            id: AstId::default(),
            span,
            kind: StmtKind::Expression(Box::new(expr)),
        }
    }

    pub fn break_(span: Span) -> Stmt {
        Stmt {
            id: AstId::default(),
            span,
            kind: StmtKind::Break,
        }
    }

    pub fn continue_(span: Span) -> Stmt {
        Stmt {
            id: AstId::default(),
            span,
            kind: StmtKind::Continue,
        }
    }

    pub fn return_(expression: Option<Expr>, span: Span) -> Stmt {
        Stmt {
            id: AstId::default(),
            span,
            kind: StmtKind::Return(expression.map(Box::new)),
        }
    }
}
