use crate::frontend::{
    scanner::span::Span,
    semantic::resolution::Resolution,
    syntax::operator::{BinaryOp, UnaryOp},
};

#[derive(Debug, Clone)]
pub struct ResolvedExpr {
    pub span: Span,
    pub kind: ResolvedExprKind,
}

#[derive(Debug, Clone)]
pub enum ResolvedExprKind {
    Binary {
        operator: BinaryOp,
        left: Box<ResolvedExpr>,
        right: Box<ResolvedExpr>,
    },
    Unary {
        operator: UnaryOp,
        right: Box<ResolvedExpr>,
    },
    Assign {
        identifier: Box<ResolvedExpr>,
        right: Box<ResolvedExpr>,
    },
    Variable(usize),
    Function(usize),
    FunctionCall {
        callee: Box<ResolvedExpr>,
        arguments: Vec<ResolvedExpr>,
    },
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
}

impl ResolvedExpr {
    pub fn binary(
        operator: BinaryOp,
        left: ResolvedExpr,
        right: ResolvedExpr,
        span: Span,
    ) -> ResolvedExpr {
        ResolvedExpr {
            span,
            kind: ResolvedExprKind::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn unary(operator: UnaryOp, right: ResolvedExpr, span: Span) -> ResolvedExpr {
        ResolvedExpr {
            span,
            kind: ResolvedExprKind::Unary {
                operator,
                right: Box::new(right),
            },
        }
    }

    pub fn increment(identifier: ResolvedExpr, span: Span) -> ResolvedExpr {
        let right = ResolvedExpr::binary(
            BinaryOp::Plus,
            identifier.to_owned(),
            ResolvedExpr::number_literal(1.0, span),
            span,
        );

        ResolvedExpr::assign(identifier, right, span)
    }

    pub fn decrement(identifier: ResolvedExpr, span: Span) -> ResolvedExpr {
        let right = ResolvedExpr::binary(
            BinaryOp::Minus,
            identifier.to_owned(),
            ResolvedExpr::number_literal(1.0, span),
            span,
        );

        ResolvedExpr::assign(identifier, right, span)
    }

    pub fn assign(identifier: ResolvedExpr, right: ResolvedExpr, span: Span) -> ResolvedExpr {
        ResolvedExpr {
            span,
            kind: ResolvedExprKind::Assign {
                identifier: Box::new(identifier),
                right: Box::new(right),
            },
        }
    }

    pub fn identifier(name: String, span: Span) -> ResolvedExpr {
        ResolvedExpr {
            span,
            kind: ResolvedExprKind::Identifier {
                name,
                resolution: Resolution::default(),
            },
        }
    }

    pub fn function_call(
        callee: ResolvedExpr,
        arguments: Vec<ResolvedExpr>,
        span: Span,
    ) -> ResolvedExpr {
        ResolvedExpr {
            span,
            kind: ResolvedExprKind::FunctionCall {
                callee: Box::new(callee),
                arguments,
            },
        }
    }

    pub fn string_literal(value: String, span: Span) -> ResolvedExpr {
        ResolvedExpr {
            span,
            kind: ResolvedExprKind::StringLiteral(value),
        }
    }

    pub fn number_literal(value: f64, span: Span) -> ResolvedExpr {
        ResolvedExpr {
            span,
            kind: ResolvedExprKind::NumberLiteral(value),
        }
    }

    pub fn boolean_literal(value: bool, span: Span) -> ResolvedExpr {
        ResolvedExpr {
            span,
            kind: ResolvedExprKind::BooleanLiteral(value),
        }
    }
}
