use crate::{
    syntax::{
        ops::{AssignOp, BinaryOp, UnaryOp},
        token::Span,
    },
    util::string_interner::Symbol,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExprId(u32);

#[derive(Default)]
pub struct Ast {
    expressions: Vec<Expr>,
    spans: Vec<Option<Span>>,
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
        right: ExprId,
    },
    Assign {
        left: ExprId,
        right: ExprId,
    },
    CompoundAssign {
        operator: AssignOp,
        left: ExprId,
        right: ExprId,
    },
    Variable {
        left: ExprId,
        right: ExprId,
    },
    Mut {
        left: ExprId,
        right: ExprId,
    },
    Identifier(Symbol),
    StringLiteral(Symbol),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    NilLiteral,
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
        else_branch: ExprId,
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
    Return(ExprId),
    Break,
    Continue,
}

impl Expr {
    pub fn as_identifier(&self) -> Symbol {
        match self {
            Self::Identifier(name) => *name,
            _ => unreachable!("must be an identifier"),
        }
    }
}

impl Ast {
    fn insert(&mut self, expr: Expr, span: Option<Span>) -> ExprId {
        let id = ExprId(self.expressions.len() as u32);

        self.expressions.push(expr);
        self.spans.push(span);

        id
    }

    pub fn entry(&self) -> ExprId {
        let last = self.expressions.len() - 1;

        ExprId(last as u32)
    }

    pub fn node(&self, id: ExprId) -> &Expr {
        &self.expressions[id.0 as usize]
    }

    pub fn span(&self, id: ExprId) -> Option<Span> {
        self.spans[id.0 as usize]
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
            Some(span),
        )
    }

    pub fn logical_and(&mut self, left: ExprId, right: ExprId, span: Span) -> ExprId {
        self.insert(Expr::LogicalAnd { left, right }, Some(span))
    }

    pub fn logical_or(&mut self, left: ExprId, right: ExprId, span: Span) -> ExprId {
        self.insert(Expr::LogicalOr { left, right }, Some(span))
    }

    pub fn logical_not(&mut self, expression: ExprId, span: Span) -> ExprId {
        self.insert(Expr::LogicalNot(expression), Some(span))
    }

    pub fn unary(&mut self, operator: UnaryOp, right: ExprId, span: Span) -> ExprId {
        self.insert(Expr::Unary { operator, right }, Some(span))
    }

    pub fn assign(&mut self, left: ExprId, right: ExprId, span: Span) -> ExprId {
        self.insert(Expr::Assign { left, right }, Some(span))
    }

    pub fn compound_assign(
        &mut self,
        operator: AssignOp,
        left: ExprId,
        right: ExprId,
        span: Span,
    ) -> ExprId {
        self.insert(
            Expr::CompoundAssign {
                operator,
                left,
                right,
            },
            Some(span),
        )
    }

    pub fn variable(&mut self, left: ExprId, right: ExprId) -> ExprId {
        self.insert(Expr::Variable { left, right }, None)
    }

    pub fn mut_(&mut self, left: ExprId, right: ExprId) -> ExprId {
        self.insert(Expr::Mut { left, right }, None)
    }

    pub fn identifier(&mut self, index: Symbol, span: Span) -> ExprId {
        self.insert(Expr::Identifier(index), Some(span))
    }

    pub fn string_literal(&mut self, index: Symbol, span: Span) -> ExprId {
        self.insert(Expr::StringLiteral(index), Some(span))
    }

    pub fn number_literal(&mut self, value: f64, span: Span) -> ExprId {
        self.insert(Expr::NumberLiteral(value), Some(span))
    }

    pub fn boolean_literal(&mut self, value: bool, span: Span) -> ExprId {
        self.insert(Expr::BooleanLiteral(value), Some(span))
    }

    pub fn nil_literal(&mut self, span: Span) -> ExprId {
        self.insert(Expr::NilLiteral, Some(span))
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

    pub fn if_(&mut self, condition: ExprId, then_branch: ExprId, else_branch: ExprId) -> ExprId {
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

    pub fn return_(&mut self, expression: ExprId, span: Span) -> ExprId {
        self.insert(Expr::Return(expression), Some(span))
    }

    pub fn break_(&mut self, span: Span) -> ExprId {
        self.insert(Expr::Break, Some(span))
    }

    pub fn continue_(&mut self, span: Span) -> ExprId {
        self.insert(Expr::Continue, Some(span))
    }
}
