use crate::token::{DataType, TokenType};

#[derive(Debug)]
pub struct BinaryOperator {
    pub ty: TokenType,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct UnaryOperator {
    pub ty: TokenType,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct Identifier {
    pub ty: TokenType,
    pub value: String,
}

#[derive(Debug)]
pub struct Literal {
    pub ty: DataType,
    pub value: String,
}

impl BinaryOperator {
    pub fn new(ty: TokenType, left: Box<Expr>, right: Box<Expr>) -> Self {
        BinaryOperator { ty, left, right }
    }
}

impl UnaryOperator {
    pub fn new(ty: TokenType, right: Box<Expr>) -> Self {
        UnaryOperator { ty, right }
    }
}

impl Identifier {
    pub fn new(ty: TokenType, value: String) -> Self {
        Identifier { ty, value }
    }
}

impl Literal {
    pub fn new(ty: DataType, value: String) -> Self {
        Literal { ty, value }
    }
}

#[derive(Debug)]
pub enum Expr {
    BinaryOperator(BinaryOperator),
    UnaryOperator(UnaryOperator),
    Identifier(Identifier),
    Literal(Literal),
}
