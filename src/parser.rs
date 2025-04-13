use crate::{
    interpreter::{
        expr::{BinaryOperator, Expr, Identifier, Literal, UnaryOperator},
        stmt::{PrintStmt, Stmt, VariableDeclStmt},
    },
    token::{DataType, Token, TokenType},
    yf_error::{ErrorType, YFError},
};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    line: u32,
    errors: Vec<YFError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            line: 1,
            errors: Vec::new(),
        }
    }

    fn look_ahead(&mut self) -> Option<Token> {
        if let Some(token) = self.tokens.get(self.pos) {
            self.line = token.line;
            return Some(token.clone());
        }

        return None;
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn consume(&mut self, expected: &TokenType) -> Result<(), ErrorType> {
        let Some(token) = self.look_ahead() else {
            return Err(ErrorType::EndOfFile);
        };

        self.advance();

        if token.ty == *expected {
            return Ok(());
        } else {
            return Err(ErrorType::UnexpectedToken);
        }
    }

    fn parse_literal(&mut self) -> Result<Box<Expr>, ErrorType> {
        let Some(token) = self.look_ahead() else {
            return Err(ErrorType::EndOfFile);
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
            _ => Err(ErrorType::UnexpectedToken),
        }
    }

    fn parse_unary(&mut self) -> Result<Box<Expr>, ErrorType> {
        let Some(Token { ty, .. }) = self.look_ahead() else {
            return Err(ErrorType::EndOfFile);
        };

        match ty {
            TokenType::Plus => {
                self.consume(&TokenType::Plus)?;
                self.parse_unary()
            }
            TokenType::Minus | TokenType::Not => {
                self.consume(&ty)?;
                Ok(Box::new(Expr::UnaryOperator(UnaryOperator {
                    ty,
                    right: self.parse_unary()?,
                })))
            }
            _ => self.parse_literal(),
        }
    }

    fn parse_factor(&mut self) -> Result<Box<Expr>, ErrorType> {
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

    fn parse_term(&mut self) -> Result<Box<Expr>, ErrorType> {
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

    fn parse_comparison(&mut self) -> Result<Box<Expr>, ErrorType> {
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

    fn parse_equality(&mut self) -> Result<Box<Expr>, ErrorType> {
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

    fn parse_and(&mut self) -> Result<Box<Expr>, ErrorType> {
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

    fn parse_or(&mut self) -> Result<Box<Expr>, ErrorType> {
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

    fn parse_expression(&mut self) -> Result<Box<Expr>, ErrorType> {
        return self.parse_or();
    }

    fn parse_expression_stmt(&mut self) -> Result<Stmt, ErrorType> {
        let exp = self.parse_expression()?;
        self.consume(&TokenType::Semicolon)?;

        return Ok(Stmt::ExprStmt(exp));
    }

    fn parse_print_stmt(&mut self) -> Result<Stmt, ErrorType> {
        self.consume(&TokenType::Print)?;
        self.consume(&TokenType::LeftParen)?;
        let exp = self.parse_expression()?;
        self.consume(&TokenType::RightParen)?;
        self.consume(&TokenType::Semicolon)?;

        return Ok(Stmt::PrintStmt(PrintStmt::new(exp)));
    }

    fn parse_variable_stmt(&mut self, data_type: DataType) -> Result<Stmt, ErrorType> {
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

    fn parse_stmt(&mut self) -> Result<Stmt, ErrorType> {
        let token = self.look_ahead().unwrap();

        match token.ty {
            TokenType::VariableDecl(data) => self.parse_variable_stmt(data),
            TokenType::Print => self.parse_print_stmt(),
            _ => self.parse_expression_stmt(),
        }
    }

    fn recover_from_error(&mut self) {
        while let Some(token) = self.look_ahead() {
            let _ = self.consume(&token.ty);

            match token.ty {
                TokenType::Semicolon => break,
                _ => (),
            };
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, YFError> {
        self.pos = 0;

        let mut statements: Vec<Stmt> = Vec::new();

        while let Some(_) = self.look_ahead() {
            match self.parse_stmt() {
                Ok(statement) => statements.push(statement),
                Err(error_type) => {
                    self.errors.push(YFError {
                        error_type,
                        line: self.line,
                    });

                    self.recover_from_error();
                }
            }
        }

        println!("{:#?}", self.errors);

        return Ok(statements);
    }
}
