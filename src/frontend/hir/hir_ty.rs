use crate::frontend::{hir::node_id::NodeId, lexer::span::Span};

#[derive(Debug)]
pub struct HirTy {
    pub id: NodeId,
    pub span: Span,
    pub kind: HirTyKind,
}

#[derive(Debug)]
pub enum HirTyKind {
    Function {
        parameters: Vec<HirTy>,
        return_ty: Option<Box<HirTy>>,
    },
    Identifier(String),
    Number,
    Bool,
}

impl HirTy {
    pub fn function(parameters: Vec<HirTy>, return_ty: Option<HirTy>, span: Span) -> HirTy {
        HirTy {
            id: NodeId::default(),
            span,
            kind: HirTyKind::Function {
                parameters,
                return_ty: return_ty.map(Box::new),
            },
        }
    }

    pub fn number(span: Span) -> HirTy {
        HirTy {
            id: NodeId::default(),
            span,
            kind: HirTyKind::Number,
        }
    }

    pub fn bool(span: Span) -> HirTy {
        HirTy {
            id: NodeId::default(),
            span,
            kind: HirTyKind::Bool,
        }
    }

    pub fn identifier(name: String, span: Span) -> HirTy {
        HirTy {
            id: NodeId::default(),
            span,
            kind: HirTyKind::Identifier(name),
        }
    }
}
