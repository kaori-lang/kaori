use super::{hir_decl::HirDecl, hir_stmt::HirStmt};

#[derive(Debug)]
pub enum HirAstNode {
    Declaration(HirDecl),
    Statement(HirStmt),
}

impl From<HirDecl> for HirAstNode {
    fn from(node: HirDecl) -> Self {
        Self::Declaration(node)
    }
}

impl From<HirStmt> for HirAstNode {
    fn from(node: HirStmt) -> Self {
        Self::Statement(node)
    }
}
