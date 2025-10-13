use crate::lexer::span::Span;

use super::ast_id::AstId;

#[derive(Debug, Clone)]
pub struct Ty {
    pub id: AstId,
    pub span: Span,
    pub kind: TyKind,
}

#[derive(Debug, Clone)]
pub enum TyKind {
    Function {
        parameters: Vec<Ty>,
        return_ty: Box<Ty>,
    },
    Struct {
        fields: Vec<Ty>,
    },
    Identifier(String),
    Number,
    Bool,
    Void,
}

impl Ty {
    pub fn function(parameters: Vec<Ty>, return_ty: Ty, span: Span) -> Ty {
        Ty {
            id: AstId::default(),
            span,
            kind: TyKind::Function {
                parameters,
                return_ty: Box::new(return_ty),
            },
        }
    }

    pub fn struct_(fields: Vec<Ty>, span: Span) -> Ty {
        Ty {
            id: AstId::default(),
            span,
            kind: TyKind::Struct { fields },
        }
    }

    pub fn number(span: Span) -> Ty {
        Ty {
            id: AstId::default(),
            span,
            kind: TyKind::Number,
        }
    }

    pub fn bool(span: Span) -> Ty {
        Ty {
            id: AstId::default(),
            span,
            kind: TyKind::Bool,
        }
    }

    pub fn void(span: Span) -> Ty {
        Ty {
            id: AstId::default(),
            span,
            kind: TyKind::Void,
        }
    }

    pub fn identifier(name: String, span: Span) -> Ty {
        Ty {
            id: AstId::default(),
            span,
            kind: TyKind::Identifier(name),
        }
    }
}
