use crate::{interpreter::interpreter::Interpreter, lexer::token::TokenType, yf_error::ErrorType};

use super::expression::Expression;

pub trait Statement: std::fmt::Debug {
    fn accept_visitor(&self, visitor: &mut Interpreter) -> Result<(), ErrorType>;
    fn get_line(&self) -> u32;
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
pub struct WhileStatement {
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

impl Statement for VariableDeclStatement {
    fn accept_visitor(&self, visitor: &mut Interpreter) -> Result<(), ErrorType> {
        visitor.visit_variable_decl_statement(self)
    }

    fn get_line(&self) -> u32 {
        self.line
    }
}

impl Statement for PrintStatement {
    fn accept_visitor(&self, visitor: &mut Interpreter) -> Result<(), ErrorType> {
        visitor.visit_print_statement(self)
    }

    fn get_line(&self) -> u32 {
        self.line
    }
}

impl Statement for ExpressionStatement {
    fn accept_visitor(&self, visitor: &mut Interpreter) -> Result<(), ErrorType> {
        visitor.visit_expr_statement(self)
    }

    fn get_line(&self) -> u32 {
        self.line
    }
}

impl Statement for BlockStatement {
    fn accept_visitor(&self, visitor: &mut Interpreter) -> Result<(), ErrorType> {
        visitor.visit_block_statement(self)
    }

    fn get_line(&self) -> u32 {
        self.line
    }
}

impl Statement for IfStatement {
    fn accept_visitor(&self, visitor: &mut Interpreter) -> Result<(), ErrorType> {
        visitor.visit_if_statement(self)
    }

    fn get_line(&self) -> u32 {
        self.line
    }
}

impl Statement for WhileStatement {
    fn accept_visitor(&self, visitor: &mut Interpreter) -> Result<(), ErrorType> {
        visitor.visit_while_statement(self)
    }

    fn get_line(&self) -> u32 {
        self.line
    }
}

impl Statement for BreakStatement {
    fn accept_visitor(&self, visitor: &mut Interpreter) -> Result<(), ErrorType> {
        visitor.visit_break_statement(self)
    }

    fn get_line(&self) -> u32 {
        self.line
    }
}

impl Statement for ContinueStatement {
    fn accept_visitor(&self, visitor: &mut Interpreter) -> Result<(), ErrorType> {
        Ok(())
    }

    fn get_line(&self) -> u32 {
        self.line
    }
}
