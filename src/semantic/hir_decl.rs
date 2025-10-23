use crate::lexer::span::Span;

use super::{hir_expr::HirExpr, hir_id::HirId, hir_node::HirNode, hir_ty::HirTy};

#[derive(Debug)]
pub struct HirDecl {
    pub id: HirId,
    pub span: Span,
    pub ty: HirTy,
    pub kind: HirDeclKind,
}

#[derive(Debug)]
pub enum HirDeclKind {
    Variable {
        right: Box<HirExpr>,
    },
    Function {
        parameters: Vec<HirParameter>,
        body: Vec<HirNode>,
    },
    Struct {
        fields: Vec<HirField>,
    },
}

#[derive(Debug)]
pub struct HirParameter {
    pub id: HirId,
    pub span: Span,
    pub ty: HirTy,
}

#[derive(Debug)]
pub struct HirField {
    pub id: HirId,
    pub span: Span,
    pub ty: HirTy,
}

impl HirParameter {
    pub fn new(id: HirId, ty: HirTy, span: Span) -> HirParameter {
        HirParameter { id, span, ty }
    }
}

impl HirField {
    pub fn new(id: HirId, ty: HirTy, span: Span) -> HirField {
        HirField { id, span, ty }
    }
}

impl HirDecl {
    pub fn struct_(id: HirId, fields: Vec<HirField>, ty: HirTy, span: Span) -> HirDecl {
        HirDecl {
            id,
            span,
            ty,
            kind: HirDeclKind::Struct { fields },
        }
    }

    pub fn variable(id: HirId, right: HirExpr, ty: HirTy, span: Span) -> HirDecl {
        HirDecl {
            id,
            span,
            ty,
            kind: HirDeclKind::Variable {
                right: Box::new(right),
            },
        }
    }

    pub fn function(
        id: HirId,
        parameters: Vec<HirParameter>,
        body: Vec<HirNode>,
        ty: HirTy,
        span: Span,
    ) -> HirDecl {
        HirDecl {
            id,
            span,
            ty,
            kind: HirDeclKind::Function { parameters, body },
        }
    }
}
