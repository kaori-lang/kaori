use super::{declaration::Decl, statement::Stmt};

#[derive(Debug)]
pub enum AstNode {
    Declaration(Decl),
    Statement(Stmt),
}
