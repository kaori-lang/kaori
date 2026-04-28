use crate::{
    ast::ops::{AssignOp, BinaryOp, UnaryOp},
    lexer::span::Span,
};

#[derive(Debug)]
pub struct Expr {
    pub span: Span,
    pub kind: ExprKind,
}

#[derive(Debug)]
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
        operator: AssignOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    DeclareAssign {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    FunctionCall {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
    MemberAccess {
        object: Box<Expr>,
        property: Box<Expr>,
    },
    DictLiteral {
        fields: Vec<(Expr, Option<Expr>)>,
    },
    Function {
        name: Option<Box<Expr>>,
        parameters: Vec<Expr>,
        captures: Vec<Expr>,
        body: Vec<Expr>,
    },
    Block {
        expressions: Vec<Expr>,
        tail: Option<Box<Expr>>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },
    WhileLoop {
        condition: Box<Expr>,
        block: Box<Expr>,
    },
    ForLoop {
        start: Box<Expr>,
        end: Box<Expr>,
        block: Box<Expr>,
    },
    UncheckedBlock {
        expressions: Vec<Expr>,
        tail: Option<Box<Expr>>,
    },
    Return(Option<Box<Expr>>),
    Break,
    Continue,
    Print(Box<Expr>),
}

impl Expr {
    pub fn binary(operator: BinaryOp, left: Expr, right: Expr) -> Self {
        let span = Span::merge(left.span, right.span);
        Self {
            span,
            kind: ExprKind::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn logical_and(left: Expr, right: Expr) -> Self {
        let span = Span::merge(left.span, right.span);
        Self {
            span,
            kind: ExprKind::LogicalAnd {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn logical_or(left: Expr, right: Expr) -> Self {
        let span = Span::merge(left.span, right.span);
        Self {
            span,
            kind: ExprKind::LogicalOr {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn logical_not(expression: Expr) -> Self {
        Self {
            span: expression.span,
            kind: ExprKind::LogicalNot(Box::new(expression)),
        }
    }

    pub fn unary(operator: UnaryOp, right: Expr) -> Self {
        let span = right.span;
        Self {
            span,
            kind: ExprKind::Unary {
                operator,
                right: Box::new(right),
            },
        }
    }

    pub fn assign(operator: AssignOp, left: Expr, right: Expr) -> Self {
        let span = Span::merge(left.span, right.span);
        Self {
            span,
            kind: ExprKind::Assign {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn declare_assign(left: Expr, right: Expr) -> Self {
        let span = Span::merge(left.span, right.span);
        Self {
            span,
            kind: ExprKind::DeclareAssign {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn identifier(name: String, span: Span) -> Self {
        Self {
            span,
            kind: ExprKind::Identifier(name),
        }
    }

    pub fn function_call(callee: Expr, arguments: Vec<Expr>, span: Span) -> Self {
        let span = Span::merge(callee.span, span);
        Self {
            span,
            kind: ExprKind::FunctionCall {
                callee: Box::new(callee),
                arguments,
            },
        }
    }

    pub fn member_access(object: Expr, property: Expr) -> Self {
        let span = Span::merge(object.span, property.span);
        Self {
            span,
            kind: ExprKind::MemberAccess {
                object: Box::new(object),
                property: Box::new(property),
            },
        }
    }

    pub fn string_literal(value: String, span: Span) -> Self {
        Self {
            span,
            kind: ExprKind::StringLiteral(value),
        }
    }

    pub fn number_literal(value: f64, span: Span) -> Self {
        Self {
            span,
            kind: ExprKind::NumberLiteral(value),
        }
    }

    pub fn boolean_literal(value: bool, span: Span) -> Self {
        Self {
            span,
            kind: ExprKind::BooleanLiteral(value),
        }
    }

    pub fn dict_literal(fields: Vec<(Expr, Option<Expr>)>, span: Span) -> Self {
        Self {
            span,
            kind: ExprKind::DictLiteral { fields },
        }
    }

    pub fn function(
        name: Option<Expr>,
        parameters: Vec<Expr>,
        captures: Vec<Expr>,
        body: Vec<Expr>,
        span: Span,
    ) -> Self {
        Self {
            span,
            kind: ExprKind::Function {
                name: name.map(Box::new),
                parameters,
                captures,
                body,
            },
        }
    }

    pub fn block(expressions: Vec<Expr>, tail: Option<Expr>, span: Span) -> Self {
        Self {
            span,
            kind: ExprKind::Block {
                expressions,
                tail: tail.map(Box::new),
            },
        }
    }

    pub fn if_(condition: Expr, then_branch: Expr, else_branch: Option<Expr>, span: Span) -> Self {
        Self {
            span,
            kind: ExprKind::If {
                condition: Box::new(condition),
                then_branch: Box::new(then_branch),
                else_branch: else_branch.map(Box::new),
            },
        }
    }

    pub fn while_loop(condition: Expr, block: Expr, span: Span) -> Self {
        Self {
            span,
            kind: ExprKind::WhileLoop {
                condition: Box::new(condition),
                block: Box::new(block),
            },
        }
    }

    pub fn for_loop(start: Expr, end: Expr, block: Expr, span: Span) -> Self {
        Self {
            span,
            kind: ExprKind::ForLoop {
                start: Box::new(start),
                end: Box::new(end),
                block: Box::new(block),
            },
        }
    }

    pub fn unchecked_block(expressions: Vec<Expr>, tail: Option<Expr>, span: Span) -> Self {
        Self {
            span,
            kind: ExprKind::UncheckedBlock {
                expressions,
                tail: tail.map(Box::new),
            },
        }
    }

    pub fn return_(expression: Option<Expr>, span: Span) -> Self {
        Self {
            span,
            kind: ExprKind::Return(expression.map(Box::new)),
        }
    }

    pub fn break_(span: Span) -> Self {
        Self {
            span,
            kind: ExprKind::Break,
        }
    }

    pub fn continue_(span: Span) -> Self {
        Self {
            span,
            kind: ExprKind::Continue,
        }
    }

    pub fn print(expression: Expr, span: Span) -> Self {
        Self {
            span,
            kind: ExprKind::Print(Box::new(expression)),
        }
    }
}
