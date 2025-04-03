use super::ast_node::ASTNode;
use super::token::Token;

pub struct BinaryOperator {
    left: ASTNode,
    right: ASTNode,
    _type: Token,
}
