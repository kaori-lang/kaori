use crate::{
    compiler::syntax::{ast_node::ASTNode, declaration::Decl, expression::Expr, statement::Stmt},
    error::compilation_error::CompilationError,
};

pub trait Visitor<T> {
    fn run(&mut self, ast: &mut Vec<ASTNode>) -> Result<(), CompilationError>;
    fn visit_ast_node(&mut self, ast_node: &mut ASTNode) -> Result<(), CompilationError>;
    fn visit_declaration(&mut self, declaration: &mut Decl) -> Result<(), CompilationError>;
    fn visit_statement(&mut self, statement: &mut Stmt) -> Result<(), CompilationError>;
    fn visit_expression(&mut self, expression: &mut Expr) -> Result<T, CompilationError>;
}
