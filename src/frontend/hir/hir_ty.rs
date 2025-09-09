use crate::frontend::{lexer::span::Span, syntax::node_id::NodeId};

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
    Identifier,
    Number,
    Bool,
}

impl HirTy {
    pub fn function(
        id: NodeId,
        parameters: Vec<HirTy>,
        return_ty: Option<HirTy>,
        span: Span,
    ) -> HirTy {
        HirTy {
            id,
            span,
            kind: HirTyKind::Function {
                parameters,
                return_ty: return_ty.map(Box::new),
            },
        }
    }

    pub fn number(id: NodeId, span: Span) -> HirTy {
        HirTy {
            id,
            span,
            kind: HirTyKind::Number,
        }
    }

    pub fn bool(id: NodeId, span: Span) -> HirTy {
        HirTy {
            id,
            span,
            kind: HirTyKind::Bool,
        }
    }

    pub fn identifier(id: NodeId, span: Span) -> HirTy {
        HirTy {
            id,
            span,
            kind: HirTyKind::Identifier,
        }
    }
}
