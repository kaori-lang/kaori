use crate::frontend::scanner::span::Span;

use super::{
    resolved_ast_node::ResolvedAstNode, resolved_expr::ResolvedExpr, resolved_ty::ResolvedTy,
};

pub struct ResolvedDecl {
    pub span: Span,
    pub kind: ResolvedDeclKind,
}

pub enum ResolvedDeclKind {
    Variable {
        offset: usize,
        right: Box<ResolvedExpr>,
        ty: ResolvedTy,
    },
    Function {
        parameters: Vec<ResolvedParameter>,
        body: Vec<ResolvedAstNode>,
        ty: ResolvedTy,
    },
}

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
        parameters: Vec<ResolvedParameter>,
        body: Vec<ResolvedAstNode>,
        ty: ResolvedTy,
        span: Span,
    ) -> ResolvedDecl {
        ResolvedDecl {
            span,
            kind: ResolvedDeclKind::Function {
                parameters,
                body,
                ty,
            },
        }
    }
}
