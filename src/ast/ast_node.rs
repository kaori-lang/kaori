use crate::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number,
    String,
    Boool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Literal {
        _type: Literal,
        value: String,
    },
    BinaryOperator {
        left: Box<ASTNode>,
        right: Box<ASTNode>,
        _type: Token,
    },
    UnaryOperator {
        _type: Token,
        right: Box<ASTNode>,
    },
}
