use crate::{
    syntax::{
        ops::{BinaryOp, CompoundOp, UnaryOp},
        token::Span,
    },
    util::string_interner::Symbol,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExprId(u32);

#[derive(Default)]
pub struct Ast {
    nodes: Vec<Expr>,
    spans: Vec<Span>,
}

#[derive(Debug, Clone, Copy)]
pub struct Name {
    pub symbol: Symbol,
    pub span: Span,
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
        operator: CompoundOp,
        left: ExprId,
        right: ExprId,
    },
    Variable {
        left: Name,
        right: ExprId,
    },
    Mut {
        left: Name,
        right: ExprId,
    },
    Identifier(Name),
    StringLiteral(Symbol),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    NilLiteral,
    FunctionCall {
        callee: ExprId,
        arguments: Vec<ExprId>,
    },
    MemberAccess {
        object: ExprId,
        property: Name,
    },
    DictLiteral {
        fields: Vec<(ExprId, ExprId)>,
    },
    NativeFunction {
        name: Name,
        parameters: Vec<Name>,
    },
    Function {
        name: Option<Name>,
        parameters: Vec<Name>,
        block: ExprId,
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
    Return(ExprId),
    Break,
    Continue,
}

impl Ast {
    fn insert(&mut self, node: Expr, span: Span) -> ExprId {
        let id = ExprId(self.nodes.len() as u32);

        self.nodes.push(node);
        self.spans.push(span);

        id
    }

    pub fn last(&self) -> ExprId {
        ExprId((self.nodes.len() - 1) as u32)
    }

    pub fn node(&self, id: ExprId) -> &Expr {
        &self.nodes[id.0 as usize]
    }

    pub fn span(&self, id: ExprId) -> Span {
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

    pub fn unary(&mut self, operator: UnaryOp, right: ExprId, span: Span) -> ExprId {
        self.insert(Expr::Unary { operator, right }, span)
    }

    pub fn assign(&mut self, left: ExprId, right: ExprId, span: Span) -> ExprId {
        self.insert(Expr::Assign { left, right }, span)
    }

    pub fn compound_assign(
        &mut self,
        operator: CompoundOp,
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
            span,
        )
    }

    pub fn variable(&mut self, left: Name, right: ExprId, span: Span) -> ExprId {
        self.insert(Expr::Variable { left, right }, span)
    }

    pub fn mut_(&mut self, left: Name, right: ExprId, span: Span) -> ExprId {
        self.insert(Expr::Mut { left, right }, span)
    }

    pub fn identifier(&mut self, name: Name) -> ExprId {
        self.insert(Expr::Identifier(name), name.span)
    }

    pub fn string_literal(&mut self, index: Symbol, span: Span) -> ExprId {
        self.insert(Expr::StringLiteral(index), span)
    }

    pub fn number_literal(&mut self, value: f64, span: Span) -> ExprId {
        self.insert(Expr::NumberLiteral(value), span)
    }

    pub fn boolean_literal(&mut self, value: bool, span: Span) -> ExprId {
        self.insert(Expr::BooleanLiteral(value), span)
    }

    pub fn nil_literal(&mut self, span: Span) -> ExprId {
        self.insert(Expr::NilLiteral, span)
    }

    pub fn function_call(&mut self, callee: ExprId, arguments: Vec<ExprId>, span: Span) -> ExprId {
        self.insert(Expr::FunctionCall { callee, arguments }, span)
    }

    pub fn member_access(&mut self, object: ExprId, property: Name, span: Span) -> ExprId {
        self.insert(Expr::MemberAccess { object, property }, span)
    }

    pub fn dict_literal(&mut self, fields: Vec<(ExprId, ExprId)>, span: Span) -> ExprId {
        self.insert(Expr::DictLiteral { fields }, span)
    }

    pub fn native_function(&mut self, name: Name, parameters: Vec<Name>, span: Span) -> ExprId {
        self.insert(Expr::NativeFunction { name, parameters }, span)
    }

    pub fn function(
        &mut self,
        name: Option<Name>,
        parameters: Vec<Name>,
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
        else_branch: Option<ExprId>,
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

    pub fn for_loop(&mut self, start: ExprId, end: ExprId, block: ExprId, span: Span) -> ExprId {
        self.insert(Expr::ForLoop { start, end, block }, span)
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
}
