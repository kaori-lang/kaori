use crate::{
    interpreter::expr::Expr,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken { line: u32 },
    EndOfFile { line: u32 },
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
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
            return Err(ParserError::UnexpectedToken { line: 0 });
        };

        if token.ty == *expected {
            self.advance();
            return Ok(());
        } else {
            return Err(ParserError::UnexpectedToken { line: token.line });
        }
    }

    fn parse_literal(&mut self) -> Result<Box<Expr>, ParserError> {
        let Some(token) = self.look_ahead() else {
            return Err(ParserError::EndOfFile { line: 0 });
        };

        match token.ty {
            TokenType::LeftParen => {
                self.consume(&TokenType::LeftParen)?;
                let expression = self.parse_expression()?;
                self.consume(&TokenType::RightParen)?;
                Ok(expression)
            }
            TokenType::Number => {
                self.consume(&TokenType::Number)?;
                Ok(Box::new(Expr::Literal {
                    ty: token.ty,
                    value: token.value,
                }))
            }
            _ => Err(ParserError::UnexpectedToken { line: token.line }),
        }
    }

    fn parse_unary(&mut self) -> Result<Box<Expr>, ParserError> {
        let Some(token) = self.look_ahead() else {
            return Err(ParserError::UnexpectedToken { line: 0 });
        };

        if token.ty == TokenType::Plus {
            self.consume(&token.ty)?;
            return self.parse_unary();
        }

        if !matches!(token.ty, TokenType::Minus | TokenType::Not) {
            return self.parse_literal();
        }

        self.consume(&token.ty)?;

        return Ok(Box::new(Expr::UnaryOperator {
            ty: token.ty,
            right: self.parse_unary()?,
        }));
    }

    fn parse_factor(&mut self) -> Result<Box<Expr>, ParserError> {
        let mut left = self.parse_unary()?;

        while let Some(token) = self.look_ahead() {
            if !matches!(token.ty, TokenType::Multiply | TokenType::Divide) {
                break;
            }

            self.consume(&token.ty)?;
            let right = self.parse_unary()?;

            left = Box::new(Expr::BinaryOperator {
                ty: token.ty,
                left,
                right,
            });
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

            left = Box::new(Expr::BinaryOperator {
                ty: token.ty,
                left,
                right,
            });
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

            left = Box::new(Expr::BinaryOperator {
                ty: token.ty,
                left,
                right,
            });
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

            left = Box::new(Expr::BinaryOperator {
                ty: token.ty,
                left,
                right,
            });
        }

        return Ok(left);
    }

    fn parse_expression(&mut self) -> Result<Box<Expr>, ParserError> {
        return self.parse_term();
    }

    pub fn get_ast(&mut self) -> Result<Box<Expr>, ParserError> {
        let ast = self.parse_equality();
        self.pos = 0;

        return ast;
    }
}
