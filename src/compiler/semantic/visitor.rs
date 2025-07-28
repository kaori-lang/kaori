use crate::{
    compiler::syntax::{
        declaration_ast::DeclarationAST, expression_ast::ExpressionAST, statement_ast::StatementAST,
    },
    error::compilation_error::CompilationError,
};

pub trait Visitor<T> {
    fn visit_expression(&self, expression: ExpressionAST) -> Result<T, CompilationError>;
    fn visit_statement(&self, statement: StatementAST) -> Result<(), CompilationError>;
    fn visit_declaration(&mut self, declaration: DeclarationAST) -> Result<(), CompilationError>;
}
