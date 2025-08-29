use crate::frontend::scanner::span::Span;

#[derive(Debug, Clone)]
pub struct Ty {
    pub span: Span,
    pub kind: TyKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TyKind {
    Boolean,
    String,
    Number,
    Void,
    Function {
        parameters: Vec<Ty>,
        return_ty: Box<Ty>,
    },
    Struct {
        fields: Vec<Ty>,
    },
    Custom {
        name: String,
    },
}

impl Ty {
    pub fn boolean(span: Span) -> Ty {
        Ty {
            span,
            kind: TyKind::Boolean,
        }
    }

    pub fn string(span: Span) -> Ty {
        Ty {
            span,
            kind: TyKind::String,
        }
    }

    pub fn number(span: Span) -> Ty {
        Ty {
            span,
            kind: TyKind::Number,
        }
    }

    pub fn void(span: Span) -> Ty {
        Ty {
            span,
            kind: TyKind::Void,
        }
    }

    pub fn function(parameters: Vec<Ty>, return_ty: Ty) -> Ty {
        Ty {
            span: return_ty.span,
            kind: TyKind::Function {
                parameters,
                return_ty: Box::new(return_ty),
            },
        }
    }

    pub fn struct_(fields: Vec<Ty>) -> Ty {
        Ty {
            span: Span::default(),
            kind: TyKind::Struct { fields },
        }
    }

    pub fn custom(name: String, span: Span) -> Ty {
        Ty {
            span,
            kind: TyKind::Custom { name },
        }
    }
}

impl PartialEq for Ty {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl Eq for Ty {}
