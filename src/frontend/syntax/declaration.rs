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
        right: Option<Box<Expr>>,
        type_annotation: Type,
    },
    Function {
        name: String,
        parameters: Vec<Decl>,
        block: Stmt,
        type_annotation: Type,
    },
}

impl Decl {
    pub fn variable(name: String, right: Option<Expr>, type_annotation: Type, span: Span) -> Decl {
        Decl {
            span,
            kind: DeclKind::Variable {
                name,
                right: right.map(Box::new),
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
                    if let DeclKind::Variable {
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
