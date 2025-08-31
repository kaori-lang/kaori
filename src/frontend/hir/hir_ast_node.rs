use super::hir_stmt::HirStmt;

#[derive(Debug)]
pub enum HirAstNode {
    Declaration(Decl),
    Statement(HirStmt),
}
