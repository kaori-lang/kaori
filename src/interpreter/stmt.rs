use crate::token::DataType;

use super::expr::Expr;

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
