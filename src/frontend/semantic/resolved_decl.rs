use crate::frontend::scanner::span::Span;

use super::{
    resolved_ast_node::ResolvedAstNode, resolved_expr::ResolvedExpr, resolved_ty::ResolvedTy,
};

#[derive(Debug)]
pub struct ResolvedDecl {
    pub span: Span,
    pub kind: ResolvedDeclKind,
}

#[derive(Debug)]
pub enum ResolvedDeclKind {
    Variable {
        offset: usize,
        right: Box<ResolvedExpr>,
        ty: ResolvedTy,
    },
    Function {
        id: usize,
        parameters: Vec<ResolvedParameter>,
        body: Vec<ResolvedAstNode>,
        ty: ResolvedTy,
    },
}

#[derive(Debug)]
pub struct ResolvedParameter {
    pub ty: ResolvedTy,
    pub span: Span,
}

impl ResolvedDecl {
    pub fn variable(
        offset: usize,
        right: ResolvedExpr,
        ty: ResolvedTy,
        span: Span,
    ) -> ResolvedDecl {
        ResolvedDecl {
            span,
            kind: ResolvedDeclKind::Variable {
                offset,
                right: Box::new(right),
                ty,
            },
        }
    }

    pub fn function(
        id: usize,
        parameters: Vec<ResolvedParameter>,
        body: Vec<ResolvedAstNode>,
        ty: ResolvedTy,
        span: Span,
    ) -> ResolvedDecl {
        ResolvedDecl {
            span,
            kind: ResolvedDeclKind::Function {
                id,
                parameters,
                body,
                ty,
            },
        }
    }
}
