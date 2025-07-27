use super::{ast_node::ASTNode, expression_ast::ExpressionAST};

pub enum DeclarationAST {
    Variable {
        identifier: String,
        right: Box<ExpressionAST>,
        line: u32,
    },
}
