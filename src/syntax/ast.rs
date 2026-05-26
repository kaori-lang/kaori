use crate::{
    syntax::{
        ops::{BinaryOp, CompoundOp, UnaryOp},
        token::Span,
    },
    util::string_interner::Symbol,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExprId(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StmtId(u32);

#[derive(Clone, Copy, Debug)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(node: T, span: Span) -> Self {
        Self { node, span }
    }
}

#[derive(Default)]
pub struct Ast {
    expressions: Vec<Spanned<Expr>>,
    statements: Vec<Spanned<Stmt>>,
}

#[derive(Debug)]
pub enum Expr {
    Assign {
        left: ExprId,
        right: ExprId,
    },
    CompoundAssign {
        operator: CompoundOp,
        left: ExprId,
        right: ExprId,
    },
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
    Identifier(Spanned<Symbol>),
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
        property: Spanned<Symbol>,
    },
    DictLiteral {
        fields: Vec<(ExprId, ExprId)>,
    },
    Function {
        name: Option<Spanned<Symbol>>,
        parameters: Vec<Spanned<Symbol>>,
        block: StmtId,
    },
    Block {
        statements: Vec<StmtId>,
        tail: Option<ExprId>,
    },
    If {
        condition: ExprId,
        then_branch: ExprId,
        else_branch: Option<ExprId>,
    },
}

#[derive(Debug)]
pub enum Stmt {
    Variable {
        left: Spanned<Symbol>,
        right: ExprId,
    },
    Mut {
        left: Spanned<Symbol>,
        right: ExprId,
    },
    NativeFunction {
        name: Spanned<Symbol>,
        parameters: Vec<Spanned<Symbol>>,
    },
    WhileLoop {
        condition: ExprId,
        block: StmtId,
    },
    ForLoop {
        start: ExprId,
        end: ExprId,
        block: StmtId,
    },
    Block {
        statements: Vec<StmtId>,
    },
    Expr(ExprId),
    Return(ExprId),
    Break,
    Continue,
}

impl Ast {
    fn insert_expr(&mut self, expr: Expr, span: Span) -> ExprId {
        let id = ExprId(self.expressions.len() as u32);

        self.expressions.push(Spanned::new(expr, span));

        id
    }

    fn insert_stmt(&mut self, stmt: Stmt, span: Span) -> StmtId {
        let id = StmtId(self.statements.len() as u32);

        self.statements.push(Spanned::new(stmt, span));

        id
    }

    pub fn last_expr(&self) -> ExprId {
        ExprId((self.expressions.len() - 1) as u32)
    }

    pub fn last_stmt(&self) -> StmtId {
        StmtId((self.statements.len() - 1) as u32)
    }

    pub fn expr(&self, id: ExprId) -> &Spanned<Expr> {
        &self.expressions[id.0 as usize]
    }

    pub fn stmt(&self, id: StmtId) -> &Spanned<Stmt> {
        &self.statements[id.0 as usize]
    }

    pub fn binary(
        &mut self,
        operator: BinaryOp,
        left: ExprId,
        right: ExprId,
        span: Span,
    ) -> ExprId {
        self.insert_expr(
            Expr::Binary {
                operator,
                left,
                right,
            },
            span,
        )
    }

    pub fn logical_and(&mut self, left: ExprId, right: ExprId, span: Span) -> ExprId {
        self.insert_expr(Expr::LogicalAnd { left, right }, span)
    }

    pub fn logical_or(&mut self, left: ExprId, right: ExprId, span: Span) -> ExprId {
        self.insert_expr(Expr::LogicalOr { left, right }, span)
    }

    pub fn logical_not(&mut self, expression: ExprId, span: Span) -> ExprId {
        self.insert_expr(Expr::LogicalNot(expression), span)
    }

    pub fn unary(&mut self, operator: UnaryOp, right: ExprId, span: Span) -> ExprId {
        self.insert_expr(Expr::Unary { operator, right }, span)
    }

    pub fn identifier(&mut self, name: Spanned<Symbol>) -> ExprId {
        self.insert_expr(Expr::Identifier(name), name.span)
    }

    pub fn string_literal(&mut self, index: Symbol, span: Span) -> ExprId {
        self.insert_expr(Expr::StringLiteral(index), span)
    }

    pub fn number_literal(&mut self, value: f64, span: Span) -> ExprId {
        self.insert_expr(Expr::NumberLiteral(value), span)
    }

    pub fn boolean_literal(&mut self, value: bool, span: Span) -> ExprId {
        self.insert_expr(Expr::BooleanLiteral(value), span)
    }

    pub fn nil_literal(&mut self, span: Span) -> ExprId {
        self.insert_expr(Expr::NilLiteral, span)
    }

    pub fn function_call(&mut self, callee: ExprId, arguments: Vec<ExprId>, span: Span) -> ExprId {
        self.insert_expr(Expr::FunctionCall { callee, arguments }, span)
    }

    pub fn member_access(
        &mut self,
        object: ExprId,
        property: Spanned<Symbol>,
        span: Span,
    ) -> ExprId {
        self.insert_expr(Expr::MemberAccess { object, property }, span)
    }

    pub fn dict_literal(&mut self, fields: Vec<(ExprId, ExprId)>, span: Span) -> ExprId {
        self.insert_expr(Expr::DictLiteral { fields }, span)
    }

    pub fn function(
        &mut self,
        name: Option<Spanned<Symbol>>,
        parameters: Vec<Spanned<Symbol>>,
        block: StmtId,
        span: Span,
    ) -> ExprId {
        self.insert_expr(
            Expr::Function {
                name,
                parameters,
                block,
            },
            span,
        )
    }

    pub fn block(&mut self, statements: Vec<StmtId>, tail: Option<ExprId>, span: Span) -> ExprId {
        self.insert_expr(Expr::Block { statements, tail }, span)
    }

    pub fn if_(
        &mut self,
        condition: ExprId,
        then_branch: ExprId,
        else_branch: Option<ExprId>,
        span: Span,
    ) -> ExprId {
        self.insert_expr(
            Expr::If {
                condition,
                then_branch,
                else_branch,
            },
            span,
        )
    }

    pub fn assign(&mut self, left: ExprId, right: ExprId, span: Span) -> ExprId {
        self.insert_expr(Expr::Assign { left, right }, span)
    }

    pub fn compound_assign(
        &mut self,
        operator: CompoundOp,
        left: ExprId,
        right: ExprId,
        span: Span,
    ) -> ExprId {
        self.insert_expr(
            Expr::CompoundAssign {
                operator,
                left,
                right,
            },
            span,
        )
    }

    pub fn variable(&mut self, left: Spanned<Symbol>, right: ExprId, span: Span) -> StmtId {
        self.insert_stmt(Stmt::Variable { left, right }, span)
    }

    pub fn mut_(&mut self, left: Spanned<Symbol>, right: ExprId, span: Span) -> StmtId {
        self.insert_stmt(Stmt::Mut { left, right }, span)
    }

    pub fn native_function(
        &mut self,
        name: Spanned<Symbol>,
        parameters: Vec<Spanned<Symbol>>,
        span: Span,
    ) -> StmtId {
        self.insert_stmt(Stmt::NativeFunction { name, parameters }, span)
    }

    pub fn while_loop(&mut self, condition: ExprId, block: StmtId, span: Span) -> StmtId {
        self.insert_stmt(Stmt::WhileLoop { condition, block }, span)
    }

    pub fn for_loop(&mut self, start: ExprId, end: ExprId, block: StmtId, span: Span) -> StmtId {
        self.insert_stmt(Stmt::ForLoop { start, end, block }, span)
    }

    pub fn stmt_block(&mut self, statements: Vec<StmtId>, span: Span) -> StmtId {
        self.insert_stmt(Stmt::Block { statements }, span)
    }

    pub fn stmt_expr(&mut self, expr: ExprId, span: Span) -> StmtId {
        self.insert_stmt(Stmt::Expr(expr), span)
    }

    pub fn return_(&mut self, expression: ExprId, span: Span) -> StmtId {
        self.insert_stmt(Stmt::Return(expression), span)
    }

    pub fn break_(&mut self, span: Span) -> StmtId {
        self.insert_stmt(Stmt::Break, span)
    }

    pub fn continue_(&mut self, span: Span) -> StmtId {
        self.insert_stmt(Stmt::Continue, span)
    }
}
