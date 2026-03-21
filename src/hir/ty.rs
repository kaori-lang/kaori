use crate::lexer::span::Span;

use super::node_id::NodeId;

#[derive(Debug, Clone)]
pub struct Ty {
    pub id: NodeId,
    pub span: Span,
    pub kind: TyKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TyKind {
    Function {
        parameters: Vec<Ty>,
        return_ty: Box<Ty>,
    },
    Struct {
        fields: Vec<Ty>,
    },
    TypeRef(NodeId),
    Number,
    Bool,
}

impl PartialEq for Ty {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl Eq for Ty {}

impl Ty {
    pub fn function(parameters: Vec<Ty>, return_ty: Ty, span: Span) -> Ty {
        Ty {
            id: NodeId::default(),
            span,
            kind: TyKind::Function {
                parameters,
                return_ty: Box::new(return_ty),
            },
        }
    }

    pub fn struct_(fields: Vec<Ty>, span: Span) -> Ty {
        Ty {
            id: NodeId::default(),
            span,
            kind: TyKind::Struct { fields },
        }
    }

    pub fn number(span: Span) -> Ty {
        Ty {
            id: NodeId::default(),
            span,
            kind: TyKind::Number,
        }
    }

    pub fn bool(span: Span) -> Ty {
        Ty {
            id: NodeId::default(),
            span,
            kind: TyKind::Bool,
        }
    }

    pub fn type_ref(id: NodeId, span: Span) -> Ty {
        Ty {
            id,
            span,
            kind: TyKind::TypeRef(id),
        }
    }
}
