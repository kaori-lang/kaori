use super::{hir_decl::HirDecl, hir_stmt::HirStmt};

#[derive(Debug)]
pub enum HirNode {
    Declaration(HirDecl),
    Statement(HirStmt),
}

impl From<HirDecl> for HirNode {
    fn from(node: HirDecl) -> Self {
        Self::Declaration(node)
    }
}

impl From<HirStmt> for HirNode {
    fn from(node: HirStmt) -> Self {
        Self::Statement(node)
    }
}
