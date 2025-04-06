use crate::token::TokenType;

use super::ast_node::ASTNode;

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

impl ASTNode for Literal {}
