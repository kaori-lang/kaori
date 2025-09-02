use crate::frontend::{hir::node_id::NodeId, scanner::span::Span};

#[derive(Debug, Clone)]
pub struct HirTy {
    pub id: NodeId,
    pub span: Span,
    pub kind: HirTyKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum HirTyKind {
    Boolean,
    String,
    Number,
    Void,
    Function {
        parameters: Vec<HirTy>,
        return_ty: Box<HirTy>,
    },
    Struct {
        fields: Vec<HirTy>,
    },
    Custom {
        name: String,
    },
}

impl HirTy {
    pub fn boolean(span: Span) -> HirTy {
        HirTy {
            id: NodeId::default(),
            span,
            kind: HirTyKind::Boolean,
        }
    }

    pub fn string(span: Span) -> HirTy {
        HirTy {
            id: NodeId::default(),
            span,
            kind: HirTyKind::String,
        }
    }

    pub fn number(span: Span) -> HirTy {
        HirTy {
            id: NodeId::default(),
            span,
            kind: HirTyKind::Number,
        }
    }

    pub fn void(span: Span) -> HirTy {
        HirTy {
            id: NodeId::default(),
            span,
            kind: HirTyKind::Void,
        }
    }

    pub fn function(parameters: Vec<HirTy>, return_ty: HirTy, span: Span) -> HirTy {
        HirTy {
            id: NodeId::default(),
            span,
            kind: HirTyKind::Function {
                parameters,
                return_ty: Box::new(return_ty),
            },
        }
    }

    pub fn struct_(fields: Vec<HirTy>, span: Span) -> HirTy {
        HirTy {
            id: NodeId::default(),
            span,
            kind: HirTyKind::Struct { fields },
        }
    }

    pub fn custom(name: String, span: Span) -> HirTy {
        HirTy {
            id: NodeId::default(),
            span,
            kind: HirTyKind::Custom { name },
        }
    }
}

impl PartialEq for HirTy {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl Eq for HirTy {}
