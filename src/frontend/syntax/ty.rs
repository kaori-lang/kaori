use crate::frontend::lexer::span::Span;

use super::ast_id::AstId;

#[derive(Debug)]
pub struct Ty {
    pub id: AstId,
    pub span: Span,
    pub kind: TyKind,
}

#[derive(Debug)]
pub enum TyKind {
    Function {
        parameters: Vec<Ty>,
        return_ty: Option<Box<Ty>>,
    },
    Identifier(String),
    Number,
    Bool,
}

impl Ty {
    pub fn function(parameters: Vec<Ty>, return_ty: Option<Ty>) -> Ty {
        Ty {
            id: AstId::default(),
            span: Span::default(),
            kind: TyKind::Function {
                parameters,
                return_ty: return_ty.map(Box::new),
            },
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

    pub fn identifier(name: String, span: Span) -> Ty {
        Ty {
            id: AstId::default(),
            span,
            kind: TyKind::Identifier(name),
        }
    }
}
