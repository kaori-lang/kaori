use super::{decl::Decl, stmt::Stmt};

#[derive(Debug)]
pub enum AstNode {
    Declaration(Decl),
    Statement(Stmt),
}
