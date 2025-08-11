use crate::error::kaori_error::KaoriError;

pub trait ExprVisitor<T, U> {
    fn visit_expression(&self, expression: &T) -> Result<U, KaoriError>;
}

pub trait StmtVisitor<T, U> {
    fn visit_statement(&self, statement: &T) -> Result<U, KaoriError>;
}

pub trait DeclVisitor<T, U> {
    fn visit_declaration(&self, declaration: &T) -> Result<U, KaoriError>;
}

pub trait AstNodeVisitor<T, U> {
    fn visit_nodes(&self, nodes: &[T]) -> Result<Vec<U>, KaoriError>;
    fn visit_ast_node(&self, node: &T) -> Result<U, KaoriError>;
}
