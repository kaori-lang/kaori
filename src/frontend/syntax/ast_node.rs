use super::{declaration::Decl, statement::Stmt};

#[derive(Debug)]
pub enum ASTNode {
    Declaration(Decl),
    Statement(Stmt),
}
