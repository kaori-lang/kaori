use crate::frontend::scanner::span::Span;

use super::{
    ast_node::AstNode,
    expr::Expr,
    node_id::{NodeId, generate_id},
    ty::Ty,
};

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
        id: NodeId,
        name: String,
        parameters: Vec<Parameter>,
        body: Vec<AstNode>,
        ty: Ty,
    },
    Struct {
        id: NodeId,
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
    pub fn struct_(name: String, fields: Vec<Field>, span: Span) -> Decl {
        let ty = Ty::struct_(&fields);

        Decl {
            span,
            kind: DeclKind::Struct {
                id: NodeId::default(),
                name,
                fields,
                ty,
            },
        }
    }

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
        let ty = Ty::function(&parameters, return_ty);

        Decl {
            span,
            kind: DeclKind::Function {
                id: NodeId::default(),
                name,
                parameters,
                body,
                ty,
            },
        }
    }
}
