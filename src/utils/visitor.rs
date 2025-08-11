use crate::{
    error::kaori_error::KaoriError,
    frontend::syntax::{ast_node::AstNode, declaration::Decl, expression::Expr, statement::Stmt},
};

pub trait Visitor<T, U> {
    fn visit_nodes(&mut self, nodes: &[AstNode]) -> Result<T, KaoriError>;
    fn visit_ast_node(&mut self, node: &AstNode) -> Result<U, KaoriError>;

    fn visit_declaration(&mut self, declaration: &Decl) -> Result<U, KaoriError>;

    fn visit_statement(&mut self, statement: &Stmt) -> Result<U, KaoriError>;
    fn visit_expression(&mut self, expression: &Expr) -> Result<T, KaoriError>;
}

pub trait ExprVisitor<T> {
    fn visit_expression(&self, expression: &Expr) -> Result<T, KaoriError>;
}

pub trait StmtVisitor<T> {
    fn visit_statement(&self, statement: &Stmt) -> Result<T, KaoriError>;
}

pub trait DeclVisitor<T> {
    fn visit_declaration(&self, declaration: &Decl) -> Result<T, KaoriError>;
}

pub trait AstNodeVisitor<T> {
    fn visit_nodes(&self, nodes: &[AstNode]) -> Result<Vec<T>, KaoriError>;
    fn visit_ast_node(&self, node: &AstNode) -> Result<T, KaoriError>;
}
