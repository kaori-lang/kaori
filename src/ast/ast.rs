use std::collections::HashMap;
use std::ops::Range;

use crate::ast::ops::{AssignOp, BinaryOp, UnaryOp};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ExprId(u32);

pub struct Ast {
    pub top_level: Vec<ExprId>,
    expressions: Vec<Expr>,
    spans: HashMap<ExprId, Range<usize>>,
}

#[derive(Debug, Clone)]
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
        right: ExprId,
    },
    Assign {
        operator: AssignOp,
        left: ExprId,
        right: ExprId,
    },
    DeclareAssign {
        left: ExprId,
        right: ExprId,
    },
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    FunctionCall {
        callee: ExprId,
        arguments: Vec<ExprId>,
    },
    MemberAccess {
        object: ExprId,
        property: ExprId,
    },
    DictLiteral {
        fields: Vec<(ExprId, Option<ExprId>)>,
    },
    Function {
        name: Option<ExprId>,
        parameters: Vec<ExprId>,
        captures: Vec<ExprId>,
        body: Vec<ExprId>,
    },
    Block {
        expressions: Vec<ExprId>,
        tail: Option<ExprId>,
    },
    If {
        condition: ExprId,
        then_branch: ExprId,
        else_branch: Option<ExprId>,
    },
    WhileLoop {
        condition: ExprId,
        block: ExprId,
    },
    ForLoop {
        start: ExprId,
        end: ExprId,
        block: ExprId,
    },
    UncheckedBlock {
        expressions: Vec<ExprId>,
        tail: Option<ExprId>,
    },
    Return(Option<ExprId>),
    Break,
    Continue,
    Print(ExprId),
}

impl Ast {
    pub fn new() -> Self {
        Self {
            top_level: Vec::new(),
            expressions: Vec::new(),
            spans: HashMap::new(),
        }
    }

    fn insert(&mut self, expr: Expr, span: Option<Range<usize>>) -> ExprId {
        let id = ExprId(self.expressions.len() as u32);

        self.expressions.push(expr);

        if let Some(span) = span {
            self.spans.insert(id, span);
        }

        id
    }

    pub fn get(&self, id: ExprId) -> &Expr {
        &self.expressions[id.0 as usize]
    }

    pub fn span(&self, id: ExprId) -> Option<&Range<usize>> {
        self.spans.get(&id)
    }

    pub fn binary(&mut self, operator: BinaryOp, left: ExprId, right: ExprId) -> ExprId {
        self.insert(
            Expr::Binary {
                operator,
                left,
                right,
            },
            None,
        )
    }

    pub fn logical_and(&mut self, left: ExprId, right: ExprId) -> ExprId {
        self.insert(Expr::LogicalAnd { left, right }, None)
    }

    pub fn logical_or(&mut self, left: ExprId, right: ExprId) -> ExprId {
        self.insert(Expr::LogicalOr { left, right }, None)
    }

    pub fn logical_not(&mut self, expression: ExprId) -> ExprId {
        self.insert(Expr::LogicalNot(expression), None)
    }

    pub fn unary(&mut self, operator: UnaryOp, right: ExprId) -> ExprId {
        self.insert(Expr::Unary { operator, right }, None)
    }

    pub fn assign(&mut self, operator: AssignOp, left: ExprId, right: ExprId) -> ExprId {
        self.insert(
            Expr::Assign {
                operator,
                left,
                right,
            },
            None,
        )
    }

    pub fn declare_assign(&mut self, left: ExprId, right: ExprId) -> ExprId {
        self.insert(Expr::DeclareAssign { left, right }, None)
    }

    pub fn identifier(&mut self, name: String, span: Range<usize>) -> ExprId {
        self.insert(Expr::Identifier(name), Some(span))
    }

    pub fn string_literal(&mut self, value: String, span: Range<usize>) -> ExprId {
        self.insert(Expr::StringLiteral(value), Some(span))
    }

    pub fn number_literal(&mut self, value: f64, span: Range<usize>) -> ExprId {
        self.insert(Expr::NumberLiteral(value), Some(span))
    }

    pub fn boolean_literal(&mut self, value: bool, span: Range<usize>) -> ExprId {
        self.insert(Expr::BooleanLiteral(value), Some(span))
    }

    pub fn function_call(
        &mut self,
        callee: ExprId,
        arguments: Vec<ExprId>,
        span: Range<usize>,
    ) -> ExprId {
        self.insert(Expr::FunctionCall { callee, arguments }, Some(span))
    }

    pub fn member_access(&mut self, object: ExprId, property: ExprId) -> ExprId {
        self.insert(Expr::MemberAccess { object, property }, None)
    }

    pub fn dict_literal(
        &mut self,
        fields: Vec<(ExprId, Option<ExprId>)>,
        span: Range<usize>,
    ) -> ExprId {
        self.insert(Expr::DictLiteral { fields }, Some(span))
    }

    pub fn function(
        &mut self,
        name: Option<ExprId>,
        parameters: Vec<ExprId>,
        captures: Vec<ExprId>,
        body: Vec<ExprId>,
        span: Range<usize>,
    ) -> ExprId {
        self.insert(
            Expr::Function {
                name,
                parameters,
                captures,
                body,
            },
            Some(span),
        )
    }

    pub fn block(
        &mut self,
        expressions: Vec<ExprId>,
        tail: Option<ExprId>,
        span: Range<usize>,
    ) -> ExprId {
        self.insert(Expr::Block { expressions, tail }, Some(span))
    }

    pub fn if_(
        &mut self,
        condition: ExprId,
        then_branch: ExprId,
        else_branch: Option<ExprId>,
        span: Range<usize>,
    ) -> ExprId {
        self.insert(
            Expr::If {
                condition,
                then_branch,
                else_branch,
            },
            Some(span),
        )
    }

    pub fn while_loop(&mut self, condition: ExprId, block: ExprId, span: Range<usize>) -> ExprId {
        self.insert(Expr::WhileLoop { condition, block }, Some(span))
    }

    pub fn for_loop(
        &mut self,
        start: ExprId,
        end: ExprId,
        block: ExprId,
        span: Range<usize>,
    ) -> ExprId {
        self.insert(Expr::ForLoop { start, end, block }, Some(span))
    }

    pub fn unchecked_block(
        &mut self,
        expressions: Vec<ExprId>,
        tail: Option<ExprId>,
        span: Range<usize>,
    ) -> ExprId {
        self.insert(Expr::UncheckedBlock { expressions, tail }, Some(span))
    }

    pub fn return_(&mut self, expression: Option<ExprId>, span: Range<usize>) -> ExprId {
        self.insert(Expr::Return(expression), Some(span))
    }

    pub fn break_(&mut self, span: Range<usize>) -> ExprId {
        self.insert(Expr::Break, Some(span))
    }

    pub fn continue_(&mut self, span: Range<usize>) -> ExprId {
        self.insert(Expr::Continue, Some(span))
    }

    pub fn print(&mut self, expression: ExprId, span: Range<usize>) -> ExprId {
        self.insert(Expr::Print(expression), Some(span))
    }
}
