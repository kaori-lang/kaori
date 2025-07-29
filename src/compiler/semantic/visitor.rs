use crate::{
    compiler::syntax::{
        ast_node::ASTNode, declaration_ast::DeclarationAST, expression_ast::ExpressionAST,
        statement_ast::StatementAST,
    },
    error::compilation_error::CompilationError,
};

pub trait Visitor<T> {
    fn run(&mut self, ast: &mut Vec<ASTNode>) -> Result<(), CompilationError>;
    fn visit_ast_node(&mut self, ast_node: &mut ASTNode) -> Result<(), CompilationError>;
    fn visit_declaration(
        &mut self,
        declaration: &mut DeclarationAST,
    ) -> Result<(), CompilationError>;
    fn visit_statement(&mut self, statement: &mut StatementAST) -> Result<(), CompilationError>;
    fn visit_expression(&mut self, expression: &mut ExpressionAST) -> Result<T, CompilationError>;
}
