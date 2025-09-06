use crate::{error::kaori_error::KaoriError, frontend::lexer::token_kind::TokenKind, kaori_error};

use super::{
    expr::Expr,
    operator::{BinaryOp, UnaryOp},
    parser::Parser,
};

impl Parser {
    pub fn parse_expression(&mut self) -> Result<Expr, KaoriError> {
        if self
            .token_stream
            .look_ahead(&[TokenKind::Identifier, TokenKind::Assign])
        {
            return self.parse_assign();
        }

        self.parse_or()
    }

    pub fn parse_assign(&mut self) -> Result<Expr, KaoriError> {
        let left = self.parse_identifier()?;

        self.token_stream.consume(TokenKind::Assign)?;

        let right = self.parse_expression()?;

        Ok(Expr::assign(left, right))
    }

    pub fn parse_or(&mut self) -> Result<Expr, KaoriError> {
        let mut left = self.parse_and()?;

        while !self.token_stream.at_end() {
            let kind = self.token_stream.token_kind();

            let operator = match kind {
                TokenKind::Or => BinaryOp::Or,
                _ => break,
            };

            self.token_stream.advance();

            let right = self.parse_and()?;

            left = Expr::binary(operator, left, right);
        }

        Ok(left)
    }

    pub fn parse_and(&mut self) -> Result<Expr, KaoriError> {
        let mut left = self.parse_equality()?;

        while !self.token_stream.at_end() {
            let kind = self.token_stream.token_kind();

            let operator = match kind {
                TokenKind::And => BinaryOp::And,
                _ => break,
            };

            self.token_stream.advance();
            let right = self.parse_equality()?;

            left = Expr::binary(operator, left, right);
        }

        Ok(left)
    }

    pub fn parse_equality(&mut self) -> Result<Expr, KaoriError> {
        let mut left = self.parse_comparison()?;

        while !self.token_stream.at_end() {
            let kind = self.token_stream.token_kind();

            let operator = match kind {
                TokenKind::Equal => BinaryOp::Equal,
                TokenKind::NotEqual => BinaryOp::NotEqual,
                _ => break,
            };

            self.token_stream.advance();
            let right = self.parse_comparison()?;

            left = Expr::binary(operator, left, right);
        }

        Ok(left)
    }

    pub fn parse_comparison(&mut self) -> Result<Expr, KaoriError> {
        let mut left = self.parse_term()?;

        while !self.token_stream.at_end() {
            let kind = self.token_stream.token_kind();

            let operator = match kind {
                TokenKind::Greater => BinaryOp::Greater,
                TokenKind::GreaterEqual => BinaryOp::GreaterEqual,
                TokenKind::Less => BinaryOp::Less,
                TokenKind::LessEqual => BinaryOp::LessEqual,
                _ => break,
            };

            self.token_stream.advance();
            let right = self.parse_term()?;

            left = Expr::binary(operator, left, right);
        }

        Ok(left)
    }

    pub fn parse_term(&mut self) -> Result<Expr, KaoriError> {
        let mut left = self.parse_factor()?;

        while !self.token_stream.at_end() {
            let kind = self.token_stream.token_kind();

            let operator = match kind {
                TokenKind::Plus => BinaryOp::Add,
                TokenKind::Minus => BinaryOp::Subtract,
                _ => break,
            };

            self.token_stream.advance();
            let right = self.parse_factor()?;

            left = Expr::binary(operator, left, right);
        }

        Ok(left)
    }

    pub fn parse_factor(&mut self) -> Result<Expr, KaoriError> {
        let mut left = self.parse_prefix_unary()?;

        while !self.token_stream.at_end() {
            let kind = self.token_stream.token_kind();

            let operator = match kind {
                TokenKind::Multiply => BinaryOp::Multiply,
                TokenKind::Divide => BinaryOp::Divide,
                TokenKind::Modulo => BinaryOp::Modulo,
                _ => break,
            };

            self.token_stream.advance();
            let right = self.parse_prefix_unary()?;

            left = Expr::binary(operator, left, right);
        }

        Ok(left)
    }

    pub fn parse_prefix_unary(&mut self) -> Result<Expr, KaoriError> {
        let kind = self.token_stream.token_kind();
        let span = self.token_stream.span();

        let operator = match kind {
            TokenKind::Plus => {
                self.token_stream.advance();
                return self.parse_prefix_unary();
            }
            TokenKind::Minus => UnaryOp::Negate,
            TokenKind::Not => UnaryOp::Not,
            _ => return self.parse_primary(),
        };

        self.token_stream.advance();

        let right = self.parse_prefix_unary()?;

        Ok(Expr::unary(operator, right, span))
    }

    pub fn parse_primary(&mut self) -> Result<Expr, KaoriError> {
        let kind = self.token_stream.token_kind();
        let span = self.token_stream.span();

        Ok(match kind {
            TokenKind::LeftParen => {
                self.token_stream.consume(TokenKind::LeftParen)?;
                let expr = self.parse_expression()?;
                self.token_stream.consume(TokenKind::RightParen)?;
                expr
            }
            TokenKind::NumberLiteral => {
                let lexeme = self.token_stream.lexeme();
                let value = lexeme.parse::<f64>().unwrap();

                self.token_stream.advance();
                Expr::number_literal(value, span)
            }
            TokenKind::BooleanLiteral => {
                let lexeme = self.token_stream.lexeme();
                let value = lexeme.parse::<bool>().unwrap();

                self.token_stream.advance();
                Expr::boolean_literal(value, span)
            }
            TokenKind::StringLiteral => {
                let value = self.token_stream.lexeme().to_owned();

                self.token_stream.advance();
                Expr::string_literal(value, span)
            }
            TokenKind::Identifier => self.parse_postfix_unary()?,
            _ => {
                let span = self.token_stream.span();

                return Err(kaori_error!(
                    span,
                    "expected a valid operand, but found: {}",
                    kind
                ));
            }
        })
    }

    pub fn parse_identifier(&mut self) -> Result<Expr, KaoriError> {
        let name = self.token_stream.lexeme().to_owned();
        let span = self.token_stream.span();

        let identifier = Expr::identifier(name, span);

        self.token_stream.consume(TokenKind::Identifier)?;

        Ok(identifier)
    }

    pub fn parse_postfix_unary(&mut self) -> Result<Expr, KaoriError> {
        let identifier = self.parse_identifier()?;

        let kind = self.token_stream.token_kind();
        let span = self.token_stream.span();

        let operator = match kind {
            TokenKind::Increment => UnaryOp::Increment,
            TokenKind::Decrement => UnaryOp::Decrement,
            TokenKind::LeftParen => return self.parse_function_call(identifier),
            _ => return Ok(identifier),
        };

        self.token_stream.advance();

        Ok(Expr::unary(operator, identifier, span))
    }

    pub fn parse_function_call(&mut self, callee: Expr) -> Result<Expr, KaoriError> {
        if self.token_stream.token_kind() != TokenKind::LeftParen {
            return Ok(callee);
        }

        self.token_stream.consume(TokenKind::LeftParen)?;

        let mut arguments: Vec<Expr> = Vec::new();

        while !self.token_stream.at_end() && self.token_stream.token_kind() != TokenKind::RightParen
        {
            let argument = self.parse_expression()?;

            arguments.push(argument);

            if self.token_stream.token_kind() == TokenKind::RightParen {
                break;
            }

            self.token_stream.consume(TokenKind::Comma)?;
        }

        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::RightParen)?;

        let callee = Expr::function_call(callee, arguments, span);

        self.parse_function_call(callee)
    }
}
