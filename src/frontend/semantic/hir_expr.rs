use crate::frontend::{
    lexer::span::Span,
    syntax::{binary_op::BinaryOp, unary_op::UnaryOp},
};

use super::hir_id::HirId;

#[derive(Debug)]
pub struct HirExpr {
    pub id: HirId,
    pub span: Span,
    pub kind: HirExprKind,
}

#[derive(Debug)]
pub enum HirExprKind {
    Binary {
        operator: BinaryOp,
        left: Box<HirExpr>,
        right: Box<HirExpr>,
    },
    Unary {
        right: Box<HirExpr>,
        operator: UnaryOp,
    },
    Assign(Box<HirExpr>, Box<HirExpr>),
    Variable {
        offset: usize,
    },
    FunctionRef(HirId),
    FunctionCall {
        callee: Box<HirExpr>,
        arguments: Vec<HirExpr>,
    },
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
}

impl HirExpr {
    pub fn binary(operator: BinaryOp, left: HirExpr, right: HirExpr, span: Span) -> HirExpr {
        HirExpr {
            id: HirId::default(),
            span,
            kind: HirExprKind::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn unary(operator: UnaryOp, right: HirExpr, span: Span) -> HirExpr {
        HirExpr {
            id: HirId::default(),
            span,
            kind: HirExprKind::Unary {
                operator,
                right: Box::new(right),
            },
        }
    }

    pub fn assign(left: HirExpr, right: HirExpr, span: Span) -> HirExpr {
        HirExpr {
            id: HirId::default(),
            span,
            kind: HirExprKind::Assign(Box::new(left), Box::new(right)),
        }
    }

    pub fn variable(offset: usize, span: Span) -> HirExpr {
        HirExpr {
            id: HirId::default(),
            span,
            kind: HirExprKind::Variable { offset },
        }
    }

    pub fn function_ref(id: HirId, span: Span) -> HirExpr {
        HirExpr {
            id,
            span,
            kind: HirExprKind::FunctionRef(id),
        }
    }

    pub fn function_call(callee: HirExpr, arguments: Vec<HirExpr>, span: Span) -> HirExpr {
        HirExpr {
            id: HirId::default(),
            span,
            kind: HirExprKind::FunctionCall {
                callee: Box::new(callee),
                arguments,
            },
        }
    }

    pub fn string_literal(value: String, span: Span) -> HirExpr {
        HirExpr {
            id: HirId::default(),
            span,
            kind: HirExprKind::StringLiteral(value),
        }
    }

    pub fn number_literal(value: f64, span: Span) -> HirExpr {
        HirExpr {
            id: HirId::default(),
            span,
            kind: HirExprKind::NumberLiteral(value),
        }
    }

    pub fn boolean_literal(value: bool, span: Span) -> HirExpr {
        HirExpr {
            id: HirId::default(),
            span,
            kind: HirExprKind::BooleanLiteral(value),
        }
    }
}
