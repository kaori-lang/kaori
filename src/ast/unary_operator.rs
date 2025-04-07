use super::expr::Expr;
use crate::token::TokenType;

#[derive(Debug)]
pub struct UnaryOperator {
    ty: TokenType,
    right: Box<dyn Expr>,
}

impl UnaryOperator {
    pub fn new(ty: TokenType, right: Box<dyn Expr>) -> Self {
        Self { ty, right }
    }
}
impl Expr for UnaryOperator {}
