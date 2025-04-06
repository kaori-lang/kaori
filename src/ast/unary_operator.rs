use super::ast_node::ASTNode;
use crate::token::TokenType;

#[derive(Debug)]
pub struct UnaryOperator {
    ty: TokenType,
    right: Box<dyn ASTNode>,
}

impl UnaryOperator {
    pub fn new(ty: TokenType, right: Box<dyn ASTNode>) -> Self {
        Self { ty, right }
    }
}
impl ASTNode for UnaryOperator {}
