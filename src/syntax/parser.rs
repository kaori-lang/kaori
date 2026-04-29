use std::ops::Range;

use logos::SpannedIter;

use crate::{
    error::error::Error,
    report_error,
    syntax::{
        ast::{Ast, ExprId},
        ops::{AssignOp, BinaryOp, UnaryOp},
        token::Token,
    },
};

pub struct Parser<'a> {
    tokens: SpannedIter<'a, Token>,
    peeked: Option<(Token, Range<usize>)>,
    ast: Ast,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: SpannedIter<'a, Token>) -> Self {
        Self {
            tokens,
            peeked: None,
            ast: Ast::default(),
        }
    }

    pub fn parse(mut self) -> Result<Ast, Error> {
        let mut statements = Vec::new();

        while !self.at_end()? {
            let statement = self.parse_expression_statement()?;

            statements.push(statement);
        }

        self.ast.block(statements);

        Ok(self.ast)
    }

    fn at_end(&mut self) -> Result<bool, Error> {
        match self.peek() {
            Ok((token, _)) => Ok(token == Token::Eof),
            Err(err) => Err(err),
        }
    }

    fn peek(&mut self) -> Result<(Token, Range<usize>), Error> {
        if self.peeked.is_none() {
            self.next()?;
        }

        let (token, span) = self.peeked.as_ref().unwrap();

        Ok((*token, span.start..span.end))
    }

    fn peek_token(&mut self) -> Result<Token, Error> {
        let (token, _) = self.peek()?;

        Ok(token)
    }

    fn peek_span(&mut self) -> Result<Range<usize>, Error> {
        let (_, span) = self.peek()?;

        Ok(span)
    }

    fn consume(&mut self, expected: Token) -> Result<(), Error> {
        let (token, span) = self.peek()?;

        if token == expected {
            self.next()?;
            Ok(())
        } else {
            Err(report_error!(
                span,
                "expected {:?}, but found : {:?}",
                expected,
                token
            ))
        }
    }

    fn next(&mut self) -> Result<(Token, Range<usize>), Error> {
        if let Some((token, span)) = self.tokens.next() {
            match token {
                Ok(Token::UnterminatedStringLiteral) => {
                    Err(report_error!(span, "unterminated string literal"))
                }
                Ok(token) => {
                    self.peeked = Some((token, span.start..span.end));

                    Ok((token, span))
                }
                Err(_) => Err(report_error!(span, "invalid token")),
            }
        } else {
            self.peeked = Some((Token::Eof, 0..0));

            Ok((Token::Eof, 0..0))
        }
    }

    fn parse_expression_statement(&mut self) -> Result<ExprId, Error> {
        let token = self.peek_token()?;

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
        parse_item: fn(&mut Self) -> Result<T, Error>,
        terminator: Token,
    ) -> Result<Vec<T>, Error> {
        let mut items = Vec::new();

        while !self.at_end()? && self.peek_token()? != terminator {
            let item = parse_item(self)?;
            items.push(item);

            if self.peek_token()? == terminator {
                break;
            }

            self.consume(Token::Comma)?;
        }

        Ok(items)
    }

    fn parse_return(&mut self) -> Result<ExprId, Error> {
        let span = self.peek_span()?;

        self.consume(Token::Return)?;

        if self.peek_token()? == Token::Semicolon {
            return Ok(self.ast.return_(None, span));
        }

        let expression = Some(self.parse_expression()?);

        Ok(self.ast.return_(expression, span))
    }

    fn parse_continue(&mut self) -> Result<ExprId, Error> {
        let span = self.peek_span()?;

        self.consume(Token::Continue)?;

        Ok(self.ast.continue_(span))
    }

    fn parse_break(&mut self) -> Result<ExprId, Error> {
        let span = self.peek_span()?;

        self.consume(Token::Break)?;

        Ok(self.ast.break_(span))
    }

    fn parse_print(&mut self) -> Result<ExprId, Error> {
        let span = self.peek_span()?;

        self.consume(Token::Print)?;
        self.consume(Token::LeftParen)?;
        let expression = self.parse_expression()?;
        self.consume(Token::RightParen)?;

        Ok(self.ast.print(expression, span))
    }

    fn parse_block(&mut self) -> Result<ExprId, Error> {
        self.consume(Token::LeftBrace)?;

        let mut expressions = Vec::new();

        while !self.at_end()? && self.peek_token()? != Token::RightBrace {
            let expression = self.parse_expression_statement()?;

            expressions.push(expression);
        }

        self.consume(Token::RightBrace)?;

        Ok(self.ast.block(expressions))
    }

    fn parse_unchecked_block(&mut self) -> Result<ExprId, Error> {
        self.consume(Token::Unchecked)?;
        self.consume(Token::LeftBrace)?;

        let mut expressions = Vec::new();

        while !self.at_end()? && self.peek_token()? != Token::RightBrace {
            let expression = self.parse_expression_statement()?;

            expressions.push(expression);
        }

        self.consume(Token::RightBrace)?;

        Ok(self.ast.unchecked_block(expressions))
    }

    fn parse_if(&mut self) -> Result<ExprId, Error> {
        self.consume(Token::If)?;

        let condition = self.parse_expression()?;
        let then_branch = self.parse_block()?;

        if self.peek_token()? != Token::Else {
            return Ok(self.ast.if_(condition, then_branch, None));
        }

        self.next()?;

        if self.peek_token()? == Token::If {
            let else_branch = Some(self.parse_if()?);
            return Ok(self.ast.if_(condition, then_branch, else_branch));
        }

        let else_branch = Some(self.parse_block()?);

        Ok(self.ast.if_(condition, then_branch, else_branch))
    }

    fn parse_while_loop(&mut self) -> Result<ExprId, Error> {
        self.consume(Token::While)?;

        let condition = self.parse_expression()?;
        let block = self.parse_block()?;

        Ok(self.ast.while_loop(condition, block))
    }

    fn parse_for_loop(&mut self) -> Result<ExprId, Error> {
        todo!()
    }

    fn parse_function(&mut self) -> Result<ExprId, Error> {
        self.consume(Token::Function)?;

        let name = if self.peek_token()? == Token::Identifier {
            Some(self.parse_identifier()?)
        } else {
            None
        };

        self.consume(Token::LeftParen)?;

        let parameters = self.parse_comma_separator(Self::parse_identifier, Token::RightParen)?;

        self.consume(Token::RightParen)?;

        let captures = if self.peek_token()? == Token::Pipe {
            self.consume(Token::Pipe)?;
            let captures = self.parse_comma_separator(Self::parse_identifier, Token::Pipe)?;
            self.consume(Token::Pipe)?;
            captures
        } else {
            Vec::new()
        };

        let mut body = Vec::new();

        self.consume(Token::LeftBrace)?;

        while !self.at_end()? && self.peek_token()? != Token::RightBrace {
            let statement = self.parse_expression_statement()?;
            body.push(statement);
        }

        self.consume(Token::RightBrace)?;

        Ok(self.ast.function(name, parameters, captures, body))
    }

    fn parse_expression(&mut self) -> Result<ExprId, Error> {
        let assign = self.parse_assign()?;

        Ok(assign)
    }

    fn parse_assign(&mut self) -> Result<ExprId, Error> {
        let left = self.parse_or()?;

        let (token, span) = self.peek()?;

        let operator = match token {
            Token::Assign => AssignOp::Assign,
            Token::AddAssign => AssignOp::AddAssign,
            Token::SubtractAssign => AssignOp::SubtractAssign,
            Token::MultiplyAssign => AssignOp::MultiplyAssign,
            Token::DivideAssign => AssignOp::DivideAssign,
            Token::ModuloAssign => AssignOp::ModuloAssign,
            Token::DeclareAssign => {
                self.next()?;
                let right = self.parse_or()?;

                return Ok(self.ast.declare_assign(left, right, span));
            }
            _ => return Ok(left),
        };

        self.next()?;

        let right = self.parse_or()?;

        Ok(self.ast.assign(operator, left, right, span))
    }

    fn parse_or(&mut self) -> Result<ExprId, Error> {
        let mut left = self.parse_and()?;

        while !self.at_end()? {
            let (token, span) = self.peek()?;

            let Token::Or = token else {
                break;
            };

            self.next()?;

            let right = self.parse_and()?;

            left = self.ast.logical_or(left, right, span);
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<ExprId, Error> {
        let mut left = self.parse_equality()?;

        while !self.at_end()? {
            let (token, span) = self.peek()?;

            let Token::And = token else {
                break;
            };

            self.next()?;

            let right = self.parse_equality()?;

            left = self.ast.logical_and(left, right, span);
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<ExprId, Error> {
        let mut left = self.parse_comparison()?;

        while !self.at_end()? {
            let (token, span) = self.peek()?;

            let operator = match token {
                Token::Equal => BinaryOp::Equal,
                Token::NotEqual => BinaryOp::NotEqual,
                _ => break,
            };

            self.next()?;

            let right = self.parse_comparison()?;

            left = self.ast.binary(operator, left, right, span);
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<ExprId, Error> {
        let mut left = self.parse_term()?;

        while !self.at_end()? {
            let (token, span) = self.peek()?;

            let operator = match token {
                Token::Greater => BinaryOp::Greater,
                Token::GreaterEqual => BinaryOp::GreaterEqual,
                Token::Less => BinaryOp::Less,
                Token::LessEqual => BinaryOp::LessEqual,
                _ => break,
            };

            self.next()?;

            let right = self.parse_term()?;

            left = self.ast.binary(operator, left, right, span);
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<ExprId, Error> {
        let mut left = self.parse_factor()?;

        while !self.at_end()? {
            let (token, span) = self.peek()?;

            let operator = match token {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Subtract,
                _ => break,
            };

            self.next()?;

            let right = self.parse_factor()?;

            left = self.ast.binary(operator, left, right, span);
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<ExprId, Error> {
        let mut left = self.parse_prefix_unary()?;

        while !self.at_end()? {
            let (token, span) = self.peek()?;

            let operator = match token {
                Token::Multiply => BinaryOp::Multiply,
                Token::Divide => BinaryOp::Divide,
                Token::Modulo => BinaryOp::Modulo,
                _ => break,
            };

            self.next()?;

            let right = self.parse_prefix_unary()?;

            left = self.ast.binary(operator, left, right, span);
        }

        Ok(left)
    }

    fn parse_prefix_unary(&mut self) -> Result<ExprId, Error> {
        let (token, span) = self.peek()?;

        let operator = match token {
            Token::Plus => {
                self.next()?;

                return self.parse_prefix_unary();
            }
            Token::Not => {
                self.next()?;

                let right = self.parse_or()?;

                return Ok(self.ast.logical_not(right, span));
            }
            Token::Minus => UnaryOp::Negate,
            _ => {
                let primary = self.parse_primary()?;

                return Ok(primary);
            }
        };

        self.next()?;

        let right = self.parse_prefix_unary()?;

        Ok(self.ast.unary(operator, right, span))
    }

    fn parse_primary(&mut self) -> Result<ExprId, Error> {
        let (token, span) = self.peek()?;

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
                let value = match self.tokens.slice().parse::<f64>() {
                    Ok(value) => Ok(value),
                    Err(..) => Err(report_error!(
                        span.start..span.end,
                        "expected a valid float to be parsed"
                    )),
                }?;

                self.next()?;

                self.ast.number_literal(value, span)
            }
            Token::True => {
                self.next()?;

                self.ast.boolean_literal(true, span)
            }
            Token::False => {
                self.next()?;

                self.ast.boolean_literal(false, span)
            }
            Token::StringLiteral => {
                let value = self.tokens.slice().to_owned();
                self.next()?;

                self.ast
                    .string_literal(value[1..value.len() - 1].to_owned(), span)
            }
            Token::Identifier => {
                let identifier = self.parse_identifier()?;

                self.parse_postfix_unary(identifier)?
            }
            Token::LeftBrace => self.parse_dict_literal()?,
            _ => {
                let span = self.peek_span()?;

                return Err(report_error!(
                    span,
                    "expected a valid operand, but found: {:?}",
                    token
                ));
            }
        };

        Ok(primary)
    }

    fn parse_identifier(&mut self) -> Result<ExprId, Error> {
        let name = self.tokens.slice().to_owned();
        let span = self.peek_span()?;

        self.consume(Token::Identifier)?;

        Ok(self.ast.identifier(name, span))
    }

    fn parse_dict_literal_field(&mut self) -> Result<(ExprId, Option<ExprId>), Error> {
        let identifier = self.parse_expression()?;

        if self.peek_token()? == Token::Colon {
            self.consume(Token::Colon)?;
            let expr = self.parse_expression()?;

            Ok((identifier, Some(expr)))
        } else {
            Ok((identifier, None))
        }
    }

    fn parse_dict_literal(&mut self) -> Result<ExprId, Error> {
        self.consume(Token::LeftBrace)?;

        let fields =
            self.parse_comma_separator(Self::parse_dict_literal_field, Token::RightBrace)?;

        self.consume(Token::RightBrace)?;

        Ok(self.ast.dict_literal(fields))
    }

    fn parse_postfix_unary(&mut self, operand: ExprId) -> Result<ExprId, Error> {
        let token = self.peek_token()?;

        Ok(match token {
            Token::LeftParen => self.parse_function_call(operand)?,
            Token::Dot => self.parse_member_access(operand)?,
            _ => operand,
        })
    }

    fn parse_function_call(&mut self, callee: ExprId) -> Result<ExprId, Error> {
        self.consume(Token::LeftParen)?;

        let arguments = self.parse_comma_separator(Self::parse_expression, Token::RightParen)?;

        self.consume(Token::RightParen)?;

        let function_call = self.ast.function_call(callee, arguments);

        self.parse_postfix_unary(function_call)
    }

    fn parse_member_access(&mut self, object: ExprId) -> Result<ExprId, Error> {
        self.consume(Token::Dot)?;

        let span = self.peek_span()?;

        let property = self
            .ast
            .string_literal(self.tokens.slice().to_owned(), span);

        self.consume(Token::Identifier)?;

        let member_access = self.ast.member_access(object, property);

        self.parse_postfix_unary(member_access)
    }
}
