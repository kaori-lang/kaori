use crate::token::DataType;

use super::{
    data::Data,
    expr::{Expr, RuntimeError},
};

#[derive(Debug)]
pub enum Stmt {
    ExprStmt(Box<Expr>),
    VariableDeclStmt {
        ty: DataType,
        name: String,
        value: Box<Expr>,
    },
    PrintStmt {
        value: Box<Expr>,
    },
}

impl Stmt {
    pub fn eval(&self) -> Result<Option<Data>, RuntimeError> {
        match self {
            Stmt::ExprStmt(e) => Ok(Some(e.accept()?)),
            _ => Ok(None),
        }
    }
}
