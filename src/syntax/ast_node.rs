use super::{declaration_ast::DeclarationAST, statement_ast::StatementAST};

pub enum ASTNode {
    Declaration(Box<DeclarationAST>),
    Statement(Box<StatementAST>),
}
