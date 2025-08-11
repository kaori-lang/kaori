use super::resolved_stmt::ResolvedStmt;

#[derive(Debug)]
pub enum ResolvedAstNode {
    Declaration(ResolvedDecl),
    Statement(ResolvedStmt),
}
