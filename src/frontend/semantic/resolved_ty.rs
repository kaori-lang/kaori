use core::fmt;

use crate::frontend::scanner::span::Span;

#[derive(Clone)]
pub struct ResolvedTy {
    pub span: Span,
    pub kind: ResolvedTyKind,
}

#[derive(PartialEq, Eq, Clone)]
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

    pub fn function(parameters: Vec<ResolvedTy>, return_ty: ResolvedTy, span: Span) -> ResolvedTy {
        ResolvedTy {
            span,
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

impl fmt::Display for ResolvedTyKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResolvedTyKind::Boolean => write!(f, "boolean"),
            ResolvedTyKind::String => write!(f, "string"),
            ResolvedTyKind::Number => write!(f, "number"),
            ResolvedTyKind::Void => write!(f, "void"),
            ResolvedTyKind::Function {
                parameters,
                return_ty,
            } => {
                let params: Vec<String> = parameters.iter().map(|p| p.to_string()).collect();
                write!(f, "({}) -> {}", params.join(", "), return_ty)
            }
            ResolvedTyKind::Struct { fields } => {
                let field_strs: Vec<String> = fields.iter().map(|f| f.to_string()).collect();
                write!(f, "struct {{{}}}", field_strs.join(", "))
            }
            ResolvedTyKind::Custom { name } => write!(f, "{name}"),
        }
    }
}

impl fmt::Display for ResolvedTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}
