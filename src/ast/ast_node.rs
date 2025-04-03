use crate::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Literal {
        _type: Token,
        value: String,
    },
    BinaryOperator {
        left: Box<ASTNode>,
        right: Box<ASTNode>,
        _type: Token,
    },
    Unary {
        _type: Token,
        left: Box<ASTNode>,
    },
}
