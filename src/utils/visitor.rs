use crate::{
    error::kaori_error::KaoriError,
    frontend::syntax::{ast_node::ASTNode, declaration::Decl, expression::Expr, statement::Stmt},
};

pub trait Visitor<T> {
    fn visit_nodes(&mut self, nodes: &mut [ASTNode]) -> Result<(), KaoriError>;
    fn visit_ast_node(&mut self, node: &mut ASTNode) -> Result<(), KaoriError>;

    fn visit_declaration(&mut self, declaration: &mut Decl) -> Result<(), KaoriError>;

    fn visit_statement(&mut self, statement: &mut Stmt) -> Result<(), KaoriError>;
    fn visit_expression(&mut self, expression: &mut Expr) -> Result<T, KaoriError>;
}
