use super::{decl::Decl, stmt::Stmt};

#[derive(Debug)]
pub enum Node {
    Declaration(Decl),
    Statement(Stmt),
}

impl From<Decl> for Node {
    fn from(node: Decl) -> Self {
        Self::Declaration(node)
    }
}

impl From<Stmt> for Node {
    fn from(node: Stmt) -> Self {
        Self::Statement(node)
    }
}
