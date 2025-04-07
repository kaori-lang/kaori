use crate::token::TokenType;

use super::expr::Expr;

#[derive(Debug)]
pub struct Literal {
    ty: TokenType,
    value: String,
}

impl Literal {
    pub fn new(ty: TokenType, value: String) -> Self {
        Self { ty, value }
    }
}

impl Expr for Literal {}
