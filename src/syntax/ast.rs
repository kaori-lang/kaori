use std::hash::Hash;

use crate::{
    syntax::{
        ops::{BinaryOp, UnaryOp},
        token::Span,
    },
    util::string_interner::Symbol,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(u32);

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

impl Hash for Spanned<Symbol> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq for Spanned<Symbol> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Spanned<Symbol> {}

#[derive(Debug, Default)]
pub struct Ast {
    nodes: Vec<Spanned<Node>>,
}

#[derive(Debug)]
pub enum Node {
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
        operand: NodeId,
    },
    Assign {
        left: NodeId,
        right: NodeId,
    },
    Variable {
        left: Spanned<Symbol>,
        right: NodeId,
    },
    Constant {
        left: Spanned<Symbol>,
        right: NodeId,
    },
    Ref {
        left: Spanned<Symbol>,
        right: NodeId,
    },
    Identifier(Spanned<Symbol>),
    String(Symbol),
    Number(f64),
    Boolean(bool),
    Nil,
    FunctionCall {
        callee: NodeId,
        arguments: Vec<NodeId>,
    },
    MemberAccess {
        object: NodeId,
        property: Spanned<Symbol>,
    },
    Map {
        entries: Vec<(NodeId, Option<NodeId>)>,
    },
    Function {
        name: Spanned<Symbol>,
        parameters: Vec<Spanned<Symbol>>,
        block: NodeId,
    },
    Lambda {
        parameters: Vec<Spanned<Symbol>>,
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
    Return(NodeId),
    Break,
    Continue,
    Use {
        path: Vec<Spanned<Symbol>>,
        bindings: Vec<Spanned<Symbol>>,
    },
}

impl Ast {
    fn insert(&mut self, node: Node, span: Span) -> NodeId {
        let id = NodeId(self.nodes.len() as u32);

        self.nodes.push(Spanned::new(node, span));

        id
    }

    pub fn last(&self) -> NodeId {
        NodeId((self.nodes.len() - 1) as u32)
    }

    pub fn node(&self, id: NodeId) -> &Node {
        &self.nodes[id.0 as usize].value
    }

    pub fn node_mut(&mut self, id: NodeId) -> &mut Node {
        &mut self.nodes[id.0 as usize].value
    }

    pub fn span(&self, id: NodeId) -> Span {
        self.nodes[id.0 as usize].span
    }

    pub fn binary(
        &mut self,
        operator: BinaryOp,
        left: NodeId,
        right: NodeId,
        span: Span,
    ) -> NodeId {
        self.insert(
            Node::Binary {
                operator,
                left,
                right,
            },
            span,
        )
    }

    pub fn logical_and(&mut self, left: NodeId, right: NodeId, span: Span) -> NodeId {
        self.insert(Node::LogicalAnd { left, right }, span)
    }

    pub fn logical_or(&mut self, left: NodeId, right: NodeId, span: Span) -> NodeId {
        self.insert(Node::LogicalOr { left, right }, span)
    }

    pub fn logical_not(&mut self, expression: NodeId, span: Span) -> NodeId {
        self.insert(Node::LogicalNot(expression), span)
    }

    pub fn unary(&mut self, operator: UnaryOp, operand: NodeId, span: Span) -> NodeId {
        self.insert(Node::Unary { operator, operand }, span)
    }

    pub fn assign(&mut self, left: NodeId, right: NodeId, span: Span) -> NodeId {
        self.insert(Node::Assign { left, right }, span)
    }

    pub fn variable(&mut self, left: Spanned<Symbol>, right: NodeId, span: Span) -> NodeId {
        self.insert(Node::Variable { left, right }, span)
    }

    pub fn constant(&mut self, left: Spanned<Symbol>, right: NodeId, span: Span) -> NodeId {
        self.insert(Node::Constant { left, right }, span)
    }

    pub fn ref_(&mut self, left: Spanned<Symbol>, right: NodeId, span: Span) -> NodeId {
        self.insert(Node::Ref { left, right }, span)
    }

    pub fn identifier(&mut self, name: Spanned<Symbol>) -> NodeId {
        self.insert(Node::Identifier(name), name.span)
    }

    pub fn string(&mut self, index: Symbol, span: Span) -> NodeId {
        self.insert(Node::String(index), span)
    }

    pub fn number(&mut self, value: f64, span: Span) -> NodeId {
        self.insert(Node::Number(value), span)
    }

    pub fn boolean(&mut self, value: bool, span: Span) -> NodeId {
        self.insert(Node::Boolean(value), span)
    }

    pub fn nil(&mut self, span: Span) -> NodeId {
        self.insert(Node::Nil, span)
    }

    pub fn function_call(&mut self, callee: NodeId, arguments: Vec<NodeId>, span: Span) -> NodeId {
        self.insert(Node::FunctionCall { callee, arguments }, span)
    }

    pub fn member_access(
        &mut self,
        object: NodeId,
        property: Spanned<Symbol>,
        span: Span,
    ) -> NodeId {
        self.insert(Node::MemberAccess { object, property }, span)
    }

    pub fn map(&mut self, entries: Vec<(NodeId, Option<NodeId>)>, span: Span) -> NodeId {
        self.insert(Node::Map { entries }, span)
    }

    pub fn function(
        &mut self,
        name: Spanned<Symbol>,
        parameters: Vec<Spanned<Symbol>>,
        block: NodeId,
        span: Span,
    ) -> NodeId {
        self.insert(
            Node::Function {
                name,
                parameters,
                block,
            },
            span,
        )
    }

    pub fn lambda(
        &mut self,
        parameters: Vec<Spanned<Symbol>>,
        block: NodeId,
        span: Span,
    ) -> NodeId {
        self.insert(Node::Lambda { parameters, block }, span)
    }

    pub fn block(&mut self, statements: Vec<NodeId>, tail: Option<NodeId>, span: Span) -> NodeId {
        self.insert(Node::Block { statements, tail }, span)
    }

    pub fn if_(
        &mut self,
        condition: NodeId,
        then_branch: NodeId,
        else_branch: Option<NodeId>,
        span: Span,
    ) -> NodeId {
        self.insert(
            Node::If {
                condition,
                then_branch,
                else_branch,
            },
            span,
        )
    }

    pub fn while_loop(&mut self, condition: NodeId, block: NodeId, span: Span) -> NodeId {
        self.insert(Node::WhileLoop { condition, block }, span)
    }

    pub fn return_(&mut self, expression: NodeId, span: Span) -> NodeId {
        self.insert(Node::Return(expression), span)
    }

    pub fn break_(&mut self, span: Span) -> NodeId {
        self.insert(Node::Break, span)
    }

    pub fn continue_(&mut self, span: Span) -> NodeId {
        self.insert(Node::Continue, span)
    }

    pub fn use_(
        &mut self,
        path: Vec<Spanned<Symbol>>,
        bindings: Vec<Spanned<Symbol>>,
        span: Span,
    ) -> NodeId {
        self.insert(Node::Use { path, bindings }, span)
    }
}
