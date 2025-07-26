use crate::lexer::data::Data;

#[derive(Debug)]
pub enum Expression {
    Binary {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Unary {
        operator: UnaryOperator,
        right: Box<Expression>,
    },
    Assign {
        identifier: String,
        right: Box<Expression>,
    },
    Identifier(String),
    Literal(Data),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
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
pub enum UnaryOperator {
    Negate,
    Not,
}
