use crate::frontend::lexer::span::Span;

use super::hir_id::HirId;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct HirTy {
    pub id: HirId,
    pub span: Span,
    pub kind: HirTyKind,
}

#[derive(Debug)]
pub enum HirTyKind {
    Function {
        parameters: Vec<HirTy>,
        return_ty: Option<Box<HirTy>>,
    },
    TypeRef(HirId),
    Number,
    Bool,
}

impl HirTy {
    pub fn function(parameters: Vec<HirTy>, return_ty: Option<HirTy>, span: Span) -> HirTy {
        HirTy {
            id: HirId::default(),
            span,
            kind: HirTyKind::Function {
                parameters,
                return_ty: return_ty.map(Box::new),
            },
        }
    }

    pub fn number(span: Span) -> HirTy {
        HirTy {
            id: HirId::default(),
            span,
            kind: HirTyKind::Number,
        }
    }

    pub fn bool(span: Span) -> HirTy {
        HirTy {
            id: HirId::default(),
            span,
            kind: HirTyKind::Bool,
        }
    }

    pub fn type_ref(id: HirId, span: Span) -> HirTy {
        HirTy {
            id,
            span,
            kind: HirTyKind::TypeRef(id),
        }
    }
}
