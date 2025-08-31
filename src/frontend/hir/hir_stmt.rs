use crate::frontend::{scanner::span::Span, syntax::node_id::NodeId};

use super::hir_expr::HirExpr;

#[derive(Debug)]
pub struct HirStmt {
    pub id: NodeId,
    pub span: Span,
    pub kind: HirStmtKind,
}

#[derive(Debug)]
pub enum HirStmtKind {
    Print(Box<HirExpr>),
    Branch {
        condition: Box<HirExpr>,
        then_branch: Box<HirStmt>,
        else_branch: Option<Box<HirStmt>>,
    },
    WhileLoop {
        condition: Box<HirExpr>,
        block: Box<HirStmt>,
    },
    Block(Vec<AstNode>),
    HirExpression(Box<HirExpr>),
    Break,
    Continue,
    Return(Option<Box<HirExpr>>),
}

impl HirStmt {
    pub fn print(expr: HirExpr, span: Span) -> HirStmt {
        HirStmt {
            id: NodeId::default(),
            span,
            kind: HirStmtKind::Print(Box::new(expr)),
        }
    }

    pub fn if_(
        condition: HirExpr,
        then_branch: HirStmt,
        else_branch: Option<HirStmt>,
        span: Span,
    ) -> HirStmt {
        HirStmt {
            id: NodeId::default(),
            span,
            kind: HirStmtKind::If {
                condition: Box::new(condition),
                then_branch: Box::new(then_branch),
                else_branch: else_branch.map(Box::new),
            },
        }
    }

    pub fn while_loop(condition: HirExpr, block: HirStmt, span: Span) -> HirStmt {
        HirStmt {
            id: NodeId::default(),
            span,
            kind: HirStmtKind::WhileLoop {
                condition: Box::new(condition),
                block: Box::new(block),
            },
        }
    }

    pub fn block(nodes: Vec<AstNode>, span: Span) -> HirStmt {
        HirStmt {
            id: NodeId::default(),
            span,
            kind: HirStmtKind::Block(nodes),
        }
    }

    pub fn expression(expr: HirExpr, span: Span) -> HirStmt {
        HirStmt {
            id: NodeId::default(),
            span,
            kind: HirStmtKind::HirExpression(Box::new(expr)),
        }
    }

    pub fn break_(span: Span) -> HirStmt {
        HirStmt {
            id: NodeId::default(),
            span,
            kind: HirStmtKind::Break,
        }
    }

    pub fn continue_(span: Span) -> HirStmt {
        HirStmt {
            id: NodeId::default(),
            span,
            kind: HirStmtKind::Continue,
        }
    }

    pub fn return_(expr: Option<HirExpr>, span: Span) -> HirStmt {
        HirStmt {
            id: NodeId::default(),
            span,
            kind: HirStmtKind::Return(expr.map(Box::new)),
        }
    }
}
