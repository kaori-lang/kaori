use crate::frontend::lexer::span::Span;

#[derive(Debug, Clone)]
pub struct Ty {
    pub span: Span,
    pub kind: TyKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
            span: Span::default(),
            kind: TyKind::Function {
                parameters,
                return_ty: return_ty.map(Box::new),
            },
        }
    }

    pub fn number(span: Span) -> Ty {
        Ty {
            span,
            kind: TyKind::Number,
        }
    }

    pub fn bool(span: Span) -> Ty {
        Ty {
            span,
            kind: TyKind::Bool,
        }
    }

    pub fn identifier(name: String, span: Span) -> Ty {
        Ty {
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
