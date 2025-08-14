use crate::frontend::{
    scanner::span::Span,
    syntax::{decl::Parameter, ty::Ty},
};

use super::{resolved_ast_node::ResolvedAstNode, resolved_expr::ResolvedExpr};

#[derive(Debug)]
pub struct ResolvedDecl {
    pub span: Span,
    pub kind: ResolvedDeclKind,
}

#[derive(Debug)]
pub enum ResolvedDeclKind {
    Variable {
        right: Box<ResolvedExpr>,
        ty: Ty,
    },
    Function {
        id: usize,
        parameters: Vec<ResolvedParameter>,
        body: Vec<ResolvedAstNode>,
        ty: Ty,
    },
}

#[derive(Debug)]
pub struct ResolvedParameter {
    pub ty: Ty,
    pub span: Span,
}

impl ResolvedDecl {
    pub fn variable(right: ResolvedExpr, ty: Ty, span: Span) -> ResolvedDecl {
        ResolvedDecl {
            span,
            kind: ResolvedDeclKind::Variable {
                right: Box::new(right),
                ty,
            },
        }
    }

    pub fn function(
        id: usize,
        parameters: &[Parameter],
        body: Vec<ResolvedAstNode>,
        ty: Ty,
        span: Span,
    ) -> ResolvedDecl {
        let parameters = parameters
            .iter()
            .map(|parameter| ResolvedParameter {
                ty: parameter.ty.to_owned(),
                span: parameter.span,
            })
            .collect();

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
