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
        parameters: Vec<HirDecl>,
        body: Vec<HirAstNode>,
        return_ty: Ty,
    },
    Struct {
        name: String,
        fields: Vec<HirDecl>,
    },
    Parameter {
        name: String,
        ty: Ty,
    },
    Field {
        name: String,
        ty: Ty,
    },
}

impl HirDecl {
    pub fn parameter(name: String, ty: Ty, span: Span) -> HirDecl {
        HirDecl {
            id: NodeId::default(),
            span,
            kind: HirDeclKind::Parameter { name, ty },
        }
    }

    pub fn field(name: String, ty: Ty, span: Span) -> HirDecl {
        HirDecl {
            id: NodeId::default(),
            span,
            kind: HirDeclKind::Field { name, ty },
        }
    }

    pub fn struct_(name: String, fields: Vec<HirDecl>, span: Span) -> HirDecl {
        HirDecl {
            id: NodeId::default(),
            span,
            kind: HirDeclKind::Struct { name, fields },
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
        parameters: Vec<HirDecl>,
        body: Vec<HirAstNode>,
        return_ty: Ty,
        span: Span,
    ) -> HirDecl {
        HirDecl {
            id: NodeId::default(),
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
