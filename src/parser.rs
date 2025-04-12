use crate::{
    interpreter::{
        expr::{BinaryOperator, Expr, Identifier, Literal, UnaryOperator},
        stmt::{PrintStmt, Stmt, VariableDeclStmt},
    },
    token::{DataType, Token, TokenType},
};

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken {
        line: u32,
        expected: TokenType,
        found: TokenType,
    },

    EndOfFile {
        line: u32,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    line: u32,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            line: 1,
        }
    }

    fn look_ahead(&mut self) -> Option<Token> {
        return self.tokens.get(self.pos).cloned();
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn consume(&mut self, expected: &TokenType) -> Result<(), ParserError> {
        let Some(token) = self.look_ahead() else {
            return Err(ParserError::EndOfFile { line: self.line });
        };

        if token.ty == *expected {
            self.line = token.line;
            self.advance();
            return Ok(());
        } else {
            return Err(ParserError::UnexpectedToken {
                line: self.line,
                found: token.ty,
                expected: expected.clone(),
            });
        }
    }

    fn parse_literal(&mut self) -> Result<Box<Expr>, ParserError> {
        let Some(token) = self.look_ahead() else {
            return Err(ParserError::EndOfFile { line: self.line });
        };

        match token.ty {
            TokenType::LeftParen => {
                self.consume(&TokenType::LeftParen)?;
                let expression = self.parse_expression()?;
                self.consume(&TokenType::RightParen)?;
                Ok(expression)
            }
            TokenType::Literal(data_type) => {
                self.consume(&TokenType::Literal(data_type.clone()))?;
                Ok(Box::new(Expr::Literal(Literal {
                    ty: data_type,
                    value: token.value,
                })))
            }
            TokenType::Identifier => {
                self.consume(&TokenType::Identifier)?;
                Ok(Box::new(Expr::Identifier(Identifier {
                    ty: token.ty,
                    value: token.value,
                })))
            }
            _ => Err(ParserError::UnexpectedToken {
                line: self.line,
                expected: TokenType::Literal(DataType::Any),
                found: token.ty,
            }),
        }
    }

    fn parse_unary(&mut self) -> Result<Box<Expr>, ParserError> {
        let Some(token) = self.look_ahead() else {
            return Err(ParserError::EndOfFile { line: self.line });
        };

        if token.ty == TokenType::Plus {
            self.consume(&token.ty)?;
            return self.parse_unary();
        }

        if !matches!(token.ty, TokenType::Minus | TokenType::Not) {
            return self.parse_literal();
        }

        self.consume(&token.ty)?;

        return Ok(Box::new(Expr::UnaryOperator(UnaryOperator {
            ty: token.ty,
            right: self.parse_unary()?,
        })));
    }

    fn parse_factor(&mut self) -> Result<Box<Expr>, ParserError> {
        let mut left = self.parse_unary()?;

        while let Some(token) = self.look_ahead() {
            if !matches!(token.ty, TokenType::Multiply | TokenType::Divide) {
                break;
            }

            self.consume(&token.ty)?;
            let right = self.parse_unary()?;

            left = Box::new(Expr::BinaryOperator(BinaryOperator {
                ty: token.ty,
                left,
                right,
            }));
        }

        return Ok(left);
    }

    fn parse_term(&mut self) -> Result<Box<Expr>, ParserError> {
        let mut left = self.parse_factor()?;

        while let Some(token) = self.look_ahead() {
            if !matches!(token.ty, TokenType::Plus | TokenType::Minus) {
                break;
            }

            self.consume(&token.ty)?;
            let right = self.parse_factor()?;

            left = Box::new(Expr::BinaryOperator(BinaryOperator {
                ty: token.ty,
                left,
                right,
            }));
        }

        return Ok(left);
    }

    fn parse_comparison(&mut self) -> Result<Box<Expr>, ParserError> {
        let mut left = self.parse_term()?;

        while let Some(token) = self.look_ahead() {
            if !matches!(
                token.ty,
                TokenType::Greater
                    | TokenType::GreaterEqual
                    | TokenType::Less
                    | TokenType::LessEqual
            ) {
                break;
            }

            self.consume(&token.ty)?;
            let right = self.parse_term()?;

            left = Box::new(Expr::BinaryOperator(BinaryOperator {
                ty: token.ty,
                left,
                right,
            }));
        }

        return Ok(left);
    }

    fn parse_equality(&mut self) -> Result<Box<Expr>, ParserError> {
        let mut left = self.parse_comparison()?;

        while let Some(token) = self.look_ahead() {
            if !matches!(token.ty, TokenType::Equal | TokenType::NotEqual) {
                break;
            }

            self.consume(&token.ty)?;
            let right = self.parse_comparison()?;

            left = Box::new(Expr::BinaryOperator(BinaryOperator {
                ty: token.ty,
                left,
                right,
            }));
        }

        return Ok(left);
    }

    fn parse_and(&mut self) -> Result<Box<Expr>, ParserError> {
        let mut left = self.parse_equality()?;

        while let Some(token) = self.look_ahead() {
            if token.ty != TokenType::And {
                break;
            }

            self.consume(&TokenType::And)?;

            let right = self.parse_equality()?;

            left = Box::new(Expr::BinaryOperator(BinaryOperator {
                ty: token.ty,
                left,
                right,
            }));
        }

        return Ok(left);
    }

    fn parse_or(&mut self) -> Result<Box<Expr>, ParserError> {
        let mut left = self.parse_and()?;

        while let Some(token) = self.look_ahead() {
            if token.ty != TokenType::Or {
                break;
            }

            self.consume(&TokenType::Or)?;

            let right = self.parse_and()?;

            left = Box::new(Expr::BinaryOperator(BinaryOperator {
                ty: token.ty,
                left,
                right,
            }));
        }

        return Ok(left);
    }

    fn parse_expression(&mut self) -> Result<Box<Expr>, ParserError> {
        return self.parse_or();
    }

    fn parse_expression_stmt(&mut self) -> Result<Stmt, ParserError> {
        let exp = self.parse_expression()?;
        self.consume(&TokenType::Semicolon)?;

        return Ok(Stmt::ExprStmt(exp));
    }

    fn parse_print_stmt(&mut self) -> Result<Stmt, ParserError> {
        self.consume(&TokenType::Print)?;
        self.consume(&TokenType::LeftParen)?;
        let exp = self.parse_expression()?;
        self.consume(&TokenType::RightParen)?;
        self.consume(&TokenType::Semicolon)?;

        return Ok(Stmt::PrintStmt(PrintStmt::new(exp)));
    }

    fn parse_variable_stmt(&mut self, data_type: DataType) -> Result<Stmt, ParserError> {
        self.consume(&TokenType::VariableDecl(data_type.clone()))?;
        let identifier = self.look_ahead().unwrap();

        self.consume(&TokenType::Identifier)?;
        self.consume(&TokenType::Assign)?;

        let exp = self.parse_expression()?;

        self.consume(&TokenType::Semicolon)?;

        return Ok(Stmt::VariableDeclStmt(VariableDeclStmt::new(
            data_type,
            identifier.value,
            exp,
        )));
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParserError> {
        let token = self.look_ahead().unwrap();

        match token.ty {
            TokenType::VariableDecl(data) => self.parse_variable_stmt(data),
            TokenType::Print => self.parse_print_stmt(),
            _ => self.parse_expression_stmt(),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParserError> {
        self.pos = 0;

        let mut statements: Vec<Stmt> = Vec::new();

        while let Some(token) = self.look_ahead() {
            if token.ty == TokenType::EndOfFile {
                break;
            }

            let statement = self.parse_stmt()?;
            statements.push(statement);
        }

        return Ok(statements);
    }
}
