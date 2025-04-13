use crate::{
    token::{DataType, TokenType},
    yf_error::ErrorType,
};

use super::{data::Data, interpreter::Interpreter};

pub trait Expression: std::fmt::Debug {
    fn accept_visitor(&self, visitor: &Interpreter) -> Result<Data, ErrorType>;
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
pub struct Identifier {
    pub ty: TokenType,
    pub value: String,
}

#[derive(Debug)]
pub struct Literal {
    pub ty: DataType,
    pub value: String,
}

impl Expression for BinaryOperator {
    fn accept_visitor(&self, visitor: &Interpreter) -> Result<Data, ErrorType> {
        visitor.visit_binary_operator(self)
    }
}

impl Expression for UnaryOperator {
    fn accept_visitor(&self, visitor: &Interpreter) -> Result<Data, ErrorType> {
        visitor.visit_unary_operator(self)
    }
}

impl Expression for Literal {
    fn accept_visitor(&self, visitor: &Interpreter) -> Result<Data, ErrorType> {
        visitor.visit_literal(self)
    }
}

impl Expression for Identifier {
    fn accept_visitor(&self, visitor: &Interpreter) -> Result<Data, ErrorType> {
        visitor.visit_identifier(self)
    }
}
