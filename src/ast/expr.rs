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
        name: String,
        parameters: Vec<(String, Span)>,
        body: Vec<Expr>,
    },
    Block {
        expressions: Vec<Expr>,
        tail: Box<Expr>,
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
        init: Box<Expr>,
        condition: Box<Expr>,
        increment: Box<Expr>,
        block: Box<Expr>,
    },
    UncheckedBlock {
        expressions: Vec<Expr>,
        tail: Box<Expr>,
    },
    Return(Option<Box<Expr>>),
    Break,
    Continue,
    Print(Box<Expr>),
}

impl Expr {
    pub fn binary(operator: BinaryOp, left: Expr, right: Expr) -> Expr {
        let span = Span::merge(left.span, right.span);
        Expr {
            span,
            kind: ExprKind::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn logical_and(left: Expr, right: Expr) -> Expr {
        let span = Span::merge(left.span, right.span);
        Expr {
            span,
            kind: ExprKind::LogicalAnd {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn logical_or(left: Expr, right: Expr) -> Expr {
        let span = Span::merge(left.span, right.span);
        Expr {
            span,
            kind: ExprKind::LogicalOr {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn logical_not(expression: Expr) -> Expr {
        Expr {
            span: expression.span,
            kind: ExprKind::LogicalNot(Box::new(expression)),
        }
    }

    pub fn unary(operator: UnaryOp, right: Expr) -> Expr {
        let span = right.span;
        Expr {
            span,
            kind: ExprKind::Unary {
                operator,
                right: Box::new(right),
            },
        }
    }

    pub fn assign(operator: AssignOp, left: Expr, right: Expr) -> Expr {
        let span = Span::merge(left.span, right.span);
        Expr {
            span,
            kind: ExprKind::Assign {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn declare_assign(left: Expr, right: Expr) -> Expr {
        let span = Span::merge(left.span, right.span);
        Expr {
            span,
            kind: ExprKind::DeclareAssign {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub fn identifier(name: String, span: Span) -> Expr {
        Expr {
            span,
            kind: ExprKind::Identifier(name),
        }
    }

    pub fn function_call(callee: Expr, arguments: Vec<Expr>, span: Span) -> Expr {
        let span = Span::merge(callee.span, span);
        Expr {
            span,
            kind: ExprKind::FunctionCall {
                callee: Box::new(callee),
                arguments,
            },
        }
    }

    pub fn member_access(object: Expr, property: Expr) -> Expr {
        let span = Span::merge(object.span, property.span);
        Expr {
            span,
            kind: ExprKind::MemberAccess {
                object: Box::new(object),
                property: Box::new(property),
            },
        }
    }

    pub fn string_literal(value: String, span: Span) -> Expr {
        Expr {
            span,
            kind: ExprKind::StringLiteral(value),
        }
    }

    pub fn number_literal(value: f64, span: Span) -> Expr {
        Expr {
            span,
            kind: ExprKind::NumberLiteral(value),
        }
    }

    pub fn boolean_literal(value: bool, span: Span) -> Expr {
        Expr {
            span,
            kind: ExprKind::BooleanLiteral(value),
        }
    }

    pub fn dict_literal(fields: Vec<(Expr, Option<Expr>)>, span: Span) -> Expr {
        Expr {
            span,
            kind: ExprKind::DictLiteral { fields },
        }
    }

    pub fn function(
        name: String,
        parameters: Vec<(String, Span)>,
        body: Vec<Expr>,
        span: Span,
    ) -> Expr {
        Expr {
            span,
            kind: ExprKind::Function {
                name,
                parameters,
                body,
            },
        }
    }

    pub fn block(expressions: Vec<Expr>, tail: Expr, span: Span) -> Expr {
        Expr {
            span,
            kind: ExprKind::Block {
                expressions,
                tail: Box::new(tail),
            },
        }
    }

    pub fn if_(condition: Expr, then_branch: Expr, else_branch: Option<Expr>, span: Span) -> Expr {
        Expr {
            span,
            kind: ExprKind::If {
                condition: Box::new(condition),
                then_branch: Box::new(then_branch),
                else_branch: else_branch.map(Box::new),
            },
        }
    }

    pub fn while_loop(condition: Expr, block: Expr, span: Span) -> Expr {
        Expr {
            span,
            kind: ExprKind::WhileLoop {
                condition: Box::new(condition),
                block: Box::new(block),
            },
        }
    }

    pub fn for_loop(init: Expr, condition: Expr, increment: Expr, block: Expr, span: Span) -> Expr {
        Expr {
            span,
            kind: ExprKind::ForLoop {
                init: Box::new(init),
                condition: Box::new(condition),
                increment: Box::new(increment),
                block: Box::new(block),
            },
        }
    }

    pub fn unchecked_block(expressions: Vec<Expr>, tail: Expr, span: Span) -> Expr {
        Expr {
            span,
            kind: ExprKind::UncheckedBlock {
                expressions,
                tail: Box::new(tail),
            },
        }
    }

    pub fn return_(expression: Option<Expr>, span: Span) -> Expr {
        Expr {
            span,
            kind: ExprKind::Return(expression.map(Box::new)),
        }
    }

    pub fn break_(span: Span) -> Expr {
        Expr {
            span,
            kind: ExprKind::Break,
        }
    }

    pub fn continue_(span: Span) -> Expr {
        Expr {
            span,
            kind: ExprKind::Continue,
        }
    }

    pub fn print(expression: Expr, span: Span) -> Expr {
        Expr {
            span,
            kind: ExprKind::Print(Box::new(expression)),
        }
    }
}
