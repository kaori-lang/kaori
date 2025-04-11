use crate::token::{DataType, TokenType};

use super::{data::Data, environment::EnvironmentError};

#[derive(Debug)]
pub enum RuntimeError {
    EnvironmentError(EnvironmentError),
    InvalidEvaluation,
}

#[derive(Debug)]
pub enum Expr {
    BinaryOperator {
        ty: TokenType,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnaryOperator {
        ty: TokenType,
        right: Box<Expr>,
    },
    Identifier {
        ty: TokenType,
        value: String,
    },
    Literal {
        ty: DataType,
        value: String,
    },
}

impl Expr {
    pub fn accept(&self) -> Result<Data, RuntimeError> {
        match self {
            Expr::Literal { .. } => self.visit_literal(),
            Expr::BinaryOperator { .. } => self.visit_binary(),
            Expr::UnaryOperator { .. } => self.visit_unary(),
            Expr::Identifier { .. } => self.visit_identifier(),
        }
    }

    fn visit_binary(&self) -> Result<Data, RuntimeError> {
        let Expr::BinaryOperator { ty, left, right } = self else {
            panic!("Expected binary operator");
        };

        let left = left.accept()?;
        let right = right.accept()?;

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

    fn visit_identifier(&self) -> Result<Data, RuntimeError> {
        return Ok(Data::Number(1.0));
    }

    fn visit_literal(&self) -> Result<Data, RuntimeError> {
        let Expr::Literal { ty, value } = self else {
            panic!("RuntimeError")
        };

        match ty {
            DataType::Number => Ok(Data::Number(value.parse::<f64>().unwrap())),
            DataType::String => Ok(Data::String(value.clone())),
            DataType::Boolean => Ok(Data::Boolean(value.parse::<bool>().unwrap())),
            _ => Err(RuntimeError::InvalidEvaluation),
        }
    }

    fn visit_unary(&self) -> Result<Data, RuntimeError> {
        let Expr::UnaryOperator { ty, right } = self else {
            panic!("RuntimeError")
        };

        let right = right.accept()?;

        match (ty, right) {
            (TokenType::Minus, Data::Number(r)) => Ok(Data::Number(-r)),
            (TokenType::Not, Data::Boolean(r)) => Ok(Data::Boolean(!r)),
            _ => Err(RuntimeError::InvalidEvaluation),
        }
    }
}
