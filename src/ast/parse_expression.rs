use crate::{error::kaori_error::KaoriError, kaori_error, lexer::token_kind::TokenKind};

use super::{
    assign_op::{AssignOp, AssignOpKind},
    binary_op::{BinaryOp, BinaryOpKind},
    expr::Expr,
    parser::Parser,
    unary_op::{UnaryOp, UnaryOpKind},
};

impl<'a> Parser<'a> {
    fn build_binary_operator(&mut self) -> BinaryOp {
        let token_kind = self.token_stream.token_kind();

        let op_kind = match token_kind {
            TokenKind::Plus => BinaryOpKind::Add,
            TokenKind::Minus => BinaryOpKind::Subtract,
            TokenKind::Multiply => BinaryOpKind::Multiply,
            TokenKind::Divide => BinaryOpKind::Divide,
            TokenKind::Modulo => BinaryOpKind::Modulo,
            TokenKind::Equal => BinaryOpKind::Equal,
            TokenKind::NotEqual => BinaryOpKind::NotEqual,
            TokenKind::Greater => BinaryOpKind::Greater,
            TokenKind::GreaterEqual => BinaryOpKind::GreaterEqual,
            TokenKind::Less => BinaryOpKind::Less,
            TokenKind::LessEqual => BinaryOpKind::LessEqual,
            TokenKind::Power => BinaryOpKind::Power,
            _ => unreachable!(),
        };

        BinaryOp::new(op_kind)
    }

    fn build_unary_operator(&mut self) -> UnaryOp {
        let token_kind = self.token_stream.token_kind();

        let op_kind = match token_kind {
            TokenKind::Minus => UnaryOpKind::Negate,
            _ => unreachable!(),
        };

        UnaryOp::new(op_kind)
    }

    pub fn parse_expression(&mut self) -> Result<Expr, KaoriError> {
        let assign = self.parse_assign()?;

        Ok(assign)
    }

    fn parse_assign(&mut self) -> Result<Expr, KaoriError> {
        let left = self.parse_or()?;

        let token_kind = self.token_stream.token_kind();

        let operator = match token_kind {
            TokenKind::Assign => AssignOpKind::Assign,
            TokenKind::AddAssign => AssignOpKind::AddAssign,
            TokenKind::SubtractAssign => AssignOpKind::SubtractAssign,
            TokenKind::MultiplyAssign => AssignOpKind::MultiplyAssign,
            TokenKind::DivideAssign => AssignOpKind::DivideAssign,
            TokenKind::ModuloAssign => AssignOpKind::ModuloAssign,
            TokenKind::PowerAssign => AssignOpKind::PowerAssign,
            TokenKind::DeclareAssign => {
                self.token_stream.advance();

                let right = self.parse_or()?;

                return Ok(Expr::declare_assign(left, right));
            }
            _ => return Ok(left),
        };

        let operator = AssignOp::new(operator);

        self.token_stream.advance();

        let right = self.parse_or()?;

        Ok(Expr::assign(operator, left, right))
    }

    fn parse_or(&mut self) -> Result<Expr, KaoriError> {
        let mut left = self.parse_and()?;

        while !self.token_stream.at_end() {
            let token_kind = self.token_stream.token_kind();

            let TokenKind::Or = token_kind else {
                break;
            };

            self.token_stream.advance();

            let right = self.parse_and()?;

            left = Expr::logical_or(left, right);
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, KaoriError> {
        let mut left = self.parse_equality()?;

        while !self.token_stream.at_end() {
            let token_kind = self.token_stream.token_kind();

            let TokenKind::And = token_kind else {
                break;
            };

            self.token_stream.advance();
            let right = self.parse_equality()?;

            left = Expr::logical_and(left, right);
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr, KaoriError> {
        let mut left = self.parse_comparison()?;

        while !self.token_stream.at_end() {
            let token_kind = self.token_stream.token_kind();

            let operator = match token_kind {
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
            let token_kind = self.token_stream.token_kind();

            let operator = match token_kind {
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
            let token_kind = self.token_stream.token_kind();

            let operator = match token_kind {
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
        let mut left = self.parse_power()?;

        while !self.token_stream.at_end() {
            let token_kind = self.token_stream.token_kind();

            let operator = match token_kind {
                TokenKind::Multiply | TokenKind::Divide | TokenKind::Modulo => {
                    self.build_binary_operator()
                }
                _ => break,
            };

            self.token_stream.advance();
            let right = self.parse_power()?;

            left = Expr::binary(operator, left, right);
        }

        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Expr, KaoriError> {
        let mut left = self.parse_prefix_unary()?;

        while !self.token_stream.at_end() {
            let token_kind = self.token_stream.token_kind();

            let operator = match token_kind {
                TokenKind::Power => self.build_binary_operator(),
                _ => break,
            };

            self.token_stream.advance();
            let right = self.parse_power()?;

            left = Expr::binary(operator, left, right);
        }

        Ok(left)
    }

    fn parse_prefix_unary(&mut self) -> Result<Expr, KaoriError> {
        let token_kind = self.token_stream.token_kind();

        let operator = match token_kind {
            TokenKind::Plus => {
                self.token_stream.advance();
                return self.parse_prefix_unary();
            }
            TokenKind::Not => {
                self.token_stream.advance();

                let right = self.parse_or()?;

                return Ok(Expr::logical_not(right));
            }
            TokenKind::Minus => self.build_unary_operator(),
            _ => {
                let primary = self.parse_primary()?;

                return Ok(primary);
            }
        };

        self.token_stream.advance();

        let right = self.parse_prefix_unary()?;

        Ok(Expr::unary(operator, right))
    }

    fn parse_primary(&mut self) -> Result<Expr, KaoriError> {
        let token_kind = self.token_stream.token_kind();
        let span = self.token_stream.span();

        let primary = match token_kind {
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

                Expr::string_literal(value[1..value.len() - 1].to_owned(), span)
            }
            TokenKind::Identifier => {
                let identifier = self.parse_identifier()?;

                self.parse_postfix_unary(identifier)?
            }
            TokenKind::LeftBrace => self.parse_dict_literal()?,
            _ => {
                let span = self.token_stream.span();

                return Err(kaori_error!(
                    span,
                    "expected a valid operand, but found: {}",
                    token_kind
                ));
            }
        };

        Ok(primary)
    }

    fn parse_identifier(&mut self) -> Result<Expr, KaoriError> {
        let name = self.token_stream.lexeme().to_owned();
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::Identifier)?;

        Ok(Expr::identifier(name, span))
    }

    fn parse_dict_literal_field(&mut self) -> Result<(Expr, Option<Expr>), KaoriError> {
        let identifier = self.parse_expression()?;

        if self.token_stream.token_kind() == TokenKind::Colon {
            self.token_stream.consume(TokenKind::Colon)?;
            let expr = self.parse_expression()?;

            Ok((identifier, Some(expr)))
        } else {
            Ok((identifier, None))
        }
    }

    fn parse_dict_literal(&mut self) -> Result<Expr, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::LeftBrace)?;

        let fields =
            self.parse_comma_separator(Parser::parse_dict_literal_field, TokenKind::RightBrace)?;

        self.token_stream.consume(TokenKind::RightBrace)?;

        Ok(Expr::dict_literal(fields, span))
    }

    fn parse_postfix_unary(&mut self, operand: Expr) -> Result<Expr, KaoriError> {
        let token_kind = self.token_stream.token_kind();

        Ok(match token_kind {
            TokenKind::LeftParen => self.parse_function_call(operand)?,
            TokenKind::Dot => self.parse_member_access(operand)?,
            _ => operand,
        })
    }

    fn parse_function_call(&mut self, callee: Expr) -> Result<Expr, KaoriError> {
        self.token_stream.consume(TokenKind::LeftParen)?;

        let arguments =
            self.parse_comma_separator(Parser::parse_expression, TokenKind::RightParen)?;

        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::RightParen)?;

        let function_call = Expr::function_call(callee, arguments, span);

        self.parse_postfix_unary(function_call)
    }

    fn parse_member_access(&mut self, object: Expr) -> Result<Expr, KaoriError> {
        self.token_stream.consume(TokenKind::Dot)?;

        let property = Expr::string_literal(
            self.token_stream.lexeme().to_owned(),
            self.token_stream.span(),
        );

        self.token_stream.consume(TokenKind::Identifier)?;

        let member_access = Expr::member_access(object, property);

        self.parse_postfix_unary(member_access)
    }
}
