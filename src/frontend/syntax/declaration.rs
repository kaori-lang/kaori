use crate::frontend::scanner::span::Span;

use super::{expression::Expr, statement::Stmt, r#type::Type};

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
    Parameter {
        name: String,
        type_annotation: Type,
    },
    Function {
        name: String,
        parameters: Vec<Parameter>,
        block: Stmt,
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
    pub fn parameter(name: String, type_annotation: Type, span: Span) -> Decl {
        Decl {
            span,
            kind: DeclKind::Parameter {
                name,
                type_annotation,
            },
        }
    }

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
        parameters: Vec<Decl>,
        block: Stmt,
        return_type: Type,
        span: Span,
    ) -> Decl {
        let type_annotation = Type::function(
            parameters
                .iter()
                .map(|p| {
                    if let DeclKind::Parameter {
                        type_annotation, ..
                    } = &p.kind
                    {
                        type_annotation.clone()
                    } else {
                        unreachable!()
                    }
                })
                .collect(),
            Box::new(return_type),
        );

        Decl {
            span,
            kind: DeclKind::Function {
                name,
                parameters,
                block,
                type_annotation,
            },
        }
    }
}
