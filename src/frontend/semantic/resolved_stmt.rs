use crate::frontend::scanner::span::Span;

use super::{resolved_ast_node::ResolvedAstNode, resolved_expr::ResolvedExpr};

pub struct ResolvedStmt {
    pub span: Span,
    pub kind: ResolvedStmtKind,
}

pub enum ResolvedStmtKind {
    Print(Box<ResolvedExpr>),
    If {
        condition: Box<ResolvedExpr>,
        then_branch: Box<ResolvedStmt>,
        else_branch: Option<Box<ResolvedStmt>>,
    },
    WhileLoop {
        condition: Box<ResolvedExpr>,
        block: Box<ResolvedStmt>,
    },
    Block(Vec<ResolvedAstNode>),
    Expression(Box<ResolvedExpr>),
    Break,
    Continue,
    Return(Option<Box<ResolvedExpr>>),
}

impl ResolvedStmt {
    pub fn print(expression: ResolvedExpr, span: Span) -> ResolvedStmt {
        ResolvedStmt {
            span,
            kind: ResolvedStmtKind::Print(Box::new(expression)),
        }
    }

    pub fn if_(
        condition: ResolvedExpr,
        then_branch: ResolvedStmt,
        else_branch: Option<ResolvedStmt>,
        span: Span,
    ) -> ResolvedStmt {
        ResolvedStmt {
            span,
            kind: ResolvedStmtKind::If {
                condition: Box::new(condition),
                then_branch: Box::new(then_branch),
                else_branch: else_branch.map(Box::new),
            },
        }
    }

    pub fn while_loop(condition: ResolvedExpr, block: ResolvedStmt, span: Span) -> ResolvedStmt {
        ResolvedStmt {
            span,
            kind: ResolvedStmtKind::WhileLoop {
                condition: Box::new(condition),
                block: Box::new(block),
            },
        }
    }

    pub fn block(nodes: Vec<ResolvedAstNode>, span: Span) -> ResolvedStmt {
        ResolvedStmt {
            span,
            kind: ResolvedStmtKind::Block(nodes),
        }
    }

    pub fn expression(expr: ResolvedExpr, span: Span) -> ResolvedStmt {
        ResolvedStmt {
            span,
            kind: ResolvedStmtKind::Expression(Box::new(expr)),
        }
    }

    pub fn break_(span: Span) -> ResolvedStmt {
        ResolvedStmt {
            span,
            kind: ResolvedStmtKind::Break,
        }
    }

    pub fn continue_(span: Span) -> ResolvedStmt {
        ResolvedStmt {
            span,
            kind: ResolvedStmtKind::Continue,
        }
    }

    pub fn return_(expression: Option<ResolvedExpr>, span: Span) -> ResolvedStmt {
        ResolvedStmt {
            span,
            kind: ResolvedStmtKind::Return(expression.map(Box::new)),
        }
    }
}
