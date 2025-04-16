use crate::{
    ast::{
        expression::{
            AssignOperator, BinaryOperator, Expression, Identifier, Literal, UnaryOperator,
        },
        statement::{
            self, BlockStatement, ExpressionStatement, IfStatement, PrintStatement, Statement,
            VariableDeclStatement, WhileStatement,
        },
    },
    lexer::token::{Token, TokenType},
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
            return Err(ErrorType::SyntaxError);
        };

        self.advance();

        if token.ty == *expected {
            self.line = token.line;
            return Ok(());
        } else {
            return Err(ErrorType::SyntaxError);
        }
    }

    fn parse_literal(&mut self) -> Result<Box<dyn Expression>, ErrorType> {
        let Some(token) = self.look_ahead() else {
            return Err(ErrorType::SyntaxError);
        };

        match token.ty {
            TokenType::LeftParen => {
                self.consume(&TokenType::LeftParen)?;
                let expression = self.parse_expression()?;
                self.consume(&TokenType::RightParen)?;
                Ok(expression)
            }
            TokenType::Literal => {
                self.consume(&TokenType::Literal)?;
                Ok(Box::new(Literal {
                    value: token.literal,
                }))
            }
            TokenType::Identifier => {
                self.consume(&TokenType::Identifier)?;
                Ok(Box::new(Identifier {
                    value: token.lexeme,
                }))
            }
            _ => Err(ErrorType::SyntaxError),
        }
    }

    fn parse_unary(&mut self) -> Result<Box<dyn Expression>, ErrorType> {
        let Some(Token { ty, .. }) = self.look_ahead() else {
            return Err(ErrorType::SyntaxError);
        };

        match ty {
            TokenType::Plus => {
                self.consume(&TokenType::Plus)?;
                self.parse_unary()
            }
            TokenType::Minus | TokenType::Not => {
                self.consume(&ty)?;
                Ok(Box::new(UnaryOperator {
                    ty,
                    right: self.parse_unary()?,
                }))
            }
            _ => self.parse_literal(),
        }
    }

    fn parse_factor(&mut self) -> Result<Box<dyn Expression>, ErrorType> {
        let mut left = self.parse_unary()?;

        while let Some(token) = self.look_ahead() {
            if !matches!(
                token.ty,
                TokenType::Multiply | TokenType::Divide | TokenType::Remainder
            ) {
                break;
            }

            self.consume(&token.ty)?;
            let right = self.parse_unary()?;

            left = Box::new(BinaryOperator {
                ty: token.ty,
                left,
                right,
            });
        }

        return Ok(left);
    }

    fn parse_term(&mut self) -> Result<Box<dyn Expression>, ErrorType> {
        let mut left = self.parse_factor()?;

        while let Some(token) = self.look_ahead() {
            if !matches!(token.ty, TokenType::Plus | TokenType::Minus) {
                break;
            }

            self.consume(&token.ty)?;
            let right = self.parse_factor()?;

            left = Box::new(BinaryOperator {
                ty: token.ty,
                left,
                right,
            });
        }

        return Ok(left);
    }

    fn parse_comparison(&mut self) -> Result<Box<dyn Expression>, ErrorType> {
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

            left = Box::new(BinaryOperator {
                ty: token.ty,
                left,
                right,
            });
        }

        return Ok(left);
    }

    fn parse_equality(&mut self) -> Result<Box<dyn Expression>, ErrorType> {
        let mut left = self.parse_comparison()?;

        while let Some(token) = self.look_ahead() {
            if !matches!(token.ty, TokenType::Equal | TokenType::NotEqual) {
                break;
            }

            self.consume(&token.ty)?;
            let right = self.parse_comparison()?;

            left = Box::new(BinaryOperator {
                ty: token.ty,
                left,
                right,
            });
        }

        return Ok(left);
    }

    fn parse_and(&mut self) -> Result<Box<dyn Expression>, ErrorType> {
        let mut left = self.parse_equality()?;

        while let Some(token) = self.look_ahead() {
            if token.ty != TokenType::And {
                break;
            }

            self.consume(&TokenType::And)?;

            let right = self.parse_equality()?;

            left = Box::new(BinaryOperator {
                ty: token.ty,
                left,
                right,
            });
        }

        return Ok(left);
    }

    fn parse_or(&mut self) -> Result<Box<dyn Expression>, ErrorType> {
        let mut left = self.parse_and()?;

        while let Some(token) = self.look_ahead() {
            if token.ty != TokenType::Or {
                break;
            }

            self.consume(&TokenType::Or)?;

            let right = self.parse_and()?;

            left = Box::new(BinaryOperator {
                ty: token.ty,
                left,
                right,
            });
        }

        return Ok(left);
    }

    fn parse_assign(&mut self) -> Result<Box<dyn Expression>, ErrorType> {
        let expression = self.parse_or()?;

        match expression.as_any().downcast_ref::<Identifier>() {
            Some(identifier) => {
                let Some(Token {
                    ty: TokenType::Assign,
                    ..
                }) = self.look_ahead()
                else {
                    return Ok(expression);
                };

                self.consume(&TokenType::Assign)?;
                Ok(Box::new(AssignOperator {
                    identifier: identifier.clone(),
                    right: self.parse_assign()?,
                }))
            }
            _ => Ok(expression),
        }
    }

    fn parse_expression(&mut self) -> Result<Box<dyn Expression>, ErrorType> {
        return self.parse_assign();
    }

    fn parse_expression_statement(&mut self) -> Result<Box<dyn Statement>, ErrorType> {
        let expression = self.parse_expression()?;
        self.consume(&TokenType::Semicolon)?;

        return Ok(Box::new(ExpressionStatement {
            expression,
            line: self.line,
        }));
    }

    fn parse_print_statement(&mut self) -> Result<Box<dyn Statement>, ErrorType> {
        self.consume(&TokenType::Print)?;
        self.consume(&TokenType::LeftParen)?;
        let expression = self.parse_expression()?;
        self.consume(&TokenType::RightParen)?;
        self.consume(&TokenType::Semicolon)?;

        return Ok(Box::new(PrintStatement {
            expression,
            line: self.line,
        }));
    }

    fn parse_variable_statement(
        &mut self,
        data_type: TokenType,
    ) -> Result<Box<dyn Statement>, ErrorType> {
        self.consume(&data_type)?;
        let identifier = self.look_ahead().unwrap();

        self.consume(&TokenType::Identifier)?;
        self.consume(&TokenType::Assign)?;

        let data = self.parse_expression()?;

        self.consume(&TokenType::Semicolon)?;

        return Ok(Box::new(VariableDeclStatement {
            data_type,
            identifier: identifier.lexeme,
            data,
            line: self.line,
        }));
    }

    fn parse_if_statement(&mut self) -> Result<Box<dyn Statement>, ErrorType> {
        self.consume(&TokenType::If)?;
        self.consume(&TokenType::LeftParen)?;
        let condition = self.parse_expression()?;
        self.consume(&TokenType::RightParen)?;

        let then_branch = self.parse_block_statement()?;

        let mut if_statement = IfStatement {
            condition,
            then_branch,
            else_branch: None,
            line: self.line,
        };

        let Some(Token {
            ty: TokenType::Else,
            ..
        }) = self.look_ahead()
        else {
            return Ok(Box::new(if_statement));
        };

        self.consume(&TokenType::Else)?;

        let Some(token) = self.look_ahead() else {
            return Err(ErrorType::SyntaxError);
        };

        if_statement.else_branch = match token.ty {
            TokenType::LeftBrace => Some(self.parse_block_statement()?),
            TokenType::If => Some(self.parse_if_statement()?),
            _ => return Err(ErrorType::SyntaxError),
        };

        return Ok(Box::new(if_statement));
    }

    fn parse_while_statement(&mut self) -> Result<Box<dyn Statement>, ErrorType> {
        self.consume(&TokenType::While)?;
        self.consume(&TokenType::LeftParen)?;
        let condition = self.parse_expression()?;
        self.consume(&TokenType::RightParen)?;

        let block = self.parse_block_statement()?;

        return Ok(Box::new(WhileStatement {
            condition,
            block,
            line: self.line,
        }));
    }

    fn parse_block_statement(&mut self) -> Result<Box<dyn Statement>, ErrorType> {
        let mut statements: Vec<Box<dyn Statement>> = Vec::new();
        self.consume(&TokenType::LeftBrace)?;

        while let Some(token) = self.look_ahead() {
            if token.ty == TokenType::RightBrace {
                break;
            }

            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        self.consume(&TokenType::RightBrace)?;

        return Ok(Box::new(BlockStatement {
            statements,
            line: self.line,
        }));
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, ErrorType> {
        let token = self.look_ahead().unwrap();

        match token.ty {
            TokenType::Float | TokenType::Boolean | TokenType::String => {
                self.parse_variable_statement(token.ty)
            }
            TokenType::Print => self.parse_print_statement(),
            TokenType::LeftBrace => self.parse_block_statement(),
            TokenType::If => self.parse_if_statement(),
            TokenType::While => self.parse_while_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_statements(&mut self) -> Result<Vec<Box<dyn Statement>>, ErrorType> {
        let mut statements: Vec<Box<dyn Statement>> = Vec::new();

        while let Some(_) = self.look_ahead() {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        return Ok(statements);
    }

    /*
    fn recover_from_error(&mut self) {
        while let Some(token) = self.look_ahead() {
            let _ = self.consume(&token.ty);

            match token.ty {
                TokenType::Semicolon => break,
                _ => (),
            };
        }
    } */

    pub fn execute(&mut self) -> Result<Vec<Box<dyn Statement>>, YFError> {
        let statements = match self.parse_statements() {
            Ok(statements) => Ok(statements),
            Err(error_type) => {
                let error = YFError {
                    error_type,
                    line: self.line,
                };

                Err(error)
            }
        };

        self.pos = 0;

        return statements;
    }
}
