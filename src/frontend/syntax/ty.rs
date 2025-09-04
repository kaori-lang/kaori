use crate::frontend::{hir::node_id::NodeId, scanner::span::Span};

#[derive(Debug, Clone)]
pub struct Ty {
    pub id: NodeId,
    pub span: Span,
    pub kind: TyKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TyKind {
    Function {
        parameters: Vec<Ty>,
        return_ty: Box<Ty>,
    },
    Identifier(String),
}

impl Ty {
    pub fn function(parameters: Vec<Ty>, return_ty: Ty) -> Ty {
        Ty {
            id: NodeId::default(),
            span: return_ty.span,
            kind: TyKind::Function {
                parameters,
                return_ty: Box::new(return_ty),
            },
        }
    }

    pub fn identifier(name: String, span: Span) -> Ty {
        Ty {
            id: NodeId::default(),
            span,
            kind: TyKind::Identifier(name),
        }
    }
}

impl PartialEq for Ty {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl Eq for Ty {}
