use crate::{
    compiler::syntax::{ast_node::ASTNode, declaration::Decl, expression::Expr, statement::Stmt},
    error::kaori_error::KaoriError,
};

pub trait Visitor<T> {
    fn run(&mut self, ast: &mut Vec<ASTNode>) -> Result<(), KaoriError>;
    fn visit_ast_node(&mut self, ast_node: &mut ASTNode) -> Result<(), KaoriError>;
    fn visit_declaration(&mut self, declaration: &mut Decl) -> Result<(), KaoriError>;
    fn visit_statement(&mut self, statement: &mut Stmt) -> Result<(), KaoriError>;
    fn visit_expression(&mut self, expression: &mut Expr) -> Result<T, KaoriError>;
}
