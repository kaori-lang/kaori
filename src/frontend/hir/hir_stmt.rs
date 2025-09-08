use crate::frontend::lexer::span::Span;

use super::{hir_ast_node::HirAstNode, hir_decl::HirDecl, hir_expr::HirExpr, node_id::NodeId};

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
        init: HirDecl,
        condition: HirExpr,
        increment: Box<HirStmt>,
        block: Box<HirStmt>,
    },
    Block(Vec<HirAstNode>),
    Expression(Box<HirExpr>),
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

    pub fn branch_(
        condition: HirExpr,
        then_branch: HirStmt,
        else_branch: Option<HirStmt>,
        span: Span,
    ) -> HirStmt {
        HirStmt {
            id: NodeId::default(),
            span,
            kind: HirStmtKind::Branch {
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
            kind: HirStmtKind::Loop {
                condition: Box::new(condition),
                block: Box::new(block),
            },
        }
    }

    pub fn block(nodes: Vec<HirAstNode>, span: Span) -> HirStmt {
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
            kind: HirStmtKind::Expression(Box::new(expr)),
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
