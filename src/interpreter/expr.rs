use crate::token::TokenType;

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
    Literal {
        ty: TokenType,
        value: String,
    },
}

#[derive(Debug)]
pub enum ExprEval {
    Number(f64),
    String(String),
    Boolean(bool),
}

impl Expr {
    pub fn eval(&self) -> ExprEval {
        match self {
            Expr::Literal { .. } => self.eval_literal(),
            Expr::BinaryOperator { .. } => self.eval_binary(),
            Expr::UnaryOperator { .. } => self.eval_unary(),
        }
    }

    fn eval_literal(&self) -> ExprEval {
        let Expr::Literal { ty, value } = self else {
            panic!("Error")
        };

        match ty {
            TokenType::Number => ExprEval::Number(value.parse::<f64>().unwrap()),
            _ => ExprEval::Number(value.parse::<f64>().expect("Invalid number literal")),
        }
    }

    fn eval_unary(&self) -> ExprEval {
        let Expr::UnaryOperator { ty, right } = self else {
            panic!("Error")
        };

        let right = right.eval();

        match (ty, right) {
            (TokenType::Minus, ExprEval::Number(r)) => ExprEval::Number(-r),
            (TokenType::Not, ExprEval::Boolean(r)) => ExprEval::Boolean(!r),
            _ => panic!("Invalid types can't do addition"),
        }
    }

    fn eval_binary(&self) -> ExprEval {
        let Expr::BinaryOperator { ty, left, right } = self else {
            panic!("Expected binary operator");
        };

        let left = left.eval();
        let right = right.eval();

        use {ExprEval as E, TokenType as T};
        match (ty, left, right) {
            (T::Plus, E::Number(l), E::Number(r)) => E::Number(l + r),
            (T::Plus, E::String(l), E::String(r)) => E::String(format!("{l}{r}")),
            (T::Minus, E::Number(l), E::Number(r)) => E::Number(l - r),
            (T::Multiply, E::Number(l), E::Number(r)) => E::Number(l * r),
            (T::Divide, E::Number(l), E::Number(r)) => E::Number(l / r),
            (T::And, E::Boolean(l), E::Boolean(r)) => E::Boolean(l && r),
            (T::Or, E::Boolean(l), E::Boolean(r)) => E::Boolean(l || r),
            (T::Equal, E::Number(l), E::Number(r)) => E::Boolean(l == r),
            (T::NotEqual, E::Number(l), E::Number(r)) => E::Boolean(l != r),
            (T::Greater, E::Number(l), E::Number(r)) => E::Boolean(l > r),
            (T::GreaterEqual, E::Number(l), E::Number(r)) => E::Boolean(l >= r),
            (T::Less, E::Number(l), E::Number(r)) => E::Boolean(l < r),
            (T::LessEqual, E::Number(l), E::Number(r)) => E::Boolean(l <= r),

            (ty, left, right) => panic!("Invalid operation: {ty:?} {left:?} {right:?}"),
        }
    }
}
