use std::any::Any;

use crate::{
    interpreter::{data::Data, interpreter::Interpreter},
    token::{DataType, TokenType},
    yf_error::ErrorType,
};

pub trait Expression: std::fmt::Debug {
    fn accept_visitor(&self, visitor: &mut Interpreter) -> Result<Data, ErrorType>;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub struct BinaryOperator {
    pub ty: TokenType,
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

#[derive(Debug)]
pub struct UnaryOperator {
    pub ty: TokenType,
    pub right: Box<dyn Expression>,
}

#[derive(Debug)]
pub struct AssignOperator {
    pub identifier: Identifier,
    pub right: Box<dyn Expression>,
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub value: String,
}

#[derive(Debug)]
pub struct Literal {
    pub ty: DataType,
    pub value: String,
}

impl Expression for BinaryOperator {
    fn accept_visitor(&self, visitor: &mut Interpreter) -> Result<Data, ErrorType> {
        visitor.visit_binary_operator(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for UnaryOperator {
    fn accept_visitor(&self, visitor: &mut Interpreter) -> Result<Data, ErrorType> {
        visitor.visit_unary_operator(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for Literal {
    fn accept_visitor(&self, visitor: &mut Interpreter) -> Result<Data, ErrorType> {
        visitor.visit_literal(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for Identifier {
    fn accept_visitor(&self, visitor: &mut Interpreter) -> Result<Data, ErrorType> {
        visitor.visit_identifier(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for AssignOperator {
    fn accept_visitor(&self, visitor: &mut Interpreter) -> Result<Data, ErrorType> {
        visitor.visit_assign_operator(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
