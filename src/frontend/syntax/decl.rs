use crate::frontend::scanner::span::Span;

use super::{ast_node::AstNode, expr::Expr, node_id::generate_id, ty::Ty};

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
        id: usize,
        name: String,
        parameters: Vec<Parameter>,
        body: Vec<AstNode>,
        ty: Ty,
    },
    Struct {
        id: usize,
        name: String,
        fields: Vec<Field>,
        ty: Ty,
    },
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub ty: Ty,
    pub span: Span,
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
        return_ty: Ty,
        span: Span,
    ) -> Decl {
        let ty = Ty::function(
            parameters
                .iter()
                .map(|parameter| parameter.ty.to_owned())
                .collect(),
            return_ty,
        );

        Decl {
            span,
            kind: DeclKind::Function {
                id: generate_id(),
                name,
                parameters,
                body,
                ty,
            },
        }
    }
}
