use crate::{
    ast::{
        expression::{AssignOperator, BinaryOperator, Identifier, Literal, UnaryOperator},
        statement::{
            BlockStatement, ExpressionStatement, PrintStatement, Statement, VariableDeclStatement,
        },
    },
    lexer::{data::Data, token::TokenType},
    yf_error::{ErrorType, YFError},
};

use super::environment::Environment;

pub struct Interpreter {
    env: Environment,
    line: u32,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
            line: 1,
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Box<dyn Statement>>) -> Result<(), YFError> {
        self.env.enter_scope();

        match self.interpret_statements(statements) {
            Err(error_type) => {
                return Err(YFError {
                    error_type,
                    line: self.line,
                });
            }
            Ok(_) => (),
        };
        self.env.exit_scope();

        return Ok(());
    }

    fn interpret_statements(
        &mut self,
        statements: &Vec<Box<dyn Statement>>,
    ) -> Result<(), ErrorType> {
        for stmt in statements.iter() {
            self.execute(stmt)?;
        }

        return Ok(());
    }

    pub fn execute(&mut self, stmt: &Box<dyn Statement>) -> Result<(), ErrorType> {
        stmt.accept_visitor(self)?;

        return Ok(());
    }

    pub fn visit_block_statement(&mut self, stmt: &BlockStatement) -> Result<(), ErrorType> {
        self.env.enter_scope();
        self.interpret_statements(&stmt.statements)?;
        self.env.exit_scope();
        return Ok(());
    }

    pub fn visit_expr_statement(&mut self, stmt: &ExpressionStatement) -> Result<(), ErrorType> {
        self.line = stmt.line;

        stmt.expression.accept_visitor(self)?;

        return Ok(());
    }

    pub fn visit_print_statement(&mut self, stmt: &PrintStatement) -> Result<(), ErrorType> {
        self.line = stmt.line;

        println!("{}", stmt.expression.accept_visitor(self)?);

        return Ok(());
    }

    pub fn visit_variable_decl_statement(
        &mut self,
        stmt: &VariableDeclStatement,
    ) -> Result<(), ErrorType> {
        self.line = stmt.line;

        let data_type = &stmt.data_type;
        let data = stmt.data.accept_visitor(self)?;
        let identifier = stmt.identifier.clone();

        self.env.create_symbol(identifier, data)?;

        return Ok(());
    }

    pub fn visit_assign_operator(&mut self, node: &AssignOperator) -> Result<Data, ErrorType> {
        let identifier = &node.identifier.value;
        let data = node.right.accept_visitor(self)?;

        self.env.update_symbol(identifier, &data)?;

        return Ok(data);
    }

    pub fn visit_binary_operator(&mut self, node: &BinaryOperator) -> Result<Data, ErrorType> {
        let left = node.left.accept_visitor(self)?;
        let right = node.right.accept_visitor(self)?;
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
            _ => return Err(ErrorType::TypeError),
        }
    }

    pub fn visit_identifier(&mut self, node: &Identifier) -> Result<Data, ErrorType> {
        let Identifier { value } = node;

        return self.env.get_symbol(&value);
    }

    pub fn visit_literal(&mut self, node: &Literal) -> Result<Data, ErrorType> {
        return Ok(node.value.clone());
    }

    pub fn visit_unary_operator(&mut self, node: &UnaryOperator) -> Result<Data, ErrorType> {
        let ty = &node.ty;
        let right = node.right.accept_visitor(self)?;

        match (ty, right) {
            (TokenType::Minus, Data::Number(r)) => Ok(Data::Number(-r)),
            (TokenType::Not, Data::Boolean(r)) => Ok(Data::Boolean(!r)),
            _ => Err(ErrorType::TypeError),
        }
    }
}
