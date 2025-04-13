use crate::{
    token::{DataType, TokenType},
    yf_error::{RuntimeError, YFError},
};

use super::{
    data::Data,
    environment::Environment,
    expr::{BinaryOperator, Expr, Identifier, Literal, UnaryOperator},
    stmt::{PrintStmt, Stmt, VariableDeclStmt},
};

pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), YFError> {
        self.env.enter_scope();

        for stmt in statements.iter() {
            self.eval_stmt(stmt)?;
        }

        self.env.exit_scope();

        Ok(())
    }

    pub fn eval_stmt(&mut self, stmt: &Stmt) -> Result<(), YFError> {
        match stmt {
            Stmt::ExprStmt(expr) => self.eval_expr_stmt(expr),
            Stmt::VariableDeclStmt(variable_decl_stmt) => {
                self.eval_var_decl_stmt(variable_decl_stmt)
            }
            Stmt::PrintStmt(print_stmt) => self.eval_print_stmt(print_stmt),
            _ => Err(YFError::RuntimeError(RuntimeError::InvalidEvaluation)),
        }
    }

    fn eval_expr_stmt(&self, expr: &Expr) -> Result<(), YFError> {
        self.eval_expr(expr)?;
        Ok(())
    }

    fn eval_print_stmt(&self, stmt: &PrintStmt) -> Result<(), YFError> {
        println!("{:?}", self.eval_expr(&stmt.value)?);

        Ok(())
    }

    fn eval_var_decl_stmt(&mut self, stmt: &VariableDeclStmt) -> Result<(), YFError> {
        let data_type = &stmt.data_type;
        let data = self.eval_expr(&stmt.data)?;
        let identifier = stmt.identifier.clone();

        self.env.create_symbol(identifier, data)?;
        Ok(())
    }

    pub fn eval_expr(&self, node: &Expr) -> Result<Data, YFError> {
        match node {
            Expr::Literal(literal) => self.eval_literal(literal),
            Expr::BinaryOperator(binary) => self.eval_binary(binary),
            Expr::UnaryOperator(unary) => self.eval_unary(unary),
            Expr::Identifier(identifier) => self.eval_identifier(identifier),
        }
    }

    fn eval_binary(&self, node: &BinaryOperator) -> Result<Data, YFError> {
        let left = self.eval_expr(&node.left)?;
        let right = self.eval_expr(&node.right)?;
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
            _ => return Err(YFError::RuntimeError(RuntimeError::InvalidEvaluation)),
        }
    }

    fn eval_identifier(&self, node: &Identifier) -> Result<Data, YFError> {
        let Identifier { ty, value } = node;

        return self.env.get_symbol(&value);
    }

    fn eval_literal(&self, node: &Literal) -> Result<Data, YFError> {
        let ty = &node.ty;
        let value = &node.value;

        match ty {
            DataType::Number => Ok(Data::Number(value.parse::<f64>().unwrap())),
            DataType::String => Ok(Data::String(value.clone())),
            DataType::Boolean => Ok(Data::Boolean(value.parse::<bool>().unwrap())),
            _ => Err(YFError::RuntimeError(RuntimeError::InvalidEvaluation)),
        }
    }

    fn eval_unary(&self, node: &UnaryOperator) -> Result<Data, YFError> {
        let ty = &node.ty;
        let right = self.eval_expr(&node.right)?;

        match (ty, right) {
            (TokenType::Minus, Data::Number(r)) => Ok(Data::Number(-r)),
            (TokenType::Not, Data::Boolean(r)) => Ok(Data::Boolean(!r)),
            _ => Err(YFError::RuntimeError(RuntimeError::InvalidEvaluation)),
        }
    }
}
