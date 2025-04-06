use crate::token::TokenType;

use super::ast_node::ASTNode;

#[derive(Debug)]
pub struct BinaryOperator {
    ty: TokenType,
    left: Box<dyn ASTNode>,
    right: Box<dyn ASTNode>,
}

impl BinaryOperator {
    pub fn new(ty: TokenType, left: Box<dyn ASTNode>, right: Box<dyn ASTNode>) -> Self {
        Self { ty, left, right }
    }
}

impl ASTNode for BinaryOperator {}
