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
    spans: Vec<Option<Span>>,
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
        property: NodeId,
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
    Block(Vec<NodeId>),
    If {
        condition: NodeId,
        then_branch: NodeId,
        else_branch: NodeId,
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
    fn insert(&mut self, node: AstNode, span: Option<Span>) -> NodeId {
        let id = NodeId(self.expressions.len() as u32);

        self.expressions.push(node);
        self.spans.push(span);

        id
    }

    pub fn last(&self) -> NodeId {
        NodeId((self.expressions.len() - 1) as u32)
    }

    pub fn get_node(&self, id: NodeId) -> &AstNode {
        &self.expressions[id.0 as usize]
    }

    pub fn get_span(&self, id: NodeId) -> Option<Span> {
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
            Some(span),
        )
    }

    pub fn logical_and(&mut self, left: NodeId, right: NodeId, span: Span) -> NodeId {
        self.insert(AstNode::LogicalAnd { left, right }, Some(span))
    }

    pub fn logical_or(&mut self, left: NodeId, right: NodeId, span: Span) -> NodeId {
        self.insert(AstNode::LogicalOr { left, right }, Some(span))
    }

    pub fn logical_not(&mut self, expression: NodeId, span: Span) -> NodeId {
        self.insert(AstNode::LogicalNot(expression), Some(span))
    }

    pub fn unary(&mut self, operator: UnaryOp, right: NodeId, span: Span) -> NodeId {
        self.insert(AstNode::Unary { operator, right }, Some(span))
    }

    pub fn assign(&mut self, left: NodeId, right: NodeId, span: Span) -> NodeId {
        self.insert(AstNode::Assign { left, right }, Some(span))
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
            Some(span),
        )
    }

    pub fn variable(&mut self, left: Name, right: NodeId) -> NodeId {
        self.insert(AstNode::Variable { left, right }, None)
    }

    pub fn mut_(&mut self, left: Name, right: NodeId) -> NodeId {
        self.insert(AstNode::Mut { left, right }, None)
    }

    pub fn identifier(&mut self, name: Name) -> NodeId {
        self.insert(AstNode::Identifier(name), None)
    }

    pub fn string_literal(&mut self, index: Symbol, span: Span) -> NodeId {
        self.insert(AstNode::StringLiteral(index), Some(span))
    }

    pub fn number_literal(&mut self, value: f64, span: Span) -> NodeId {
        self.insert(AstNode::NumberLiteral(value), Some(span))
    }

    pub fn boolean_literal(&mut self, value: bool, span: Span) -> NodeId {
        self.insert(AstNode::BooleanLiteral(value), Some(span))
    }

    pub fn nil_literal(&mut self, span: Span) -> NodeId {
        self.insert(AstNode::NilLiteral, Some(span))
    }

    pub fn function_call(&mut self, callee: NodeId, arguments: Vec<NodeId>) -> NodeId {
        self.insert(AstNode::FunctionCall { callee, arguments }, None)
    }

    pub fn member_access(&mut self, object: NodeId, property: NodeId) -> NodeId {
        self.insert(AstNode::MemberAccess { object, property }, None)
    }

    pub fn dict_literal(&mut self, fields: Vec<(NodeId, NodeId)>) -> NodeId {
        self.insert(AstNode::DictLiteral { fields }, None)
    }

    pub fn native_function(&mut self, name: Name, parameters: Vec<Name>) -> NodeId {
        self.insert(AstNode::NativeFunction { name, parameters }, None)
    }

    pub fn function(&mut self, name: Option<Name>, parameters: Vec<Name>, block: NodeId) -> NodeId {
        self.insert(
            AstNode::Function {
                name,
                parameters,
                block,
            },
            None,
        )
    }

    pub fn block(&mut self, expressions: Vec<NodeId>) -> NodeId {
        self.insert(AstNode::Block(expressions), None)
    }

    pub fn if_(&mut self, condition: NodeId, then_branch: NodeId, else_branch: NodeId) -> NodeId {
        self.insert(
            AstNode::If {
                condition,
                then_branch,
                else_branch,
            },
            None,
        )
    }

    pub fn while_loop(&mut self, condition: NodeId, block: NodeId) -> NodeId {
        self.insert(AstNode::WhileLoop { condition, block }, None)
    }

    pub fn for_loop(&mut self, start: NodeId, end: NodeId, block: NodeId) -> NodeId {
        self.insert(AstNode::ForLoop { start, end, block }, None)
    }

    pub fn return_(&mut self, expression: NodeId, span: Span) -> NodeId {
        self.insert(AstNode::Return(expression), Some(span))
    }

    pub fn break_(&mut self, span: Span) -> NodeId {
        self.insert(AstNode::Break, Some(span))
    }

    pub fn continue_(&mut self, span: Span) -> NodeId {
        self.insert(AstNode::Continue, Some(span))
    }
}
