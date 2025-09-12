use crate::frontend::lexer::span::Span;

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
        offset: usize,
        right: Box<HirExpr>,
    },
    Function {
        parameters: Vec<HirDecl>,
        body: Vec<HirNode>,
    },
    Struct {
        fields: Vec<HirDecl>,
    },
    Parameter {
        offset: usize,
    },
    Field {
        offset: usize,
    },
}

impl HirDecl {
    pub fn parameter(id: HirId, offset: usize, ty: HirTy, span: Span) -> HirDecl {
        HirDecl {
            id,
            span,
            ty,
            kind: HirDeclKind::Parameter { offset },
        }
    }

    pub fn field(id: HirId, offset: usize, ty: HirTy, span: Span) -> HirDecl {
        HirDecl {
            id,
            span,
            ty,
            kind: HirDeclKind::Field { offset },
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

    pub fn variable(id: HirId, offset: usize, right: HirExpr, ty: HirTy, span: Span) -> HirDecl {
        HirDecl {
            id,
            span,
            ty,
            kind: HirDeclKind::Variable {
                offset,
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
