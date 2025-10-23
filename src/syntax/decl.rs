use crate::lexer::span::Span;

use super::{ast_id::AstId, ast_node::AstNode, expr::Expr, ty::Ty};

#[derive(Debug)]
pub struct Decl {
    pub id: AstId,
    pub span: Span,
    pub kind: DeclKind,
}

#[derive(Debug)]
pub enum DeclKind {
    Variable {
        name: String,
        right: Expr,
        ty: Option<Ty>,
    },
    Function {
        name: String,
        parameters: Vec<Parameter>,
        body: Vec<AstNode>,
        ty: Ty,
    },
    Struct {
        name: String,
        fields: Vec<Field>,
        ty: Ty,
    },
}

#[derive(Debug)]
pub struct Parameter {
    pub id: AstId,
    pub span: Span,
    pub name: String,
    pub ty: Ty,
}

#[derive(Debug)]
pub struct Field {
    pub id: AstId,
    pub span: Span,
    pub name: String,
    pub ty: Ty,
}

impl Parameter {
    pub fn new(name: String, ty: Ty, span: Span) -> Parameter {
        Parameter {
            id: AstId::default(),
            span,
            name,
            ty,
        }
    }
}

impl Field {
    pub fn new(name: String, ty: Ty, span: Span) -> Field {
        Field {
            id: AstId::default(),
            span,
            name,
            ty,
        }
    }
}

impl Decl {
    pub fn struct_(name: String, fields: Vec<Field>, ty: Ty, span: Span) -> Decl {
        Decl {
            id: AstId::default(),
            span,

            kind: DeclKind::Struct { name, fields, ty },
        }
    }

    pub fn variable(name: String, right: Expr, ty: Option<Ty>, span: Span) -> Decl {
        Decl {
            id: AstId::default(),
            span,

            kind: DeclKind::Variable { name, right, ty },
        }
    }

    pub fn function(
        name: String,
        parameters: Vec<Parameter>,
        body: Vec<AstNode>,
        ty: Ty,
        span: Span,
    ) -> Decl {
        Decl {
            id: AstId::default(),
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
