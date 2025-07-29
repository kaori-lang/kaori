use crate::compiler::lexer::span::Span;

use super::{ast_node::ASTNode, expression::Expr};

#[derive(Debug)]
pub struct Stmt {
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
    Block(Vec<ASTNode>),
    Expression(Box<Expr>),
    Break,
    Continue,
}

impl Stmt {
    pub fn print(expression: Box<Expr>, span: Span) -> Stmt {
        Stmt {
            span,
            kind: StmtKind::Print(expression),
        }
    }

    pub fn if_(
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
        span: Span,
    ) -> Stmt {
        Stmt {
            span,
            kind: StmtKind::If {
                condition,
                then_branch,
                else_branch,
            },
        }
    }

    pub fn while_loop(condition: Box<Expr>, block: Box<Stmt>, span: Span) -> Stmt {
        Stmt {
            span,
            kind: StmtKind::WhileLoop { condition, block },
        }
    }

    pub fn block(declarations: Vec<ASTNode>, span: Span) -> Stmt {
        Stmt {
            span,
            kind: StmtKind::Block(declarations),
        }
    }

    pub fn expression(expr: Box<Expr>, span: Span) -> Stmt {
        Stmt {
            span,
            kind: StmtKind::Expression(expr),
        }
    }

    pub fn break_(span: Span) -> Stmt {
        Stmt {
            span,
            kind: StmtKind::Break,
        }
    }

    pub fn continue_(span: Span) -> Stmt {
        Stmt {
            span,
            kind: StmtKind::Continue,
        }
    }
}
