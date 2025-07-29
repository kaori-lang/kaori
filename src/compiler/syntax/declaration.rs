use crate::compiler::lexer::span::Span;

use super::{ast_node::ASTNode, expression::Expr, r#type::Type, statement::Stmt};

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
