use super::{declaration_ast::DeclarationAST, statement_ast::StatementAST};

#[derive(Debug)]
pub enum ASTNode {
    Declaration(Box<DeclarationAST>),
    Statement(Box<StatementAST>),
}
