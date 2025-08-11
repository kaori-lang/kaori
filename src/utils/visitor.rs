use crate::{
    error::kaori_error::KaoriError,
    frontend::syntax::{ast_node::ASTNode, declaration::Decl, expression::Expr, statement::Stmt},
};

pub trait Visitor<T> {
    fn visit_nodes(&mut self, nodes: &[ASTNode]) -> Result<T, KaoriError>;
    fn visit_ast_node(&mut self, node: &ASTNode) -> Result<T, KaoriError>;

    fn visit_declaration(&mut self, declaration: &Decl) -> Result<T, KaoriError>;

    fn visit_statement(&mut self, statement: &Stmt) -> Result<T, KaoriError>;
    fn visit_expression(&mut self, expression: &Expr) -> Result<T, KaoriError>;
}
