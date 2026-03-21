use crate::lexer::span::Span;

use super::{hir_expr::HirExpr, hir_node::HirNode, hir_ty::HirTy, node_id::NodeId};

#[derive(Debug)]
pub struct HirDecl {
    pub id: NodeId,
    pub span: Span,
    pub kind: HirDeclKind,
}

#[derive(Debug)]
pub enum HirDeclKind {
    Variable {
        right: Box<HirExpr>,
        ty: Option<HirTy>,
    },
    Function {
        parameters: Vec<HirParameter>,
        body: Vec<HirNode>,
        return_ty: Option<HirTy>,
    },
    Struct {
        fields: Vec<HirField>,
    },
}

#[derive(Debug)]
pub struct HirParameter {
    pub id: NodeId,
    pub span: Span,
    pub ty: HirTy,
}

#[derive(Debug)]
pub struct HirField {
    pub id: NodeId,
    pub span: Span,
    pub ty: HirTy,
}

impl HirParameter {
    pub fn new(id: NodeId, ty: HirTy, span: Span) -> HirParameter {
        HirParameter { id, span, ty }
    }
}

impl HirField {
    pub fn new(id: NodeId, ty: HirTy, span: Span) -> HirField {
        HirField { id, span, ty }
    }
}

impl HirDecl {
    pub fn struct_(id: NodeId, fields: Vec<HirField>, span: Span) -> HirDecl {
        HirDecl {
            id,
            span,

            kind: HirDeclKind::Struct { fields },
        }
    }

    pub fn variable(id: NodeId, right: HirExpr, ty: Option<HirTy>, span: Span) -> HirDecl {
        HirDecl {
            id,
            span,

            kind: HirDeclKind::Variable {
                right: Box::new(right),
                ty,
            },
        }
    }

    pub fn function(
        id: NodeId,
        parameters: Vec<HirParameter>,
        body: Vec<HirNode>,
        return_ty: Option<HirTy>,
        span: Span,
    ) -> HirDecl {
        HirDecl {
            id,
            span,
            kind: HirDeclKind::Function {
                parameters,
                body,
                return_ty,
            },
        }
    }
}
