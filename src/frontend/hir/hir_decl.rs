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
        name: String,
        right: Box<HirExpr>,
        ty: HirTy,
    },
    Function {
        name: String,
        parameters: Vec<HirDecl>,
        body: Vec<HirNode>,
        return_ty: Option<HirTy>,
    },
    Struct {
        name: String,
        fields: Vec<HirDecl>,
    },
    Parameter {
        name: String,
        ty: HirTy,
    },
    Field {
        name: String,
        ty: HirTy,
    },
}

impl HirDecl {
    pub fn parameter(id: NodeId, name: String, ty: HirTy, span: Span) -> HirDecl {
        HirDecl {
            id,
            span,
            kind: HirDeclKind::Parameter { name, ty },
        }
    }

    pub fn field(id: NodeId, name: String, ty: HirTy, span: Span) -> HirDecl {
        HirDecl {
            id,
            span,
            kind: HirDeclKind::Field { name, ty },
        }
    }

    pub fn struct_(id: NodeId, name: String, fields: Vec<HirDecl>, span: Span) -> HirDecl {
        HirDecl {
            id,
            span,
            kind: HirDeclKind::Struct { name, fields },
        }
    }

    pub fn variable(id: NodeId, name: String, right: HirExpr, ty: HirTy, span: Span) -> HirDecl {
        HirDecl {
            id,
            span,
            kind: HirDeclKind::Variable {
                name,
                right: Box::new(right),
                ty,
            },
        }
    }

    pub fn function(
        id: NodeId,
        name: String,
        parameters: Vec<HirDecl>,
        body: Vec<HirNode>,
        return_ty: Option<HirTy>,
        span: Span,
    ) -> HirDecl {
        HirDecl {
            id,
            span,
            kind: HirDeclKind::Function {
                name,
                parameters,
                body,
                return_ty,
            },
        }
    }
}
