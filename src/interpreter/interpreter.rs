use crate::{
    ast::{
        expression::{AssignOperator, BinaryOperator, Identifier, Literal, UnaryOperator},
        statement::{
            BlockStatement, BreakStatement, ExpressionStatement, ForLoopStatement, IfStatement,
            PrintStatement, Statement, VariableDeclStatement, WhileLoopStatement,
        },
    },
    lexer::{data::Data, token_type::TokenType},
    yf_error::{ErrorType, YFError},
};

use super::environment::Environment;

pub struct Interpreter {
    env: Environment,
    line: u32,
    is_breaking: bool,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
            line: 1,
            is_breaking: false,
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
            if self.is_breaking {
                break;
            }

            self.execute(stmt)?;
        }

        return Ok(());
    }

    pub fn execute(&mut self, stmt: &Box<dyn Statement>) -> Result<(), ErrorType> {
        self.line = stmt.get_line();

        stmt.accept_visitor(self)?;

        return Ok(());
    }

    pub fn visit_block_statement(&mut self, stmt: &BlockStatement) -> Result<(), ErrorType> {
        self.env.enter_scope();
        self.interpret_statements(&stmt.statements)?;
        self.env.exit_scope();
        return Ok(());
    }

    pub fn visit_break_statement(&mut self, stmt: &BreakStatement) -> Result<(), ErrorType> {
        self.is_breaking = true;

        return Ok(());
    }

    pub fn visit_while_loop_statement(
        &mut self,
        stmt: &WhileLoopStatement,
    ) -> Result<(), ErrorType> {
        loop {
            let is_truthy = stmt.condition.accept_visitor(self)?;

            match is_truthy {
                Data::Boolean(true) => stmt.block.accept_visitor(self)?,
                Data::Boolean(false) => break,
                _ => return Err(ErrorType::TypeError),
            };

            if self.is_breaking {
                self.is_breaking = false;
                break;
            }
        }

        return Ok(());
    }

    pub fn visit_for_loop_statement(&mut self, stmt: &ForLoopStatement) -> Result<(), ErrorType> {
        stmt.declaration.accept_visitor(self)?;

        loop {
            let is_truthy = stmt.condition.accept_visitor(self)?;

            match is_truthy {
                Data::Boolean(true) => stmt.block.accept_visitor(self)?,
                Data::Boolean(false) => break,
                _ => return Err(ErrorType::TypeError),
            };

            if self.is_breaking {
                self.is_breaking = false;
                break;
            }

            stmt.increment.accept_visitor(self)?;
        }

        return Ok(());
    }

    pub fn visit_if_statement(&mut self, stmt: &IfStatement) -> Result<(), ErrorType> {
        let is_truthy = stmt.condition.accept_visitor(self)?;

        match is_truthy {
            Data::Boolean(true) => stmt.then_branch.accept_visitor(self)?,
            Data::Boolean(false) => {
                if let Some(else_branch) = &stmt.else_branch {
                    else_branch.accept_visitor(self)?;
                }
            }
            _ => return Err(ErrorType::TypeError),
        };

        return Ok(());
    }

    pub fn visit_expr_statement(&mut self, stmt: &ExpressionStatement) -> Result<(), ErrorType> {
        stmt.expression.accept_visitor(self)?;

        return Ok(());
    }

    pub fn visit_print_statement(&mut self, stmt: &PrintStatement) -> Result<(), ErrorType> {
        let expression = stmt.expression.accept_visitor(self)?;

        let string_literal = match expression {
            Data::String(s) => s,
            _ => return Err(ErrorType::SyntaxError),
        };

        //let formatted_string_literal = self.string_formatter.format(&string_literal, &self.env)?;

        println!("{}", string_literal);

        return Ok(());
    }

    pub fn visit_variable_decl_statement(
        &mut self,
        stmt: &VariableDeclStatement,
    ) -> Result<(), ErrorType> {
        let data_type = &stmt.data_type;
        let data = stmt.data.accept_visitor(self)?;

        match (&data, data_type) {
            (Data::Float(_), TokenType::Float) => (),
            (Data::Boolean(_), TokenType::Boolean) => (),
            (Data::String(_), TokenType::String) => (),
            _ => return Err(ErrorType::TypeError),
        };

        let identifier = stmt.identifier.clone();

        self.env.create_symbol(identifier, data)?;

        return Ok(());
    }

    pub fn visit_assign_operator(&mut self, node: &AssignOperator) -> Result<Data, ErrorType> {
        let identifier = &node.identifier.value;
        let data = node.right.accept_visitor(self)?;

        self.env.update_symbol(identifier, data.clone())?;

        return Ok(data);
    }

    pub fn visit_binary_operator(&mut self, node: &BinaryOperator) -> Result<Data, ErrorType> {
        let left = node.left.accept_visitor(self)?;
        let right = node.right.accept_visitor(self)?;
        let ty = &node.ty;

        use {Data as D, TokenType as T};

        match (ty, left, right) {
            (T::Plus, D::Float(l), D::Float(r)) => Ok(D::Float(l + r)),
            (T::Minus, D::Float(l), D::Float(r)) => Ok(D::Float(l - r)),
            (T::Multiply, D::Float(l), D::Float(r)) => Ok(D::Float(l * r)),
            (T::Divide, D::Float(l), D::Float(r)) => Ok(D::Float(l / r)),
            (T::Remainder, D::Float(l), D::Float(r)) => Ok(D::Float(l % r)),

            (T::And, D::Boolean(l), D::Boolean(r)) => Ok(D::Boolean(l && r)),
            (T::Or, D::Boolean(l), D::Boolean(r)) => Ok(D::Boolean(l || r)),

            (T::Equal, D::Float(l), D::Float(r)) => Ok(D::Boolean(l == r)),
            (T::NotEqual, D::Float(l), D::Float(r)) => Ok(D::Boolean(l != r)),
            (T::Greater, D::Float(l), D::Float(r)) => Ok(D::Boolean(l > r)),
            (T::GreaterEqual, D::Float(l), D::Float(r)) => Ok(D::Boolean(l >= r)),
            (T::Less, D::Float(l), D::Float(r)) => Ok(D::Boolean(l < r)),
            (T::LessEqual, D::Float(l), D::Float(r)) => Ok(D::Boolean(l <= r)),
            _ => Err(ErrorType::TypeError),
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
            (TokenType::Minus, Data::Float(r)) => Ok(Data::Float(-r)),
            (TokenType::Not, Data::Boolean(r)) => Ok(Data::Boolean(!r)),
            _ => Err(ErrorType::TypeError),
        }
    }
}
