use crate::{token::DataType, yf_error::ErrorType};

use super::{expression::Expression, interpreter::Interpreter};

pub trait Statement: std::fmt::Debug {
    fn accept_visitor(&self, visitor: &mut Interpreter) -> Result<(), ErrorType>;
}

#[derive(Debug)]
pub struct VariableDeclStatement {
    pub data_type: DataType,
    pub identifier: String,
    pub data: Box<dyn Expression>,
}

#[derive(Debug)]
pub struct PrintStatement {
    pub value: Box<dyn Expression>,
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub value: Box<dyn Expression>,
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
