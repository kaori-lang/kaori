use crate::compiler::syntax::{
    ast_node::ASTNode, declaration_ast::DeclarationAST, expression_ast::ExpressionAST,
    statement_ast::StatementAST,
};

pub trait Visitor<T> {
    fn visit_expression(&self, expression: ExpressionAST) -> T;
    fn visit_statement(&self, statement: StatementAST);
    fn visit_declaration(&self, declaration: DeclarationAST);
}
