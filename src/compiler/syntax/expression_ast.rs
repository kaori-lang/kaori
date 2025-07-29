use crate::compiler::{lexer::span::Span, semantic::resolution::Resolution};

#[derive(Debug)]
pub enum ExpressionAST {
    Binary {
        operator: BinaryOp,
        left: Box<ExpressionAST>,
        right: Box<ExpressionAST>,
        span: Span,
    },
    Unary {
        operator: UnaryOp,
        right: Box<ExpressionAST>,
        span: Span,
    },
    Assign {
        identifier: String,
        right: Box<ExpressionAST>,
        span: Span,
    },
    Identifier {
        name: String,
        resolution: Option<Resolution>,
        span: Span,
    },
    StringLiteral(String, Span),
    NumberLiteral(f64, Span),
    BooleanLiteral(bool, Span),
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
