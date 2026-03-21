use crate::{
    ast::{binary_op::BinaryOp, unary_op::UnaryOp},
    lexer::span::Span,
};

use super::node_id::NodeId;

#[derive(Debug, Clone)]
pub struct HirExpr {
    pub id: NodeId,
    pub span: Span,
    pub kind: HirExprKind,
}

#[derive(Debug, Clone)]
pub enum HirExprKind {
    Binary {
        operator: BinaryOp,
        left: Box<HirExpr>,
        right: Box<HirExpr>,
    },
    Unary {
        right: Box<HirExpr>,
        operator: UnaryOp,
    },
    Assign {
        left: Box<HirExpr>,
        right: Box<HirExpr>,
    },
    Variable(NodeId),
    FunctionCall {
        callee: Box<HirExpr>,
        arguments: Vec<HirExpr>,
    },
    Function(NodeId),
    String(String),
    Number(f64),
    Boolean(bool),
}

impl HirExpr {
    pub fn binary(operator: BinaryOp, left: HirExpr, right: HirExpr, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn unary(operator: UnaryOp, right: HirExpr, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::Unary {
                operator,
                right: Box::new(right),
            },
        }
    }

    pub fn assign(left: HirExpr, right: HirExpr, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::Assign {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn variable(id: NodeId, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::Variable(id),
        }
    }

    pub fn function_call(callee: HirExpr, arguments: Vec<HirExpr>, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::FunctionCall {
                callee: Box::new(callee),
                arguments,
            },
        }
    }

    pub fn function(id: NodeId, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::Function(id),
        }
    }

    pub fn string(value: String, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::String(value),
        }
    }

    pub fn number(value: f64, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::Number(value),
        }
    }

    pub fn boolean(value: bool, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::Boolean(value),
        }
    }
}
