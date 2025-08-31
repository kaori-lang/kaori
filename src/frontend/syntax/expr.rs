use crate::frontend::scanner::span::Span;

use super::{
    node_id::NodeId,
    operator::{BinaryOp, UnaryOp},
};

#[derive(Debug, Clone)]
pub struct Expr {
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
    Identifier {
        name: String,
    },
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
            span,
            kind: ExprKind::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn unary(operator: UnaryOp, right: Expr, span: Span) -> Expr {
        let span = Span::merge(span, right.span);

        Expr {
            span,
            kind: ExprKind::Unary {
                operator,
                right: Box::new(right),
            },
        }
    }

    pub fn increment(identifier: Expr, span: Span) -> Expr {
        let span = Span::merge(span, identifier.span);

        let right = Expr::binary(
            BinaryOp::Add,
            identifier.to_owned(),
            Expr::number_literal(1.0, span),
        );

        Expr::assign(identifier, right)
    }

    pub fn decrement(identifier: Expr, span: Span) -> Expr {
        let span = Span::merge(span, identifier.span);

        let right = Expr::binary(
            BinaryOp::Subtract,
            identifier.to_owned(),
            Expr::number_literal(1.0, span),
        );

        Expr::assign(identifier, right)
    }

    pub fn assign(left: Expr, right: Expr) -> Expr {
        let span = Span::merge(left.span, right.span);

        Expr {
            span,
            kind: ExprKind::Assign {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn identifier(name: String, span: Span) -> Expr {
        Expr {
            span,
            kind: ExprKind::Identifier { name },
        }
    }

    pub fn function_call(callee: Expr, arguments: Vec<Expr>, span: Span) -> Expr {
        let span = Span::merge(callee.span, span);

        Expr {
            span,
            kind: ExprKind::FunctionCall {
                callee: Box::new(callee),
                arguments,
            },
        }
    }

    pub fn string_literal(value: String, span: Span) -> Expr {
        Expr {
            span,
            kind: ExprKind::StringLiteral(value),
        }
    }

    pub fn number_literal(value: f64, span: Span) -> Expr {
        Expr {
            span,
            kind: ExprKind::NumberLiteral(value),
        }
    }

    pub fn boolean_literal(value: bool, span: Span) -> Expr {
        Expr {
            span,
            kind: ExprKind::BooleanLiteral(value),
        }
    }
}
