use crate::token::DataType;

use super::expr::Expr;

#[derive(Debug)]
pub enum Stmt {
    ExprStmt(Box<Expr>),
    VariableDeclStmt(VariableDeclStmt),
    PrintStmt(PrintStmt),
}

#[derive(Debug)]
pub struct VariableDeclStmt {
    pub data_type: DataType,
    pub identifier: String,
    pub data: Box<Expr>,
}

#[derive(Debug)]
pub struct PrintStmt {
    pub value: Box<Expr>,
}

impl VariableDeclStmt {
    pub fn new(data_type: DataType, identifier: String, data: Box<Expr>) -> Self {
        Self {
            data_type,
            identifier,
            data,
        }
    }
}

impl PrintStmt {
    pub fn new(value: Box<Expr>) -> Self {
        Self { value }
    }
}
