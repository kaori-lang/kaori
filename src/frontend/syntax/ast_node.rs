use super::{decl::Decl, stmt::Stmt};

#[derive(Debug, Clone)]
pub enum AstNode {
    Declaration(Decl),
    Statement(Stmt),
}

impl From<Decl> for AstNode {
    fn from(node: Decl) -> Self {
        Self::Declaration(node)
    }
}

impl From<Stmt> for AstNode {
    fn from(node: Stmt) -> Self {
        Self::Statement(node)
    }
}
