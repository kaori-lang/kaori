use crate::frontend::{lexer::span::Span, syntax::node_id::NodeId};

use super::{hir_expr::HirExpr, hir_node::HirNode, hir_ty::HirTy};

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
    pub fn parameter(id: NodeId, ty: HirTy, span: Span) -> HirDecl {
        HirDecl {
            id,
            span,
            kind: HirDeclKind::Parameter { ty },
        }
    }

    pub fn field(id: NodeId, ty: HirTy, span: Span) -> HirDecl {
        HirDecl {
            id,
            span,
            kind: HirDeclKind::Field { ty },
        }
    }

    pub fn struct_(id: NodeId, fields: Vec<HirDecl>, span: Span) -> HirDecl {
        HirDecl {
            id,
            span,
            kind: HirDeclKind::Struct { fields },
        }
    }

    pub fn variable(id: NodeId, right: HirExpr, ty: HirTy, span: Span) -> HirDecl {
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
