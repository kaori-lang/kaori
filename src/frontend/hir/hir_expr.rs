use crate::frontend::scanner::span::Span;

use super::node_id::NodeId;

#[derive(Debug, Clone)]
pub struct HirExpr {
    pub id: NodeId,
    pub span: Span,
    pub kind: HirExprKind,
}

#[derive(Debug, Clone)]
pub enum HirExprKind {
    Add(Box<HirExpr>, Box<HirExpr>),
    Sub(Box<HirExpr>, Box<HirExpr>),
    Mul(Box<HirExpr>, Box<HirExpr>),
    Div(Box<HirExpr>, Box<HirExpr>),
    Mod(Box<HirExpr>, Box<HirExpr>),
    Equal(Box<HirExpr>, Box<HirExpr>),
    NotEqual(Box<HirExpr>, Box<HirExpr>),
    Less(Box<HirExpr>, Box<HirExpr>),
    LessEqual(Box<HirExpr>, Box<HirExpr>),
    Greater(Box<HirExpr>, Box<HirExpr>),
    GreaterEqual(Box<HirExpr>, Box<HirExpr>),
    And(Box<HirExpr>, Box<HirExpr>),
    Or(Box<HirExpr>, Box<HirExpr>),
    Negate(Box<HirExpr>),
    Not(Box<HirExpr>),
    Assign(Box<HirExpr>, Box<HirExpr>),
    Identifier(String),
    FunctionCall {
        callee: Box<HirExpr>,
        arguments: Vec<HirExpr>,
    },
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
}

impl HirExpr {
    pub fn add(left: HirExpr, right: HirExpr, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::Add(Box::new(left), Box::new(right)),
        }
    }

    pub fn sub(left: HirExpr, right: HirExpr, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::Sub(Box::new(left), Box::new(right)),
        }
    }

    pub fn mul(left: HirExpr, right: HirExpr, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::Mul(Box::new(left), Box::new(right)),
        }
    }

    pub fn div(left: HirExpr, right: HirExpr, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::Div(Box::new(left), Box::new(right)),
        }
    }

    pub fn mod_(left: HirExpr, right: HirExpr, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::Div(Box::new(left), Box::new(right)),
        }
    }

    pub fn equal(left: HirExpr, right: HirExpr, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::Equal(Box::new(left), Box::new(right)),
        }
    }

    pub fn not_equal(left: HirExpr, right: HirExpr, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::NotEqual(Box::new(left), Box::new(right)),
        }
    }

    pub fn less(left: HirExpr, right: HirExpr, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::Less(Box::new(left), Box::new(right)),
        }
    }

    pub fn less_equal(left: HirExpr, right: HirExpr, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::LessEqual(Box::new(left), Box::new(right)),
        }
    }

    pub fn greater(left: HirExpr, right: HirExpr, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::Greater(Box::new(left), Box::new(right)),
        }
    }

    pub fn greater_equal(left: HirExpr, right: HirExpr, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::GreaterEqual(Box::new(left), Box::new(right)),
        }
    }

    pub fn and(left: HirExpr, right: HirExpr, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::And(Box::new(left), Box::new(right)),
        }
    }

    pub fn or(left: HirExpr, right: HirExpr, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::Or(Box::new(left), Box::new(right)),
        }
    }

    pub fn negate(expr: HirExpr, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::Negate(Box::new(expr)),
        }
    }

    pub fn not(expr: HirExpr, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::Not(Box::new(expr)),
        }
    }

    pub fn assign(left: HirExpr, right: HirExpr, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::Assign(Box::new(left), Box::new(right)),
        }
    }

    pub fn identifier(name: String, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::Identifier(name),
        }
    }

    pub fn function_call(callee: HirExpr, arguments: Vec<HirExpr>, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::FunctionCall {
                callee: Box::new(callee),
                arguments,
            },
        }
    }

    pub fn string_literal(value: String, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::StringLiteral(value),
        }
    }

    pub fn number_literal(value: f64, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::NumberLiteral(value),
        }
    }

    pub fn boolean_literal(value: bool, span: Span) -> HirExpr {
        HirExpr {
            id: NodeId::default(),
            span,
            kind: HirExprKind::BooleanLiteral(value),
        }
    }
}
