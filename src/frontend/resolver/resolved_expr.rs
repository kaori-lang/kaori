use crate::frontend::{
    scanner::span::Span,
    syntax::{
        operator::{BinaryOp, UnaryOp},
        r#type::Type,
    },
};

#[derive(Debug)]
pub struct ResolvedExpr {
    pub span: Span,
    pub kind: ResolvedExprKind,
}

#[derive(Debug)]
pub enum ResolvedExprKind {
    Binary {
        operator: BinaryOp,
        left: Box<ResolvedExpr>,
        right: Box<ResolvedExpr>,
    },
    Unary {
        operator: UnaryOp,
        right: Box<ResolvedExpr>,
    },
    Assign {
        identifier: Box<ResolvedExpr>,
        right: Box<ResolvedExpr>,
    },
    VariableRef {
        offset: usize,
        type_annotation: Type,
    },
    FunctionRef {
        function_id: usize,
        type_annotation: Type,
    },
    FunctionCall {
        callee: Box<ResolvedExpr>,
        arguments: Vec<ResolvedExpr>,
    },
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
}

impl ResolvedExpr {
    pub fn binary(
        operator: BinaryOp,
        left: ResolvedExpr,
        right: ResolvedExpr,
        span: Span,
    ) -> ResolvedExpr {
        ResolvedExpr {
            span,
            kind: ResolvedExprKind::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn unary(operator: UnaryOp, right: ResolvedExpr, span: Span) -> ResolvedExpr {
        ResolvedExpr {
            span,
            kind: ResolvedExprKind::Unary {
                operator,
                right: Box::new(right),
            },
        }
    }

    pub fn assign(identifier: ResolvedExpr, right: ResolvedExpr, span: Span) -> ResolvedExpr {
        ResolvedExpr {
            span,
            kind: ResolvedExprKind::Assign {
                identifier: Box::new(identifier),
                right: Box::new(right),
            },
        }
    }

    pub fn variable_ref(offset: usize, type_annotation: Type, span: Span) -> ResolvedExpr {
        ResolvedExpr {
            span,
            kind: ResolvedExprKind::VariableRef {
                offset,
                type_annotation,
            },
        }
    }

    pub fn function_ref(function_id: usize, type_annotation: Type, span: Span) -> ResolvedExpr {
        ResolvedExpr {
            span,
            kind: ResolvedExprKind::FunctionRef {
                function_id,
                type_annotation,
            },
        }
    }

    pub fn function_call(
        callee: ResolvedExpr,
        arguments: Vec<ResolvedExpr>,
        span: Span,
    ) -> ResolvedExpr {
        ResolvedExpr {
            span,
            kind: ResolvedExprKind::FunctionCall {
                callee: Box::new(callee),
                arguments,
            },
        }
    }

    pub fn string_literal(value: String, span: Span) -> ResolvedExpr {
        ResolvedExpr {
            span,
            kind: ResolvedExprKind::StringLiteral(value),
        }
    }

    pub fn number_literal(value: f64, span: Span) -> ResolvedExpr {
        ResolvedExpr {
            span,
            kind: ResolvedExprKind::NumberLiteral(value),
        }
    }

    pub fn boolean_literal(value: bool, span: Span) -> ResolvedExpr {
        ResolvedExpr {
            span,
            kind: ResolvedExprKind::BooleanLiteral(value),
        }
    }
}
