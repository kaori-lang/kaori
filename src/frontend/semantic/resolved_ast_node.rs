use super::{resolved_decl::ResolvedDecl, resolved_stmt::ResolvedStmt};

pub enum ResolvedAstNode {
    Declaration(ResolvedDecl),
    Statement(ResolvedStmt),
}
