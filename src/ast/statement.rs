use crate::{interpreter::interpreter::Interpreter, lexer::token::TokenType, yf_error::ErrorType};

use super::expression::Expression;

pub trait Statement: std::fmt::Debug {
    fn accept_visitor(&self, visitor: &mut Interpreter) -> Result<(), ErrorType>;
}

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
pub struct BlockStatement {
    pub statements: Vec<Box<dyn Statement>>,
    pub line: u32,
}

impl Statement for VariableDeclStatement {
    fn accept_visitor(&self, visitor: &mut Interpreter) -> Result<(), ErrorType> {
        visitor.visit_variable_decl_statement(self)
    }
}

impl Statement for PrintStatement {
    fn accept_visitor(&self, visitor: &mut Interpreter) -> Result<(), ErrorType> {
        visitor.visit_print_statement(self)
    }
}

impl Statement for ExpressionStatement {
    fn accept_visitor(&self, visitor: &mut Interpreter) -> Result<(), ErrorType> {
        visitor.visit_expr_statement(self)
    }
}

impl Statement for BlockStatement {
    fn accept_visitor(&self, visitor: &mut Interpreter) -> Result<(), ErrorType> {
        visitor.visit_block_statement(self)
    }
}

impl Statement for IfStatement {
    fn accept_visitor(&self, visitor: &mut Interpreter) -> Result<(), ErrorType> {
        visitor.visit_if_statement(self)
    }
}
