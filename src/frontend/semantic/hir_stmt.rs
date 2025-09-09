use crate::frontend::{lexer::span::Span, syntax::node_id::NodeId};

use super::{hir_decl::HirDecl, hir_expr::HirExpr, hir_node::HirNode};

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
    Loop {
        init: Option<HirDecl>,
        condition: HirExpr,
        block: Box<HirStmt>,
    },
    Block(Vec<HirNode>),
    Expression(Box<HirExpr>),
    Break,
    Continue,
    Return(Option<Box<HirExpr>>),
}

impl HirStmt {
    pub fn print(id: NodeId, expr: HirExpr, span: Span) -> HirStmt {
        HirStmt {
            id,
            span,
            kind: HirStmtKind::Print(Box::new(expr)),
        }
    }

    pub fn branch_(
        id: NodeId,
        condition: HirExpr,
        then_branch: HirStmt,
        else_branch: Option<HirStmt>,
        span: Span,
    ) -> HirStmt {
        HirStmt {
            id,
            span,
            kind: HirStmtKind::Branch {
                condition: Box::new(condition),
                then_branch: Box::new(then_branch),
                else_branch: else_branch.map(Box::new),
            },
        }
    }

    pub fn loop_(
        id: NodeId,
        init: Option<HirDecl>,
        condition: HirExpr,
        block: HirStmt,
        span: Span,
    ) -> HirStmt {
        HirStmt {
            id,
            span,
            kind: HirStmtKind::Loop {
                init,
                condition,
                block: Box::new(block),
            },
        }
    }

    pub fn block(id: NodeId, nodes: Vec<HirNode>, span: Span) -> HirStmt {
        HirStmt {
            id,
            span,
            kind: HirStmtKind::Block(nodes),
        }
    }

    pub fn expression(id: NodeId, expr: HirExpr, span: Span) -> HirStmt {
        HirStmt {
            id,
            span,
            kind: HirStmtKind::Expression(Box::new(expr)),
        }
    }

    pub fn break_(id: NodeId, span: Span) -> HirStmt {
        HirStmt {
            id,
            span,
            kind: HirStmtKind::Break,
        }
    }

    pub fn continue_(id: NodeId, span: Span) -> HirStmt {
        HirStmt {
            id,
            span,
            kind: HirStmtKind::Continue,
        }
    }

    pub fn return_(id: NodeId, expr: Option<HirExpr>, span: Span) -> HirStmt {
        HirStmt {
            id,
            span,
            kind: HirStmtKind::Return(expr.map(Box::new)),
        }
    }
}
