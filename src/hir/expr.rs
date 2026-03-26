use crate::{
    ast::{binary_op::BinaryOp, unary_op::UnaryOp},
    lexer::span::Span,
};

use super::node_id::NodeId;

#[derive(Debug, Clone)]
pub struct Expr {
    pub id: NodeId,
    pub span: Span,
    pub kind: ExprKind,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Binary {
        operator: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        operator: UnaryOp,
        right: Box<Expr>,
    },
    Assign {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Variable(NodeId),
    FunctionCall {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
    Function(NodeId),
    String(String),
    Number(f64),
    Boolean(bool),
}

impl Expr {
    pub fn binary(operator: BinaryOp, left: Expr, right: Expr, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn unary(operator: UnaryOp, right: Expr, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::Unary {
                operator,
                right: Box::new(right),
            },
        }
    }

    pub fn assign(left: Expr, right: Expr, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::Assign {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn variable(id: NodeId, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::Variable(id),
        }
    }

    pub fn function_call(callee: Expr, arguments: Vec<Expr>, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::FunctionCall {
                callee: Box::new(callee),
                arguments,
            },
        }
    }

    pub fn function(id: NodeId, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::Function(id),
        }
    }

    pub fn string(value: String, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::String(value),
        }
    }

    pub fn number(value: f64, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::Number(value),
        }
    }

    pub fn boolean(value: bool, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::Boolean(value),
        }
    }
}
