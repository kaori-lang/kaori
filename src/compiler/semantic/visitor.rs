use crate::{
    compiler::syntax::{
        ast_node::ASTNode, declaration_ast::DeclarationAST, expression_ast::ExpressionAST,
        statement_ast::StatementAST,
    },
    error::compilation_error::CompilationError,
};

pub trait Visitor<T> {
    fn visit_ast_node(&mut self, ast_node: ASTNode) -> Result<(), CompilationError>;
    fn visit_declaration(&mut self, declaration: DeclarationAST) -> Result<(), CompilationError>;
    fn visit_statement(&mut self, statement: StatementAST) -> Result<(), CompilationError>;
    fn visit_expression(&mut self, expression: ExpressionAST) -> Result<T, CompilationError>;
}
