use crate::compiler::{lexer::span::Span, semantic::resolution::Resolution};

#[derive(Debug)]
pub struct Expression {
    pub span: Span,
    pub kind: ExpressionKind,
}

#[derive(Debug)]
pub enum ExpressionKind {
    Binary {
        operator: BinaryOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Unary {
        operator: UnaryOp,
        right: Box<Expression>,
    },
    Assign {
        identifier: Box<Expression>,
        right: Box<Expression>,
    },
    Identifier {
        name: String,
        resolution: Option<Resolution>,
        span: Span,
    },
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
}

impl Expression {
    pub fn binary(
        operator: BinaryOp,
        left: Box<Expression>,
        right: Box<Expression>,
        span: Span,
    ) -> Expression {
        Expression {
            span,
            kind: ExpressionKind::Binary {
                operator,
                left,
                right,
            },
        }
    }

    pub fn unary(operator: UnaryOp, right: Box<Expression>, span: Span) -> Expression {
        Expression {
            span,
            kind: ExpressionKind::Unary { operator, right },
        }
    }

    pub fn assign(identifier: Box<Expression>, right: Box<Expression>, span: Span) -> Expression {
        Expression {
            span,
            kind: ExpressionKind::Assign { identifier, right },
        }
    }

    pub fn identifier(name: String, resolution: Option<Resolution>, span: Span) -> Expression {
        Expression {
            span,
            kind: ExpressionKind::Identifier {
                name,
                resolution,
                span,
            },
        }
    }

    pub fn string_literal(value: String, span: Span) -> Expression {
        Expression {
            span,
            kind: ExpressionKind::StringLiteral(value),
        }
    }

    pub fn number_literal(value: f64, span: Span) -> Expression {
        Expression {
            span,
            kind: ExpressionKind::NumberLiteral(value),
        }
    }

    pub fn boolean_literal(value: bool, span: Span) -> Expression {
        Expression {
            span,
            kind: ExpressionKind::BooleanLiteral(value),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Plus,
    Minus,
    Multiply,
    Divide,
    Remainder,
    And,
    Or,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOp {
    Negate,
    Not,
}
