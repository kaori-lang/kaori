use super::{declaration_ast::DeclarationAST, statement::Stmt};

#[derive(Debug)]
pub enum ASTNode {
    Declaration(DeclarationAST),
    Statement(Stmt),
}
