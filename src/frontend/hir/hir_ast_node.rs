use super::{hir_decl::HirDecl, hir_stmt::HirStmt};

#[derive(Debug)]
pub enum HirAstNode {
    Declaration(HirDecl),
    Statement(HirStmt),
}
