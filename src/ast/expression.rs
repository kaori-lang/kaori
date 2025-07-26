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
        identifier: Identifier,
        right: Box<Expression>,
    },
    Identifier(Identifier),
    Literal(Data),
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub value: String,
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
