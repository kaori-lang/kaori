use crate::frontend::scanner::span::Span;

use super::{ast_node::AstNode, expression::Expr, r#type::Type};
use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug)]
pub struct Decl {
    pub span: Span,
    pub kind: DeclKind,
}

#[derive(Debug)]
pub enum DeclKind {
    Variable {
        name: String,
        right: Box<Expr>,
        type_annotation: Type,
    },
    Function {
        id: usize,
        name: String,
        parameters: Vec<Parameter>,
        body: Vec<AstNode>,
        type_annotation: Type,
    },
}

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Type,
    pub span: Span,
}

impl Decl {
    pub fn variable(name: String, right: Expr, type_annotation: Type, span: Span) -> Decl {
        Decl {
            span,
            kind: DeclKind::Variable {
                name,
                right: Box::new(right),
                type_annotation,
            },
        }
    }

    pub fn function(
        name: String,
        parameters: Vec<Parameter>,
        body: Vec<AstNode>,
        return_type: Type,
        span: Span,
    ) -> Decl {
        let type_annotation = Type::function(
            parameters
                .iter()
                .map(|parameter| parameter.type_annotation.to_owned())
                .collect(),
            Box::new(return_type),
        );

        Decl {
            span,
            kind: DeclKind::Function {
                id: GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
                name,
                parameters,
                body,
                type_annotation,
            },
        }
    }
}
