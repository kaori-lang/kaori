use crate::frontend::lexer::span::Span;

use super::{hir_expr::HirExpr, hir_id::HirId, hir_node::HirNode, hir_ty::HirTy};

#[derive(Debug)]
pub struct HirDecl {
    pub id: HirId,
    pub span: Span,
    pub kind: HirDeclKind,
}

#[derive(Debug)]
pub enum HirDeclKind {
    Variable {
        offset: usize,
        right: Box<HirExpr>,
        ty: HirTy,
    },
    Function {
        parameters: Vec<HirDecl>,
        body: Vec<HirNode>,
        return_ty: Option<HirTy>,
    },
    Struct {
        fields: Vec<HirDecl>,
    },
    Parameter {
        ty: HirTy,
    },
    Field {
        ty: HirTy,
    },
}

impl HirDecl {
    pub fn parameter(id: HirId, ty: HirTy, span: Span) -> HirDecl {
        HirDecl {
            id,
            span,
            kind: HirDeclKind::Parameter { ty },
        }
    }

    pub fn field(id: HirId, ty: HirTy, span: Span) -> HirDecl {
        HirDecl {
            id,
            span,
            kind: HirDeclKind::Field { ty },
        }
    }

    pub fn struct_(id: HirId, fields: Vec<HirDecl>, span: Span) -> HirDecl {
        HirDecl {
            id,
            span,
            kind: HirDeclKind::Struct { fields },
        }
    }

    pub fn variable(id: HirId, offset: usize, right: HirExpr, ty: HirTy, span: Span) -> HirDecl {
        HirDecl {
            id,
            span,
            kind: HirDeclKind::Variable {
                offset,
                right: Box::new(right),
                ty,
            },
        }
    }

    pub fn function(
        id: HirId,
        parameters: Vec<HirDecl>,
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
