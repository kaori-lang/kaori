use crate::{
    ast::ops::{BinaryOp, UnaryOp},
    lexer::span::Span,
};

use super::node_id::NodeId;

#[derive(Debug, Clone)]
pub struct Expr {
    pub id: NodeId,
    pub span: Span,
    pub kind: ExprKind,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Binary {
        operator: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    LogicalAnd {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    LogicalOr {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    LogicalNot(Box<Expr>),
    Unary {
        operator: UnaryOp,
        right: Box<Expr>,
    },
    Assign {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    DeclareAssign {
        id: NodeId,
        right: Box<Expr>,
    },
    VariableRef(NodeId),
    FunctionRef(NodeId),
    String(String),
    Number(f64),
    DictLiteral {
        fields: Vec<(Expr, Expr)>,
    },
    FunctionCall {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
    MemberAccess {
        object: Box<Expr>,
        property: Box<Expr>,
    },
    Block(Vec<Expr>),
    UncheckedBlock(Vec<Expr>),
    Function {
        parameters: Vec<(NodeId, Span)>,
        body: Vec<Expr>,
    },
    Branch {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },
    Loop {
        init: Option<Box<Expr>>,
        condition: Box<Expr>,
        block: Box<Expr>,
        increment: Option<Box<Expr>>,
    },
    Return(Option<Box<Expr>>),
    Break,
    Continue,
    Print(Box<Expr>),
}

impl Expr {
    pub fn binary(operator: BinaryOp, left: Expr, right: Expr, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn logical_and(left: Expr, right: Expr, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::LogicalAnd {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn logical_or(left: Expr, right: Expr, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::LogicalOr {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn logical_not(expression: Expr, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::LogicalNot(Box::new(expression)),
        }
    }

    pub fn unary(operator: UnaryOp, right: Expr, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::Unary {
                operator,
                right: Box::new(right),
            },
        }
    }

    pub fn assign(left: Expr, right: Expr, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::Assign {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn declare_assign(id: NodeId, right: Expr, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::DeclareAssign {
                id,
                right: Box::new(right),
            },
        }
    }

    pub fn variable_ref(id: NodeId, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::VariableRef(id),
        }
    }

    pub fn function_ref(id: NodeId, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::FunctionRef(id),
        }
    }

    pub fn string(value: String, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::String(value),
        }
    }

    pub fn number(value: f64, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::Number(value),
        }
    }

    pub fn dict_literal(fields: Vec<(Expr, Expr)>, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::DictLiteral { fields },
        }
    }

    pub fn function_call(callee: Expr, arguments: Vec<Expr>, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::FunctionCall {
                callee: Box::new(callee),
                arguments,
            },
        }
    }

    pub fn member_access(object: Expr, property: Expr, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::MemberAccess {
                object: Box::new(object),
                property: Box::new(property),
            },
        }
    }

    pub fn block(body: Vec<Expr>, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::Block(body),
        }
    }

    pub fn unchecked_block(body: Vec<Expr>, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::UncheckedBlock(body),
        }
    }

    pub fn function(
        id: NodeId,
        parameters: Vec<(NodeId, Span)>,
        body: Vec<Expr>,
        span: Span,
    ) -> Expr {
        Expr {
            id,
            span,
            kind: ExprKind::Function { parameters, body },
        }
    }

    pub fn branch(
        condition: Expr,
        then_branch: Expr,
        else_branch: Option<Expr>,
        span: Span,
    ) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::Branch {
                condition: Box::new(condition),
                then_branch: Box::new(then_branch),
                else_branch: else_branch.map(Box::new),
            },
        }
    }

    pub fn loop_(
        init: Option<Expr>,
        condition: Expr,
        block: Expr,
        increment: Option<Expr>,
        span: Span,
    ) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::Loop {
                init: init.map(Box::new),
                condition: Box::new(condition),
                block: Box::new(block),
                increment: increment.map(Box::new),
            },
        }
    }

    pub fn return_(expr: Option<Expr>, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::Return(expr.map(Box::new)),
        }
    }

    pub fn break_(span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::Break,
        }
    }

    pub fn continue_(span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::Continue,
        }
    }

    pub fn print(expr: Expr, span: Span) -> Expr {
        Expr {
            id: NodeId::default(),
            span,
            kind: ExprKind::Print(Box::new(expr)),
        }
    }
}
