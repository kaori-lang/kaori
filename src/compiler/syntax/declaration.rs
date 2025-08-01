use crate::compiler::scanner::span::Span;

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
    Function {
        name: String,
        parameters: Vec<Decl>,
        block: Stmt,
        type_annotation: Type,
    },
}

impl Decl {
    pub fn variable(name: String, right: Box<Expr>, type_annotation: Type, span: Span) -> Decl {
        Decl {
            span,
            kind: DeclKind::Variable {
                name,
                right,
                type_annotation,
            },
        }
    }

    pub fn function(
        name: String,
        parameters: Vec<Decl>,
        block: Stmt,
        type_annotation: Type,
        span: Span,
    ) -> Decl {
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
