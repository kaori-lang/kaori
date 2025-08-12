use crate::frontend::scanner::span::Span;

use super::{ast_node::AstNode, expression::Expr, ty::Ty};

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
        ty: Ty,
    },
    Function {
        name: String,
        parameters: Vec<Parameter>,
        body: Vec<AstNode>,
        ty: Ty,
    },
}

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub ty: Ty,
    pub span: Span,
}

impl Decl {
    pub fn variable(name: String, right: Expr, ty: Ty, span: Span) -> Decl {
        Decl {
            span,
            kind: DeclKind::Variable {
                name,
                right: Box::new(right),
                ty,
            },
        }
    }

    pub fn function(
        name: String,
        parameters: Vec<Parameter>,
        body: Vec<AstNode>,
        return_type: Ty,
        span: Span,
    ) -> Decl {
        let ty = Ty::function(
            parameters
                .iter()
                .map(|parameter| parameter.ty.to_owned())
                .collect(),
            Box::new(return_type),
        );

        Decl {
            span,
            kind: DeclKind::Function {
                name,
                parameters,
                body,
                ty,
            },
        }
    }
}
