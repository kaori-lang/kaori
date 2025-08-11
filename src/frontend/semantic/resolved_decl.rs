use crate::frontend::{scanner::span::Span, syntax::r#type::Type};

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
        name: String,
        right: Box<ResolvedExpr>,
        type_annotation: Type,
    },
    Function {
        id: usize,
        name: String,
        parameters: Vec<Parameter>,
        body: Vec<ResolvedAstNode>,
        type_annotation: Type,
    },
}

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Type,
    pub span: Span,
}

impl ResolvedDecl {
    pub fn variable(
        name: String,
        right: ResolvedExpr,
        type_annotation: Type,
        span: Span,
    ) -> ResolvedDecl {
        ResolvedDecl {
            span,
            kind: ResolvedDeclKind::Variable {
                name,
                right: Box::new(right),
                type_annotation,
            },
        }
    }

    pub fn function(
        id: usize,
        name: String,
        parameters: Vec<Parameter>,
        body: Vec<ResolvedAstNode>,
        type_annotation: Type,
        span: Span,
    ) -> ResolvedDecl {
        ResolvedDecl {
            span,
            kind: ResolvedDeclKind::Function {
                id,
                name,
                parameters,
                body,
                type_annotation,
            },
        }
    }
}
