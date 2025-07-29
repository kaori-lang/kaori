use crate::compiler::{lexer::span::Span, semantic::resolution::Resolution};

use super::operator::{BinaryOp, UnaryOp};

#[derive(Debug)]
pub struct Expr {
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
    Unary {
        operator: UnaryOp,
        right: Box<Expr>,
    },
    Assign {
        identifier: Box<Expr>,
        right: Box<Expr>,
    },
    Identifier {
        name: String,
        resolution: Resolution,
        span: Span,
    },
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
}

impl Expr {
    pub fn binary(operator: BinaryOp, left: Box<Expr>, right: Box<Expr>, span: Span) -> Expr {
        Expr {
            span,
            kind: ExprKind::Binary {
                operator,
                left,
                right,
            },
        }
    }

    pub fn unary(operator: UnaryOp, right: Box<Expr>, span: Span) -> Expr {
        Expr {
            span,
            kind: ExprKind::Unary { operator, right },
        }
    }

    pub fn assign(identifier: Box<Expr>, right: Box<Expr>, span: Span) -> Expr {
        Expr {
            span,
            kind: ExprKind::Assign { identifier, right },
        }
    }

    pub fn identifier(name: String, span: Span) -> Expr {
        Expr {
            span,
            kind: ExprKind::Identifier {
                name,
                resolution: Resolution {
                    global: false,
                    offset: 0,
                },
                span,
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
