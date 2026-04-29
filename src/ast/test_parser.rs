use std::iter::Peekable;

use logos::{Span, SpannedIter};

use crate::{
    ast::{
        expr::Expr,
        ops::{AssignOp, BinaryOp, UnaryOp},
        token::{self, Token},
    },
    error::kaori_error::KaoriError,
    kaori_error,
};

pub struct Parser<'a> {
    tokens: Peekable<SpannedIter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn parse(&mut self) -> Result<Vec<Expr>, KaoriError> {
        let mut statements = Vec::new();
        Span::
        let token = self.tokens.peek();
        while let Some(token) = self.tokens.peek() {
            let statement = self.parse_expression_statement()?;

            statements.push(statement);
        }

        Ok(statements)
    }


    fn peek(&mut self) -> Result<(Token, Span), KaoriError> {
        if let Some((token, span)) = self.tokens.peek_mut().copied(){
            
           match token {
            Ok(token) if token == Token::UnterminatedStringLiteral => kaori_error!(span, "unterminated string literal"),
            Ok(token) => Ok((token, span)),
            Err(_) => Err(kaori_error!(*span, "invalid token"))
           }
        } else {
            Err(kaori_error!("unexpected end"))
        }
    }

    fn consume(&mut self, expected: Token) -> Result<(), KaoriError> {
        let (token, span) = self.peek()?;

        if token == expected {
            self.tokens.next();
            Ok(())
        } else {
            Err(kaori_error!(span, "expected {}, but found : {}", expected, token))
        }
    }

    fn parse_expression_statement(&mut self) -> Result<Expr, KaoriError> {
        let (token, _) = self.peek()?;

         let expression = match token {
            Token::Function => self.parse_function(),
            Token::Print => self.parse_print(),
            Token::If => self.parse_if(),
            Token::While => self.parse_while_loop(),
            Token::For => self.parse_for_loop(),
            Token::Break => self.parse_break(),
            Token::Continue => self.parse_continue(),
            Token::Return => self.parse_return(),
            Token::Unchecked => self.parse_unchecked_block(),
            _ => {
                let expression = self.parse_expression();

                if expression.is_ok() {
                    self.consume(Token::Semicolon)?;
                    expression
                } else {
                    expression
                }
            }
        }?;

        match token {
            Token::Print | Token::Break | Token::Continue | Token::Return => {
                self.consume(Token::Semicolon)?;
            }
            _ => (),
        };

        Ok(expression)
    }

    fn parse_comma_separator<T>(
        &mut self,
        parse_item: fn(&mut Self) -> Result<T, KaoriError>,
        terminator: Token,
    ) -> Result<Vec<T>, KaoriError> {
        let mut items = Vec::new();

        while !self.token_stream.at_end() && self.token_stream.token() != terminator {
            let item = parse_item(self)?;
            items.push(item);

            if self.token_stream.token() == terminator {
                break;
            }

            self.consume(Token::Comma)?;
        }

        Ok(items)
    }

    fn parse_return(&mut self) -> Result<Expr, KaoriError> {
        let (_, span) = self.peek()?;

        self.consume(Token::Return)?;

        if self.token_stream.token() == Token::Semicolon {
            return Ok(Expr::return_(None, span));
        }

        let expression = Some(self.parse_expression()?);

        Ok(Expr::return_(expression, span))
    }

    fn parse_continue(&mut self) -> Result<Expr, KaoriError> {
        let (_, span) = self.peek()?;

        self.consume(Token::Continue)?;

        Ok(Expr::continue_(span))
    }

    fn parse_break(&mut self) -> Result<Expr, KaoriError> {
        let (_, span) = self.peek()?;

        self.consume(Token::Break)?;

        Ok(Expr::break_(span))
    }

    fn parse_print(&mut self) -> Result<Expr, KaoriError> {
        let (_, span) = self.peek()?;

        self.consume(Token::Print)?;
        self.consume(Token::LeftParen)?;
        let expression = self.parse_expression()?;
        self.consume(Token::RightParen)?;

        Ok(Expr::print(expression, span))
    }

    fn parse_block(&mut self) -> Result<Expr, KaoriError> {
        let (_, span) = self.peek()?;

        self.consume(Token::LeftBrace)?;

        let mut expressions = Vec::new();

        while !self.token_stream.at_end() && self.token_stream.token() != Token::RightBrace {
            let expression = self.parse_expression_statement()?;

            expressions.push(expression);
        }

        let tail = expressions.pop();

        self.consume(Token::RightBrace)?;

        Ok(Expr::block(expressions, tail, span))
    }

    fn parse_unchecked_block(&mut self) -> Result<Expr, KaoriError> {
        let (_, span) = self.peek()?;

        self.consume(Token::Unchecked)?;
        self.consume(Token::LeftBrace)?;

        let mut expressions = Vec::new();

        while !self.token_stream.at_end() && self.token_stream.token() != Token::RightBrace {
            let expression = self.parse_expression_statement()?;

            expressions.push(expression);
        }

        let tail = expressions.pop();

        self.consume(Token::RightBrace)?;

        Ok(Expr::unchecked_block(expressions, tail, span))
    }

    fn parse_if(&mut self) -> Result<Expr, KaoriError> {
        let (_, span) = self.peek()?;

        self.consume(Token::If)?;

        let condition = self.parse_expression()?;
        let then_branch = self.parse_block()?;

        if self.token_stream.token() != Token::Else {
            return Ok(Expr::if_(condition, then_branch, None, span));
        }

        self.token_stream.advance();

        if self.token_stream.token() == Token::If {
            let else_branch = Some(self.parse_if()?);
            return Ok(Expr::if_(condition, then_branch, else_branch, span));
        }

        let else_branch = Some(self.parse_block()?);

        Ok(Expr::if_(condition, then_branch, else_branch, span))
    }

    fn parse_while_loop(&mut self) -> Result<Expr, KaoriError> {
        let (_, span) = self.peek()?;

        self.consume(Token::While)?;

        let condition = self.parse_expression()?;
        let block = self.parse_block()?;

        Ok(Expr::while_loop(condition, block, span))
    }

    fn parse_for_loop(&mut self) -> Result<Expr, KaoriError> {
        let (_, span) = self.peek()?;

        self.consume(Token::For)?;

        let start = self.parse_expression()?;

        let down_to = match self.token_stream.token() {
            Token::To => false,
            Token::DownTo => true,
            _ => {
                return Err(kaori_error!(
                    span,
                    "expected {} or {} and found {}",
                    Token::To,
                    Token::DownTo,
                    self.token_stream.token(),
                ));
            }
        };

        self.token_stream.advance();

        let end = self.parse_expression()?;

        let block = self.parse_block()?;

        Ok(Expr::for_loop(start, end, block, span))
    }

    fn parse_function(&mut self) -> Result<Expr, KaoriError> {
        let (_, span) = self.peek()?;

        self.consume(Token::Function)?;

        let name = if self.token_stream.token() == Token::Identifier {
            Some(self.parse_identifier()?)
        } else {
            None
        };

        self.consume(Token::LeftParen)?;

        let parameters = self.parse_comma_separator(Self::parse_identifier, Token::RightParen)?;

        self.consume(Token::RightParen)?;

        let captures = if self.token_stream.token() == Token::Pipe {
            self.consume(Token::Pipe)?;
            let captures = self.parse_comma_separator(Self::parse_identifier, Token::Pipe)?;
            self.consume(Token::Pipe)?;
            captures
        } else {
            Vec::new()
        };

        let mut body = Vec::new();

        self.consume(Token::LeftBrace)?;

        while !self.token_stream.at_end() && self.token_stream.token() != Token::RightBrace {
            let statement = self.parse_expression_statement()?;
            body.push(statement);
        }

        self.consume(Token::RightBrace)?;

        Ok(Expr::function(name, parameters, captures, body, span))
    }

    fn build_binary_operator(&mut self) -> BinaryOp {
        let token = self.token_stream.token();

        match token {
            Token::Plus => BinaryOp::Add,
            Token::Minus => BinaryOp::Subtract,
            Token::Multiply => BinaryOp::Multiply,
            Token::Divide => BinaryOp::Divide,
            Token::Modulo => BinaryOp::Modulo,
            Token::Equal => BinaryOp::Equal,
            Token::NotEqual => BinaryOp::NotEqual,
            Token::Greater => BinaryOp::Greater,
            Token::GreaterEqual => BinaryOp::GreaterEqual,
            Token::Less => BinaryOp::Less,
            Token::LessEqual => BinaryOp::LessEqual,
            _ => unreachable!(),
        }
    }

    fn build_unary_operator(&mut self) -> UnaryOp {
        let token = self.token_stream.token();

        match token {
            Token::Minus => UnaryOp::Negate,
            _ => unreachable!(),
        }
    }

    fn parse_expression(&mut self) -> Result<Expr, KaoriError> {
        let assign = self.parse_assign()?;

        Ok(assign)
    }

    fn parse_assign(&mut self) -> Result<Expr, KaoriError> {
        let left = self.parse_or()?;

        let token = self.token_stream.token();

        let operator = match token {
            Token::Assign => AssignOp::Assign,
            Token::AddAssign => AssignOp::AddAssign,
            Token::SubtractAssign => AssignOp::SubtractAssign,
            Token::MultiplyAssign => AssignOp::MultiplyAssign,
            Token::DivideAssign => AssignOp::DivideAssign,
            Token::ModuloAssign => AssignOp::ModuloAssign,
            Token::DeclareAssign => {
                self.token_stream.advance();

                let right = self.parse_or()?;

                return Ok(Expr::declare_assign(left, right));
            }
            _ => return Ok(left),
        };

        self.token_stream.advance();

        let right = self.parse_or()?;

        Ok(Expr::assign(operator, left, right))
    }

    fn parse_or(&mut self) -> Result<Expr, KaoriError> {
        let mut left = self.parse_and()?;

        while !self.token_stream.at_end() {
            let token = self.token_stream.token();

            let Token::Or = token else {
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
            let token = self.token_stream.token();

            let Token::And = token else {
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
            let token = self.token_stream.token();

            let operator = match token {
                Token::Equal | Token::NotEqual => self.build_binary_operator(),
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
            let token = self.token_stream.token();

            let operator = match token {
                Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual => {
                    self.build_binary_operator()
                }
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
            let token = self.token_stream.token();

            let operator = match token {
                Token::Plus | Token::Minus => self.build_binary_operator(),
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
            let token = self.token_stream.token();

            let operator = match token {
                Token::Multiply | Token::Divide | Token::Modulo => self.build_binary_operator(),
                _ => break,
            };

            self.token_stream.advance();
            let right = self.parse_prefix_unary()?;

            left = Expr::binary(operator, left, right);
        }

        Ok(left)
    }

    fn parse_prefix_unary(&mut self) -> Result<Expr, KaoriError> {
        let token = self.token_stream.token();

        let operator = match token {
            Token::Plus => {
                self.token_stream.advance();
                return self.parse_prefix_unary();
            }
            Token::Not => {
                self.token_stream.advance();

                let right = self.parse_or()?;

                return Ok(Expr::logical_not(right));
            }
            Token::Minus => self.build_unary_operator(),
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
        let token = self.token_stream.token();
        let (_, span) = self.peek()?;

        let primary = match token {
            Token::If => self.parse_if()?,
            Token::Function => self.parse_function()?,
            Token::LeftParen => {
                self.consume(Token::LeftParen)?;
                let expr = self.parse_expression()?;
                self.consume(Token::RightParen)?;
                expr
            }
            Token::NumberLiteral => {
                let value = match self.token_stream.lexeme().parse::<f64>() {
                    Ok(value) => Ok(value),
                    Err(..) => Err(kaori_error!(span, "expected a valid float to be parsed")),
                }?;

                self.token_stream.advance();

                Expr::number_literal(value, span)
            }
            Token::True => {
                self.token_stream.advance();

                Expr::boolean_literal(true, span)
            }
            Token::False => {
                self.token_stream.advance();

                Expr::boolean_literal(false, span)
            }
            Token::StringLiteral => {
                let value = self.token_stream.lexeme().to_owned();
                self.token_stream.advance();

                Expr::string_literal(value[1..value.len() - 1].to_owned(), span)
            }
            Token::Identifier => {
                let identifier = self.parse_identifier()?;

                self.parse_postfix_unary(identifier)?
            }
            Token::LeftBrace => self.parse_dict_literal()?,
            _ => {
                let (_, span) = self.peek()?;

                return Err(kaori_error!(
                    span,
                    "expected a valid operand, but found: {}",
                    token
                ));
            }
        };

        Ok(primary)
    }

    fn parse_identifier(&mut self) -> Result<Expr, KaoriError> {
        let name = self.token_stream.lexeme().to_owned();
        let (_, span) = self.peek()?;

        self.consume(Token::Identifier)?;

        Ok(Expr::identifier(name, span))
    }

    fn parse_dict_literal_field(&mut self) -> Result<(Expr, Option<Expr>), KaoriError> {
        let identifier = self.parse_expression()?;

        if self.token_stream.token() == Token::Colon {
            self.consume(Token::Colon)?;
            let expr = self.parse_expression()?;

            Ok((identifier, Some(expr)))
        } else {
            Ok((identifier, None))
        }
    }

    fn parse_dict_literal(&mut self) -> Result<Expr, KaoriError> {
        let (_, span) = self.peek()?;

        self.consume(Token::LeftBrace)?;

        let fields =
            self.parse_comma_separator(Self::parse_dict_literal_field, Token::RightBrace)?;

        self.consume(Token::RightBrace)?;

        Ok(Expr::dict_literal(fields, span))
    }

    fn parse_postfix_unary(&mut self, operand: Expr) -> Result<Expr, KaoriError> {
        let token = self.token_stream.token();

        Ok(match token {
            Token::LeftParen => self.parse_function_call(operand)?,
            Token::Dot => self.parse_member_access(operand)?,
            _ => operand,
        })
    }

    fn parse_function_call(&mut self, callee: Expr) -> Result<Expr, KaoriError> {
        self.consume(Token::LeftParen)?;

        let arguments = self.parse_comma_separator(Self::parse_expression, Token::RightParen)?;

        let (_, span) = self.peek()?;

        self.consume(Token::RightParen)?;

        let function_call = Expr::function_call(callee, arguments, span);

        self.parse_postfix_unary(function_call)
    }

    fn parse_member_access(&mut self, object: Expr) -> Result<Expr, KaoriError> {
        self.consume(Token::Dot)?;

        let property = Expr::string_literal(
            self.token_stream.lexeme().to_owned(),
            self.token_stream.span(),
        );

        self.consume(Token::Identifier)?;

        let member_access = Expr::member_access(object, property);

        self.parse_postfix_unary(member_access)
    }
}
