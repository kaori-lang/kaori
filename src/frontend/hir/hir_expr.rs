use crate::frontend::{
    scanner::span::Span,
    syntax::{binary_op::BinaryOp, unary_op::UnaryOp},
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
    Assign(Box<HirExpr>, Box<HirExpr>),
    Identifier(String),
    FunctionCall {
        callee: Box<HirExpr>,
        arguments: Vec<HirExpr>,
    },
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
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
            kind: HirExprKind::Assign(Box::new(left), Box::new(right)),
        }
    }

    pub fn identifier(name: String, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::Identifier(name),
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

    pub fn string_literal(value: String, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::StringLiteral(value),
        }
    }

    pub fn number_literal(value: f64, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::NumberLiteral(value),
        }
    }

    pub fn boolean_literal(value: bool, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::BooleanLiteral(value),
        }
    }
}
