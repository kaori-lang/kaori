use crate::token::TokenType;

use super::expr::Expr;

#[derive(Debug)]
pub struct BinaryOperator {
    ty: TokenType,
    left: Box<dyn Expr>,
    right: Box<dyn Expr>,
}

impl BinaryOperator {
    pub fn new(ty: TokenType, left: Box<dyn Expr>, right: Box<dyn Expr>) -> Self {
        Self { ty, left, right }
    }
}

impl Expr for BinaryOperator {}
