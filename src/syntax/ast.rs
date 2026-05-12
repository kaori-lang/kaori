use crate::{
    syntax::ops::{AssignOp, BinaryOp, UnaryOp},
    util::string_interner::StringIndex,
};

use std::collections::HashMap;
use std::ops::Range;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ExprId(u32);

#[derive(Default)]
pub struct Ast {
    expressions: Vec<Expr>,
    spans: HashMap<ExprId, Range<usize>>,
}

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
    Identifier(StringIndex),
    StringLiteral(StringIndex),
    NumberLiteral(f64),
    FunctionCall {
        callee: ExprId,
        arguments: Box<[ExprId]>,
    },
    MemberAccess {
        object: ExprId,
        property: ExprId,
    },
    DictLiteral {
        fields: Box<[(ExprId, Option<ExprId>)]>,
    },
    NativeFunction {
        name: ExprId,
        parameters: Box<[ExprId]>,
    },
    Function {
        name: Option<ExprId>,
        parameters: Box<[ExprId]>,
        block: ExprId,
    },
    Block(Box<[ExprId]>),
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
    Return(Option<ExprId>),
    Break,
    Continue,
}

impl Ast {
    fn insert(&mut self, expr: Expr, span: Option<Range<usize>>) -> ExprId {
        let id = ExprId(self.expressions.len() as u32);

        self.expressions.push(expr);

        if let Some(span) = span {
            self.spans.insert(id, span);
        }

        id
    }

    pub fn entry(&self) -> ExprId {
        let last = self.expressions.len() - 1;

        ExprId(last as u32)
    }

    pub fn get(&self, id: ExprId) -> &Expr {
        &self.expressions[id.0 as usize]
    }

    pub fn span(&self, id: ExprId) -> Option<&Range<usize>> {
        self.spans.get(&id)
    }

    pub fn binary(
        &mut self,
        operator: BinaryOp,
        left: ExprId,
        right: ExprId,
        span: Range<usize>,
    ) -> ExprId {
        self.insert(
            Expr::Binary {
                operator,
                left,
                right,
            },
            Some(span),
        )
    }

    pub fn logical_and(&mut self, left: ExprId, right: ExprId, span: Range<usize>) -> ExprId {
        self.insert(Expr::LogicalAnd { left, right }, Some(span))
    }

    pub fn logical_or(&mut self, left: ExprId, right: ExprId, span: Range<usize>) -> ExprId {
        self.insert(Expr::LogicalOr { left, right }, Some(span))
    }

    pub fn logical_not(&mut self, expression: ExprId, span: Range<usize>) -> ExprId {
        self.insert(Expr::LogicalNot(expression), Some(span))
    }

    pub fn unary(&mut self, operator: UnaryOp, right: ExprId, span: Range<usize>) -> ExprId {
        self.insert(Expr::Unary { operator, right }, Some(span))
    }

    pub fn assign(
        &mut self,
        operator: AssignOp,
        left: ExprId,
        right: ExprId,
        span: Range<usize>,
    ) -> ExprId {
        self.insert(
            Expr::Assign {
                operator,
                left,
                right,
            },
            Some(span),
        )
    }

    pub fn declare_assign(&mut self, left: ExprId, right: ExprId, span: Range<usize>) -> ExprId {
        self.insert(Expr::DeclareAssign { left, right }, Some(span))
    }

    pub fn identifier(&mut self, index: StringIndex, span: Range<usize>) -> ExprId {
        self.insert(Expr::Identifier(index), Some(span))
    }

    pub fn string_literal(&mut self, index: StringIndex, span: Range<usize>) -> ExprId {
        self.insert(Expr::StringLiteral(index), Some(span))
    }

    pub fn number_literal(&mut self, value: f64, span: Range<usize>) -> ExprId {
        self.insert(Expr::NumberLiteral(value), Some(span))
    }

    pub fn function_call(&mut self, callee: ExprId, arguments: Vec<ExprId>) -> ExprId {
        self.insert(
            Expr::FunctionCall {
                callee,
                arguments: arguments.into(),
            },
            None,
        )
    }

    pub fn member_access(&mut self, object: ExprId, property: ExprId) -> ExprId {
        self.insert(Expr::MemberAccess { object, property }, None)
    }

    pub fn dict_literal(&mut self, fields: Vec<(ExprId, Option<ExprId>)>) -> ExprId {
        self.insert(
            Expr::DictLiteral {
                fields: fields.into(),
            },
            None,
        )
    }

    pub fn native_function(&mut self, name: ExprId, parameters: Vec<ExprId>) -> ExprId {
        self.insert(
            Expr::NativeFunction {
                name,
                parameters: parameters.into(),
            },
            None,
        )
    }

    pub fn function(
        &mut self,
        name: Option<ExprId>,
        parameters: Vec<ExprId>,
        block: ExprId,
    ) -> ExprId {
        self.insert(
            Expr::Function {
                name,
                parameters: parameters.into(),
                block,
            },
            None,
        )
    }

    pub fn block(&mut self, expressions: Vec<ExprId>) -> ExprId {
        self.insert(Expr::Block(expressions.into()), None)
    }

    pub fn if_(
        &mut self,
        condition: ExprId,
        then_branch: ExprId,
        else_branch: Option<ExprId>,
    ) -> ExprId {
        self.insert(
            Expr::If {
                condition,
                then_branch,
                else_branch,
            },
            None,
        )
    }

    pub fn while_loop(&mut self, condition: ExprId, block: ExprId) -> ExprId {
        self.insert(Expr::WhileLoop { condition, block }, None)
    }

    pub fn for_loop(&mut self, start: ExprId, end: ExprId, block: ExprId) -> ExprId {
        self.insert(Expr::ForLoop { start, end, block }, None)
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
}
