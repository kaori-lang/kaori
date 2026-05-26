use crate::{
    syntax::{
        ops::{BinaryOp, CompoundOp, UnaryOp},
        token::Span,
    },
    util::string_interner::Symbol,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NodeId(u32);

#[derive(Default)]
pub struct Ast {
    expressions: Vec<AstNode>,
    spans: Vec<Span>,
}

#[derive(Debug, Clone, Copy)]
pub struct Name {
    pub symbol: Symbol,
    pub span: Span,
}

#[derive(Debug)]
pub enum AstNode {
    Binary {
        operator: BinaryOp,
        left: NodeId,
        right: NodeId,
    },
    LogicalAnd {
        left: NodeId,
        right: NodeId,
    },
    LogicalOr {
        left: NodeId,
        right: NodeId,
    },
    LogicalNot(NodeId),
    Unary {
        operator: UnaryOp,
        right: NodeId,
    },
    Assign {
        left: NodeId,
        right: NodeId,
    },
    CompoundAssign {
        operator: CompoundOp,
        left: NodeId,
        right: NodeId,
    },
    Variable {
        left: Name,
        right: NodeId,
    },
    Mut {
        left: Name,
        right: NodeId,
    },
    Identifier(Name),
    StringLiteral(Symbol),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    NilLiteral,
    FunctionCall {
        callee: NodeId,
        arguments: Vec<NodeId>,
    },
    MemberAccess {
        object: NodeId,
        property: Name,
    },
    DictLiteral {
        fields: Vec<(NodeId, NodeId)>,
    },
    NativeFunction {
        name: Name,
        parameters: Vec<Name>,
    },
    Function {
        name: Option<Name>,
        parameters: Vec<Name>,
        block: NodeId,
    },
    Block {
        statements: Vec<NodeId>,
        tail: Option<NodeId>,
    },
    If {
        condition: NodeId,
        then_branch: NodeId,
        else_branch: Option<NodeId>,
    },
    WhileLoop {
        condition: NodeId,
        block: NodeId,
    },
    ForLoop {
        start: NodeId,
        end: NodeId,
        block: NodeId,
    },
    Return(NodeId),
    Break,
    Continue,
}

impl Ast {
    fn insert(&mut self, node: AstNode, span: Span) -> NodeId {
        let id = NodeId(self.expressions.len() as u32);

        self.expressions.push(node);
        self.spans.push(span);

        id
    }

    pub fn last(&self) -> NodeId {
        NodeId((self.expressions.len() - 1) as u32)
    }

    pub fn node(&self, id: NodeId) -> &AstNode {
        &self.expressions[id.0 as usize]
    }

    pub fn span(&self, id: NodeId) -> Span {
        self.spans[id.0 as usize]
    }

    pub fn binary(
        &mut self,
        operator: BinaryOp,
        left: NodeId,
        right: NodeId,
        span: Span,
    ) -> NodeId {
        self.insert(
            AstNode::Binary {
                operator,
                left,
                right,
            },
            span,
        )
    }

    pub fn logical_and(&mut self, left: NodeId, right: NodeId, span: Span) -> NodeId {
        self.insert(AstNode::LogicalAnd { left, right }, span)
    }

    pub fn logical_or(&mut self, left: NodeId, right: NodeId, span: Span) -> NodeId {
        self.insert(AstNode::LogicalOr { left, right }, span)
    }

    pub fn logical_not(&mut self, expression: NodeId, span: Span) -> NodeId {
        self.insert(AstNode::LogicalNot(expression), span)
    }

    pub fn unary(&mut self, operator: UnaryOp, right: NodeId, span: Span) -> NodeId {
        self.insert(AstNode::Unary { operator, right }, span)
    }

    pub fn assign(&mut self, left: NodeId, right: NodeId, span: Span) -> NodeId {
        self.insert(AstNode::Assign { left, right }, span)
    }

    pub fn compound_assign(
        &mut self,
        operator: CompoundOp,
        left: NodeId,
        right: NodeId,
        span: Span,
    ) -> NodeId {
        self.insert(
            AstNode::CompoundAssign {
                operator,
                left,
                right,
            },
            span,
        )
    }

    pub fn variable(&mut self, left: Name, right: NodeId, span: Span) -> NodeId {
        self.insert(AstNode::Variable { left, right }, span)
    }

    pub fn mut_(&mut self, left: Name, right: NodeId, span: Span) -> NodeId {
        self.insert(AstNode::Mut { left, right }, span)
    }

    pub fn identifier(&mut self, name: Name) -> NodeId {
        self.insert(AstNode::Identifier(name), name.span)
    }

    pub fn string_literal(&mut self, index: Symbol, span: Span) -> NodeId {
        self.insert(AstNode::StringLiteral(index), span)
    }

    pub fn number_literal(&mut self, value: f64, span: Span) -> NodeId {
        self.insert(AstNode::NumberLiteral(value), span)
    }

    pub fn boolean_literal(&mut self, value: bool, span: Span) -> NodeId {
        self.insert(AstNode::BooleanLiteral(value), span)
    }

    pub fn nil_literal(&mut self, span: Span) -> NodeId {
        self.insert(AstNode::NilLiteral, span)
    }

    pub fn function_call(&mut self, callee: NodeId, arguments: Vec<NodeId>, span: Span) -> NodeId {
        self.insert(AstNode::FunctionCall { callee, arguments }, span)
    }

    pub fn member_access(&mut self, object: NodeId, property: Name, span: Span) -> NodeId {
        self.insert(AstNode::MemberAccess { object, property }, span)
    }

    pub fn dict_literal(&mut self, fields: Vec<(NodeId, NodeId)>, span: Span) -> NodeId {
        self.insert(AstNode::DictLiteral { fields }, span)
    }

    pub fn native_function(&mut self, name: Name, parameters: Vec<Name>, span: Span) -> NodeId {
        self.insert(AstNode::NativeFunction { name, parameters }, span)
    }

    pub fn function(
        &mut self,
        name: Option<Name>,
        parameters: Vec<Name>,
        block: NodeId,
        span: Span,
    ) -> NodeId {
        self.insert(
            AstNode::Function {
                name,
                parameters,
                block,
            },
            span,
        )
    }

    pub fn block(&mut self, statements: Vec<NodeId>, tail: Option<NodeId>, span: Span) -> NodeId {
        self.insert(AstNode::Block { statements, tail }, span)
    }

    pub fn if_(
        &mut self,
        condition: NodeId,
        then_branch: NodeId,
        else_branch: Option<NodeId>,
        span: Span,
    ) -> NodeId {
        self.insert(
            AstNode::If {
                condition,
                then_branch,
                else_branch,
            },
            span,
        )
    }

    pub fn while_loop(&mut self, condition: NodeId, block: NodeId, span: Span) -> NodeId {
        self.insert(AstNode::WhileLoop { condition, block }, span)
    }

    pub fn for_loop(&mut self, start: NodeId, end: NodeId, block: NodeId, span: Span) -> NodeId {
        self.insert(AstNode::ForLoop { start, end, block }, span)
    }

    pub fn return_(&mut self, expression: NodeId, span: Span) -> NodeId {
        self.insert(AstNode::Return(expression), span)
    }

    pub fn break_(&mut self, span: Span) -> NodeId {
        self.insert(AstNode::Break, span)
    }

    pub fn continue_(&mut self, span: Span) -> NodeId {
        self.insert(AstNode::Continue, span)
    }
}
