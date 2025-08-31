use crate::frontend::{scanner::span::Span, syntax::ty::Ty};

use super::{hir_ast_node::HirAstNode, hir_expr::HirExpr, node_id::NodeId};

#[derive(Debug)]
pub struct HirDecl {
    pub id: NodeId,
    pub span: Span,
    pub kind: HirDeclKind,
}

#[derive(Debug)]
pub enum HirDeclKind {
    Variable {
        name: String,
        right: Box<HirExpr>,
        ty: Ty,
    },
    Function {
        name: String,
        parameters: Vec<Parameter>,
        body: Vec<HirAstNode>,
        ty: Ty,
    },
    Struct {
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

impl HirDecl {
    pub fn struct_(name: String, fields: Vec<Field>, ty: Ty, span: Span) -> HirDecl {
        HirDecl {
            id: NodeId::default(),
            span,
            kind: HirDeclKind::Struct { name, fields, ty },
        }
    }

    pub fn variable(name: String, right: HirExpr, ty: Ty, span: Span) -> HirDecl {
        HirDecl {
            id: NodeId::default(),
            span,
            kind: HirDeclKind::Variable {
                name,
                right: Box::new(right),
                ty,
            },
        }
    }

    pub fn function(
        name: String,
        parameters: Vec<Parameter>,
        body: Vec<HirAstNode>,
        ty: Ty,
        span: Span,
    ) -> HirDecl {
        HirDecl {
            id: NodeId::default(),
            span,
            kind: HirDeclKind::Function {
                name,
                parameters,
                body,
                ty,
            },
        }
    }
}
