use crate::frontend::{
    scanner::span::Span,
    syntax::{declaration::Parameter, r#type::Type},
};

use std::sync::atomic::{AtomicUsize, Ordering};

use super::{resolved_ast_node::ResolvedAstNode, resolved_expr::ResolvedExpr};

static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug)]
pub struct ResolvedDecl {
    pub span: Span,
    pub kind: ResolvedDeclKind,
}

#[derive(Debug)]
pub enum ResolvedDeclKind {
    Variable {
        right: Box<ResolvedExpr>,
        type_annotation: Type,
    },
    Function {
        id: usize,
        parameters: Vec<ResolvedParameter>,
        body: Vec<ResolvedAstNode>,
        type_annotation: Type,
    },
}

#[derive(Debug)]
pub struct ResolvedParameter {
    pub type_annotation: Type,
    pub span: Span,
}

impl ResolvedDecl {
    pub fn variable(right: ResolvedExpr, type_annotation: Type, span: Span) -> ResolvedDecl {
        ResolvedDecl {
            span,
            kind: ResolvedDeclKind::Variable {
                right: Box::new(right),
                type_annotation,
            },
        }
    }

    pub fn function(
        parameters: &[Parameter],
        body: Vec<ResolvedAstNode>,
        type_annotation: Type,
        span: Span,
    ) -> ResolvedDecl {
        let parameters = parameters
            .iter()
            .map(|parameter| ResolvedParameter {
                type_annotation: parameter.type_annotation.to_owned(),
                span: parameter.span,
            })
            .collect();

        ResolvedDecl {
            span,
            kind: ResolvedDeclKind::Function {
                id: GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
                parameters,
                body,
                type_annotation,
            },
        }
    }
}
