use crate::{error::kaori_error::KaoriError, kaori_error, lexer::token_kind::TokenKind};

use super::{
    assign_op::{AssignOp, AssignOpKind},
    binary_op::{BinaryOp, BinaryOpKind},
    expr::Expr,
    parser::Parser,
    unary_op::{UnaryOp, UnaryOpKind},
};

impl Parser {
    fn build_binary_operator(&mut self) -> BinaryOp {
        let token_kind = self.token_stream.token_kind();
        let span = self.token_stream.span();

        let kind = match token_kind {
            TokenKind::Plus => BinaryOpKind::Add,
            TokenKind::Minus => BinaryOpKind::Subtract,
            TokenKind::Multiply => BinaryOpKind::Multiply,
            TokenKind::Divide => BinaryOpKind::Divide,
            TokenKind::Modulo => BinaryOpKind::Modulo,
            TokenKind::And => BinaryOpKind::And,
            TokenKind::Or => BinaryOpKind::Or,
            TokenKind::Equal => BinaryOpKind::Equal,
            TokenKind::NotEqual => BinaryOpKind::NotEqual,
            TokenKind::Greater => BinaryOpKind::Greater,
            TokenKind::GreaterEqual => BinaryOpKind::GreaterEqual,
            TokenKind::Less => BinaryOpKind::Less,
            TokenKind::LessEqual => BinaryOpKind::LessEqual,
            _ => unreachable!(),
        };

        BinaryOp::new(kind, span)
    }

    fn build_unary_operator(&mut self) -> UnaryOp {
        let token_kind = self.token_stream.token_kind();
        let span = self.token_stream.span();

        let kind = match token_kind {
            TokenKind::Minus => UnaryOpKind::Negate,
            TokenKind::Not => UnaryOpKind::Not,
            _ => unreachable!(),
        };

        UnaryOp::new(kind, span)
    }

    pub fn parse_expression(&mut self) -> Result<Expr, KaoriError> {
        self.parse_assign()
    }

    fn parse_assign(&mut self) -> Result<Expr, KaoriError> {
        let left = self.parse_or()?;

        let kind = self.token_stream.token_kind();
        let span = self.token_stream.span();

        let operator = match kind {
            TokenKind::Assign => AssignOpKind::Assign,
            TokenKind::AddAssign => AssignOpKind::AddAssign,
            TokenKind::SubtractAssign => AssignOpKind::SubtractAssign,
            TokenKind::MultiplyAssign => AssignOpKind::MultiplyAssign,
            TokenKind::DivideAssign => AssignOpKind::DivideAssign,
            TokenKind::ModuloAssign => AssignOpKind::ModuloAssign,
            _ => return Ok(left),
        };

        let operator = AssignOp::new(operator, span);

        self.token_stream.advance();

        let right = self.parse_or()?;

        Ok(Expr::assign(operator, left, right))
    }

    fn parse_or(&mut self) -> Result<Expr, KaoriError> {
        let mut left = self.parse_and()?;

        while !self.token_stream.at_end() {
            let kind = self.token_stream.token_kind();

            let operator = match kind {
                TokenKind::Or => self.build_binary_operator(),
                _ => break,
            };

            self.token_stream.advance();

            let right = self.parse_and()?;

            left = Expr::binary(operator, left, right);
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, KaoriError> {
        let mut left = self.parse_equality()?;

        while !self.token_stream.at_end() {
            let kind = self.token_stream.token_kind();

            let operator = match kind {
                TokenKind::And => self.build_binary_operator(),
                _ => break,
            };

            self.token_stream.advance();
            let right = self.parse_equality()?;

            left = Expr::binary(operator, left, right);
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr, KaoriError> {
        let mut left = self.parse_comparison()?;

        while !self.token_stream.at_end() {
            let kind = self.token_stream.token_kind();

            let operator = match kind {
                TokenKind::Equal | TokenKind::NotEqual => self.build_binary_operator(),
                _ => break,
            };

            self.token_stream.advance();
            let right = self.parse_comparison()?;

            left = Expr::binary(operator, left, right);
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, KaoriError> {
        let mut left = self.parse_term()?;

        while !self.token_stream.at_end() {
            let kind = self.token_stream.token_kind();

            let operator = match kind {
                TokenKind::Greater
                | TokenKind::GreaterEqual
                | TokenKind::Less
                | TokenKind::LessEqual => self.build_binary_operator(),
                _ => break,
            };

            self.token_stream.advance();
            let right = self.parse_term()?;

            left = Expr::binary(operator, left, right);
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, KaoriError> {
        let mut left = self.parse_factor()?;

        while !self.token_stream.at_end() {
            let kind = self.token_stream.token_kind();

            let operator = match kind {
                TokenKind::Plus | TokenKind::Minus => self.build_binary_operator(),
                _ => break,
            };

            self.token_stream.advance();
            let right = self.parse_factor()?;

            left = Expr::binary(operator, left, right);
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, KaoriError> {
        let mut left = self.parse_prefix_unary()?;

        while !self.token_stream.at_end() {
            let kind = self.token_stream.token_kind();

            let operator = match kind {
                TokenKind::Multiply | TokenKind::Divide | TokenKind::Modulo => {
                    self.build_binary_operator()
                }
                _ => break,
            };

            self.token_stream.advance();
            let right = self.parse_prefix_unary()?;

            left = Expr::binary(operator, left, right);
        }

        Ok(left)
    }

    fn parse_prefix_unary(&mut self) -> Result<Expr, KaoriError> {
        let kind = self.token_stream.token_kind();

        let operator = match kind {
            TokenKind::Plus => {
                self.token_stream.advance();
                return self.parse_prefix_unary();
            }
            TokenKind::Not => {
                let operator = self.build_unary_operator();
                self.token_stream.advance();

                let right = self.parse_or()?;

                return Ok(Expr::unary(operator, right));
            }
            TokenKind::Minus => self.build_unary_operator(),
            _ => return self.parse_primary(),
        };

        self.token_stream.advance();

        let right = self.parse_prefix_unary()?;

        Ok(Expr::unary(operator, right))
    }

    fn parse_primary(&mut self) -> Result<Expr, KaoriError> {
        let kind = self.token_stream.token_kind();
        let span = self.token_stream.span();

        let primary = match kind {
            TokenKind::LeftParen => {
                self.token_stream.consume(TokenKind::LeftParen)?;
                let expr = self.parse_expression()?;
                self.token_stream.consume(TokenKind::RightParen)?;
                expr
            }
            TokenKind::NumberLiteral => {
                let value = match self.token_stream.lexeme().parse::<f64>() {
                    Ok(value) => Ok(value),
                    Err(..) => Err(kaori_error!(span, "expected a valid float to be parsed")),
                }?;

                self.token_stream.advance();

                Expr::number_literal(value, span)
            }
            TokenKind::True => {
                self.token_stream.advance();

                Expr::boolean_literal(true, span)
            }
            TokenKind::False => {
                self.token_stream.advance();

                Expr::boolean_literal(false, span)
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
        };

        Ok(primary)
    }

    fn parse_identifier(&mut self) -> Result<Expr, KaoriError> {
        let name = self.token_stream.lexeme().to_owned();
        let span = self.token_stream.span();

        let identifier = Expr::identifier(name, span);

        self.token_stream.consume(TokenKind::Identifier)?;

        Ok(identifier)
    }

    fn parse_postfix_unary(&mut self) -> Result<Expr, KaoriError> {
        let identifier = self.parse_identifier()?;

        let kind = self.token_stream.token_kind();

        Ok(match kind {
            TokenKind::LeftParen => self.parse_function_call(identifier)?,
            _ => identifier,
        })
    }

    fn parse_function_call(&mut self, callee: Expr) -> Result<Expr, KaoriError> {
        if self.token_stream.token_kind() != TokenKind::LeftParen {
            return Ok(callee);
        }

        self.token_stream.consume(TokenKind::LeftParen)?;

        let arguments =
            self.parse_comma_separator(Parser::parse_expression, TokenKind::RightParen)?;

        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::RightParen)?;

        let callee = Expr::function_call(callee, arguments, span);

        self.parse_function_call(callee)
    }
}
