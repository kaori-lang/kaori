use super::{declaration_ast::DeclarationAST, statement_ast::StatementAST};

#[derive(Debug)]
pub enum ASTNode {
    Declaration(DeclarationAST),
    Statement(StatementAST),
}
