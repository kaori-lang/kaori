use crate::token::{DataType, TokenType};

use super::{
    data::Data,
    environment::Environment,
    error::RuntimeError,
    expr::{BinaryOperator, Expr, Identifier, Literal, UnaryOperator},
    stmt::Stmt,
};

pub struct Interpreter {
    statements: Vec<Stmt>,
    env: Environment,
}

impl Interpreter {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Self {
            statements,
            env: Environment::new(None),
        }
    }

    pub fn interpret(&self) -> Result<(), RuntimeError> {
        for stat in self.statements.iter() {
            self.visit_stmt(stat)?;
        }

        Ok(())
    }

    pub fn visit_stmt(&self, stmt: &Stmt) -> Result<Data, RuntimeError> {
        match stmt {
            Stmt::ExprStmt(expr) => self.visit_expr(expr),
            _ => Err(RuntimeError::InvalidEvaluation),
        }
    }

    pub fn visit_expr(&self, node: &Expr) -> Result<Data, RuntimeError> {
        match node {
            Expr::Literal(literal) => self.visit_literal(literal),
            Expr::BinaryOperator(binary) => self.visit_binary(binary),
            Expr::UnaryOperator(unary) => self.visit_unary(unary),
            Expr::Identifier(identifier) => self.visit_identifier(identifier),
        }
    }

    fn visit_binary(&self, node: &BinaryOperator) -> Result<Data, RuntimeError> {
        let left = self.visit_expr(&node.left)?;
        let right = self.visit_expr(&node.right)?;
        let ty = &node.ty;

        use {Data as E, TokenType as T};
        match (ty, left, right) {
            (T::Plus, E::Number(l), E::Number(r)) => Ok(E::Number(l + r)),
            (T::Plus, E::String(l), E::String(r)) => Ok(E::String(format!("{l}{r}"))),
            (T::Minus, E::Number(l), E::Number(r)) => Ok(E::Number(l - r)),
            (T::Multiply, E::Number(l), E::Number(r)) => Ok(E::Number(l * r)),
            (T::Divide, E::Number(l), E::Number(r)) => Ok(E::Number(l / r)),
            (T::And, E::Boolean(l), E::Boolean(r)) => Ok(E::Boolean(l && r)),
            (T::Or, E::Boolean(l), E::Boolean(r)) => Ok(E::Boolean(l || r)),
            (T::Equal, E::Number(l), E::Number(r)) => Ok(E::Boolean(l == r)),
            (T::NotEqual, E::Number(l), E::Number(r)) => Ok(E::Boolean(l != r)),
            (T::Greater, E::Number(l), E::Number(r)) => Ok(E::Boolean(l > r)),
            (T::GreaterEqual, E::Number(l), E::Number(r)) => Ok(E::Boolean(l >= r)),
            (T::Less, E::Number(l), E::Number(r)) => Ok(E::Boolean(l < r)),
            (T::LessEqual, E::Number(l), E::Number(r)) => Ok(E::Boolean(l <= r)),
            _ => return Err(RuntimeError::InvalidEvaluation),
        }
    }

    fn visit_identifier(&self, node: &Identifier) -> Result<Data, RuntimeError> {
        let ty = &node.ty;
        let value = &node.value;

        return self.env.get(&value);
    }

    fn visit_literal(&self, node: &Literal) -> Result<Data, RuntimeError> {
        let ty = &node.ty;
        let value = &node.value;

        match ty {
            DataType::Number => Ok(Data::Number(value.parse::<f64>().unwrap())),
            DataType::String => Ok(Data::String(value.clone())),
            DataType::Boolean => Ok(Data::Boolean(value.parse::<bool>().unwrap())),
            _ => Err(RuntimeError::InvalidEvaluation),
        }
    }

    fn visit_unary(&self, node: &UnaryOperator) -> Result<Data, RuntimeError> {
        let ty = &node.ty;
        let right = self.visit_expr(&node.right)?;

        match (ty, right) {
            (TokenType::Minus, Data::Number(r)) => Ok(Data::Number(-r)),
            (TokenType::Not, Data::Boolean(r)) => Ok(Data::Boolean(!r)),
            _ => Err(RuntimeError::InvalidEvaluation),
        }
    }
}
