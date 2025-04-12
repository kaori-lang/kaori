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
    pub ty: DataType,
    pub name: String,
    pub value: Box<Expr>,
}

#[derive(Debug)]
pub struct PrintStmt {
    value: Box<Expr>,
}

impl VariableDeclStmt {
    pub fn new(ty: DataType, name: String, value: Box<Expr>) -> Self {
        Self { ty, name, value }
    }
}

impl PrintStmt {
    pub fn new(value: Box<Expr>) -> Self {
        Self { value }
    }
}
