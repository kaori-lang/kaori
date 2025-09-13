use crate::frontend::lexer::span::Span;

use super::{assign_op::AssignOp, ast_id::AstId, binary_op::BinaryOp, unary_op::UnaryOp};

#[derive(Debug)]
pub struct Expr {
    pub id: AstId,
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
        right: Box<Expr>,
        operator: UnaryOp,
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
            id: AstId::default(),
            span,
            kind: ExprKind::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn unary(operator: UnaryOp, right: Expr) -> Expr {
        let span = Span::merge(operator.span, right.span);

        Expr {
            id: AstId::default(),
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
            id: AstId::default(),
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
            id: AstId::default(),
            span,
            kind: ExprKind::Identifier(name),
        }
    }

    pub fn function_call(callee: Expr, arguments: Vec<Expr>, span: Span) -> Expr {
        let span = Span::merge(callee.span, span);

        Expr {
            id: AstId::default(),
            span,
            kind: ExprKind::FunctionCall {
                callee: Box::new(callee),
                arguments,
            },
        }
    }

    pub fn string_literal(value: String, span: Span) -> Expr {
        Expr {
            id: AstId::default(),
            span,
            kind: ExprKind::StringLiteral(value),
        }
    }

    pub fn number_literal(value: f64, span: Span) -> Expr {
        Expr {
            id: AstId::default(),
            span,
            kind: ExprKind::NumberLiteral(value),
        }
    }

    pub fn boolean_literal(value: bool, span: Span) -> Expr {
        Expr {
            id: AstId::default(),
            span,
            kind: ExprKind::BooleanLiteral(value),
        }
    }
}
