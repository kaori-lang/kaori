use crate::frontend::scanner::span::Span;

#[derive(Debug, Clone)]
pub struct ResolvedTy {
    pub span: Span,
    pub kind: ResolvedTyKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ResolvedTyKind {
    Boolean,
    String,
    Number,
    Void,
    Function {
        parameters: Vec<ResolvedTy>,
        return_ty: Box<ResolvedTy>,
    },
    Struct {
        fields: Vec<ResolvedTy>,
    },
    Custom {
        name: String,
    },
}

impl ResolvedTy {
    pub fn boolean(span: Span) -> ResolvedTy {
        ResolvedTy {
            span,
            kind: ResolvedTyKind::Boolean,
        }
    }

    pub fn string(span: Span) -> ResolvedTy {
        ResolvedTy {
            span,
            kind: ResolvedTyKind::String,
        }
    }

    pub fn number(span: Span) -> ResolvedTy {
        ResolvedTy {
            span,
            kind: ResolvedTyKind::Number,
        }
    }

    pub fn void(span: Span) -> ResolvedTy {
        ResolvedTy {
            span,
            kind: ResolvedTyKind::Void,
        }
    }

    pub fn function(parameters: Vec<ResolvedTy>, return_ty: ResolvedTy) -> ResolvedTy {
        ResolvedTy {
            span: return_ty.span,
            kind: ResolvedTyKind::Function {
                parameters,
                return_ty: Box::new(return_ty),
            },
        }
    }

    pub fn struct_(fields: Vec<ResolvedTy>, span: Span) -> ResolvedTy {
        ResolvedTy {
            span,
            kind: ResolvedTyKind::Struct { fields },
        }
    }

    pub fn custom(name: String, span: Span) -> ResolvedTy {
        ResolvedTy {
            span,
            kind: ResolvedTyKind::Custom { name },
        }
    }
}

impl PartialEq for ResolvedTy {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl Eq for ResolvedTy {}
