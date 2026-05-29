use crate::{
    syntax::{
        ops::{BinaryOp, UnaryOp},
        token::Span,
    },
    util::string_interner::Symbol,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExprId(u32);

#[derive(Clone, Copy, Debug)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }
}

#[derive(Default, Debug)]
pub struct Ast {
    nodes: Vec<Spanned<Expr>>,
}

#[derive(Debug)]
pub enum Expr {
    Binary {
        operator: BinaryOp,
        left: ExprId,
        right: ExprId,
    },
    LogicalAnd {
        left: ExprId,
        right: ExprId,
    },
    LogicalOr {
        left: ExprId,
        right: ExprId,
    },
    LogicalNot(ExprId),
    Unary {
        operator: UnaryOp,
        operand: ExprId,
    },
    Assign {
        left: ExprId,
        right: ExprId,
    },
    Variable {
        left: Spanned<Symbol>,
        right: ExprId,
    },
    Ref {
        left: Spanned<Symbol>,
        right: ExprId,
    },
    Identifier(Spanned<Symbol>),
    String(Symbol),
    Number(f64),
    Boolean(bool),
    Nil,
    FunctionCall {
        callee: ExprId,
        arguments: Vec<ExprId>,
    },
    MemberAccess {
        object: ExprId,
        property: Spanned<Symbol>,
    },
    Map {
        entries: Vec<(ExprId, ExprId)>,
    },
    Function {
        name: Option<Spanned<Symbol>>,
        parameters: Vec<Spanned<Symbol>>,
        block: ExprId,
    },
    Block {
        expressions: Vec<ExprId>,
        tail: Option<ExprId>,
    },
    If {
        condition: ExprId,
        then_branch: ExprId,
        else_branch: ExprId,
    },
    WhileLoop {
        condition: ExprId,
        block: ExprId,
    },
    Return(ExprId),
    Break,
    Continue,
    Import {
        path: Vec<Spanned<Symbol>>,
    },
}

impl Ast {
    fn insert(&mut self, node: Expr, span: Span) -> ExprId {
        let id = ExprId(self.nodes.len() as u32);

        self.nodes.push(Spanned::new(node, span));

        id
    }

    pub fn last(&self) -> ExprId {
        ExprId((self.nodes.len() - 1) as u32)
    }

    pub fn node(&self, id: ExprId) -> &Expr {
        &self.nodes[id.0 as usize].value
    }

    pub fn span(&self, id: ExprId) -> Span {
        self.nodes[id.0 as usize].span
    }

    pub fn binary(
        &mut self,
        operator: BinaryOp,
        left: ExprId,
        right: ExprId,
        span: Span,
    ) -> ExprId {
        self.insert(
            Expr::Binary {
                operator,
                left,
                right,
            },
            span,
        )
    }

    pub fn logical_and(&mut self, left: ExprId, right: ExprId, span: Span) -> ExprId {
        self.insert(Expr::LogicalAnd { left, right }, span)
    }

    pub fn logical_or(&mut self, left: ExprId, right: ExprId, span: Span) -> ExprId {
        self.insert(Expr::LogicalOr { left, right }, span)
    }

    pub fn logical_not(&mut self, expression: ExprId, span: Span) -> ExprId {
        self.insert(Expr::LogicalNot(expression), span)
    }

    pub fn unary(&mut self, operator: UnaryOp, operand: ExprId, span: Span) -> ExprId {
        self.insert(Expr::Unary { operator, operand }, span)
    }

    pub fn assign(&mut self, left: ExprId, right: ExprId, span: Span) -> ExprId {
        self.insert(Expr::Assign { left, right }, span)
    }

    pub fn variable(&mut self, left: Spanned<Symbol>, right: ExprId, span: Span) -> ExprId {
        self.insert(Expr::Variable { left, right }, span)
    }

    pub fn ref_(&mut self, left: Spanned<Symbol>, right: ExprId, span: Span) -> ExprId {
        self.insert(Expr::Ref { left, right }, span)
    }

    pub fn identifier(&mut self, name: Spanned<Symbol>) -> ExprId {
        self.insert(Expr::Identifier(name), name.span)
    }

    pub fn string(&mut self, index: Symbol, span: Span) -> ExprId {
        self.insert(Expr::String(index), span)
    }

    pub fn number(&mut self, value: f64, span: Span) -> ExprId {
        self.insert(Expr::Number(value), span)
    }

    pub fn boolean(&mut self, value: bool, span: Span) -> ExprId {
        self.insert(Expr::Boolean(value), span)
    }

    pub fn nil(&mut self, span: Span) -> ExprId {
        self.insert(Expr::Nil, span)
    }

    pub fn function_call(&mut self, callee: ExprId, arguments: Vec<ExprId>, span: Span) -> ExprId {
        self.insert(Expr::FunctionCall { callee, arguments }, span)
    }

    pub fn member_access(
        &mut self,
        object: ExprId,
        property: Spanned<Symbol>,
        span: Span,
    ) -> ExprId {
        self.insert(Expr::MemberAccess { object, property }, span)
    }

    pub fn map(&mut self, entries: Vec<(ExprId, ExprId)>, span: Span) -> ExprId {
        self.insert(Expr::Map { entries }, span)
    }

    pub fn function(
        &mut self,
        name: Option<Spanned<Symbol>>,
        parameters: Vec<Spanned<Symbol>>,
        block: ExprId,
        span: Span,
    ) -> ExprId {
        self.insert(
            Expr::Function {
                name,
                parameters,
                block,
            },
            span,
        )
    }

    pub fn block(&mut self, expressions: Vec<ExprId>, tail: Option<ExprId>, span: Span) -> ExprId {
        self.insert(Expr::Block { expressions, tail }, span)
    }

    pub fn if_(
        &mut self,
        condition: ExprId,
        then_branch: ExprId,
        else_branch: ExprId,
        span: Span,
    ) -> ExprId {
        self.insert(
            Expr::If {
                condition,
                then_branch,
                else_branch,
            },
            span,
        )
    }

    pub fn while_loop(&mut self, condition: ExprId, block: ExprId, span: Span) -> ExprId {
        self.insert(Expr::WhileLoop { condition, block }, span)
    }

    pub fn return_(&mut self, expression: ExprId, span: Span) -> ExprId {
        self.insert(Expr::Return(expression), span)
    }

    pub fn break_(&mut self, span: Span) -> ExprId {
        self.insert(Expr::Break, span)
    }

    pub fn continue_(&mut self, span: Span) -> ExprId {
        self.insert(Expr::Continue, span)
    }

    pub fn import(&mut self, path: Vec<Spanned<Symbol>>, span: Span) -> ExprId {
        self.insert(Expr::Import { path }, span)
    }
}
