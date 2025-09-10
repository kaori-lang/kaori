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
    pub fn parameter(ty: HirTy, span: Span) -> HirDecl {
        HirDecl {
            id: HirId::default(),
            span,
            kind: HirDeclKind::Parameter { ty },
        }
    }

    pub fn field(ty: HirTy, span: Span) -> HirDecl {
        HirDecl {
            id: HirId::default(),
            span,
            kind: HirDeclKind::Field { ty },
        }
    }

    pub fn struct_(fields: Vec<HirDecl>, span: Span) -> HirDecl {
        HirDecl {
            id: HirId::default(),
            span,
            kind: HirDeclKind::Struct { fields },
        }
    }

    pub fn variable(right: HirExpr, ty: HirTy, span: Span) -> HirDecl {
        HirDecl {
            id: HirId::default(),
            span,
            kind: HirDeclKind::Variable {
                right: Box::new(right),
                ty,
            },
        }
    }

    pub fn function(
        parameters: Vec<HirDecl>,
        body: Vec<HirNode>,
        return_ty: Option<HirTy>,
        span: Span,
    ) -> HirDecl {
        HirDecl {
            id: HirId::default(),
            span,
            kind: HirDeclKind::Function {
                parameters,
                body,
                return_ty,
            },
        }
    }
}
