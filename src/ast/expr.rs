use crate::lexer::span::Span;

use super::{assign_op::AssignOp, binary_op::BinaryOp, node_id::NodeId, unary_op::UnaryOp};

#[derive(Debug)]
pub struct Expr {
    pub id: NodeId,
    pub span: Span,
    pub kind: ExprKind,
}

#[derive(Debug)]
pub enum ExprKind {
    Binary {
        operator: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    LogicalAnd {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    LogicalOr {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    LogicalNot {
        expr: Box<Expr>,
    },
    Unary {
        operator: UnaryOp,
        right: Box<Expr>,
    },
    Assign {
        operator: AssignOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Identifier(String),
    FunctionCall {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
}

impl Expr {
    pub fn binary(operator: BinaryOp, left: Expr, right: Expr) -> Expr {
        let span = Span::merge(left.span, right.span);

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

    pub fn logical_and(left: Expr, right: Expr) -> Expr {
        let span = Span::merge(left.span, right.span);

        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::LogicalAnd {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn logical_or(left: Expr, right: Expr) -> Expr {
        let span = Span::merge(left.span, right.span);

        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::LogicalOr {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn logical_not(expr: Expr) -> Expr {
        Expr {
            id: NodeId::default(),
            span: expr.span,
            kind: ExprKind::LogicalNot {
                expr: Box::new(expr),
            },
        }
    }

    pub fn unary(operator: UnaryOp, right: Expr) -> Expr {
        let span = right.span;

        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::Unary {
                operator,
                right: Box::new(right),
            },
        }
    }

    pub fn assign(operator: AssignOp, left: Expr, right: Expr) -> Expr {
        let span = Span::merge(left.span, right.span);

        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::Assign {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn identifier(name: String, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::Identifier(name),
        }
    }

    pub fn function_call(callee: Expr, arguments: Vec<Expr>, span: Span) -> Expr {
        let span = Span::merge(callee.span, span);

        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::FunctionCall {
                callee: Box::new(callee),
                arguments,
            },
        }
    }

    pub fn string_literal(value: String, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::StringLiteral(value),
        }
    }

    pub fn number_literal(value: f64, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::NumberLiteral(value),
        }
    }

    pub fn boolean_literal(value: bool, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::BooleanLiteral(value),
        }
    }
}
