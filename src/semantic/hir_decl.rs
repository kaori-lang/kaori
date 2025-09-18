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
        parameters: Vec<HirDecl>,
        body: Vec<HirNode>,
    },
    Struct {
        fields: Vec<HirDecl>,
    },
    Parameter,
    Field,
}

impl HirDecl {
    pub fn parameter(id: HirId, ty: HirTy, span: Span) -> HirDecl {
        HirDecl {
            id,
            span,
            ty,
            kind: HirDeclKind::Parameter,
        }
    }

    pub fn field(id: HirId, ty: HirTy, span: Span) -> HirDecl {
        HirDecl {
            id,
            span,
            ty,
            kind: HirDeclKind::Field,
        }
    }

    pub fn struct_(id: HirId, fields: Vec<HirDecl>, ty: HirTy, span: Span) -> HirDecl {
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
        parameters: Vec<HirDecl>,
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
