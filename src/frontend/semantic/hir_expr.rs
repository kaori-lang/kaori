use crate::frontend::{
    lexer::span::Span,
    syntax::{binary_op::BinaryOp, node_id::NodeId, unary_op::UnaryOp},
};

#[derive(Debug)]
pub struct HirExpr {
    pub id: NodeId,
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
    Identifier,
    FunctionCall {
        callee: Box<HirExpr>,
        arguments: Vec<HirExpr>,
    },
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
}

impl HirExpr {
    pub fn binary(
        id: NodeId,
        operator: BinaryOp,
        left: HirExpr,
        right: HirExpr,
        span: Span,
    ) -> HirExpr {
        HirExpr {
            id,
            span,
            kind: HirExprKind::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn unary(id: NodeId, operator: UnaryOp, right: HirExpr, span: Span) -> HirExpr {
        HirExpr {
            id,
            span,
            kind: HirExprKind::Unary {
                operator,
                right: Box::new(right),
            },
        }
    }

    pub fn assign(id: NodeId, left: HirExpr, right: HirExpr, span: Span) -> HirExpr {
        HirExpr {
            id,
            span,
            kind: HirExprKind::Assign(Box::new(left), Box::new(right)),
        }
    }

    pub fn identifier(id: NodeId, span: Span) -> HirExpr {
        HirExpr {
            id,
            span,
            kind: HirExprKind::Identifier,
        }
    }

    pub fn function_call(
        id: NodeId,
        callee: HirExpr,
        arguments: Vec<HirExpr>,
        span: Span,
    ) -> HirExpr {
        HirExpr {
            id,
            span,
            kind: HirExprKind::FunctionCall {
                callee: Box::new(callee),
                arguments,
            },
        }
    }

    pub fn string_literal(id: NodeId, value: String, span: Span) -> HirExpr {
        HirExpr {
            id,
            span,
            kind: HirExprKind::StringLiteral(value),
        }
    }

    pub fn number_literal(id: NodeId, value: f64, span: Span) -> HirExpr {
        HirExpr {
            id,
            span,
            kind: HirExprKind::NumberLiteral(value),
        }
    }

    pub fn boolean_literal(id: NodeId, value: bool, span: Span) -> HirExpr {
        HirExpr {
            id,
            span,
            kind: HirExprKind::BooleanLiteral(value),
        }
    }
}
