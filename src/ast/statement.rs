use crate::{lexer::token_type::TokenType, yf_error::ErrorType};

use super::expression::Expression;

pub trait Statement: std::fmt::Debug {}

#[derive(Debug)]
pub struct VariableDeclStatement {
    pub data_type: TokenType,
    pub identifier: String,
    pub data: Box<dyn Expression>,
    pub line: u32,
}

#[derive(Debug)]
pub struct PrintStatement {
    pub expression: Box<dyn Expression>,
    pub line: u32,
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expression: Box<dyn Expression>,
    pub line: u32,
}

#[derive(Debug)]
pub struct IfStatement {
    pub condition: Box<dyn Expression>,
    pub then_branch: Box<dyn Statement>,
    pub else_branch: Option<Box<dyn Statement>>,
    pub line: u32,
}

#[derive(Debug)]
pub struct WhileLoopStatement {
    pub condition: Box<dyn Expression>,
    pub block: Box<dyn Statement>,
    pub line: u32,
}

#[derive(Debug)]
pub struct BlockStatement {
    pub statements: Vec<Box<dyn Statement>>,
    pub line: u32,
}

#[derive(Debug)]
pub struct BreakStatement {
    pub line: u32,
}

#[derive(Debug)]
pub struct ContinueStatement {
    pub line: u32,
}
