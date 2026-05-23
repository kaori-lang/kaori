use std::ops::Range;

use crate::{
    diagnostics::error::Error,
    program::INTERNER,
    report_error,
    syntax::{
        ast::{Ast, ExprId},
        ops::{AssignOp, BinaryOp, UnaryOp},
        token::{Span, Token},
    },
};

pub struct Parser<'a> {
    source: &'a str,
    tokens: Vec<(Token, Span)>,
    pos: usize,
    ast: Ast,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, tokens: Vec<(Token, Span)>) -> Self {
        Self {
            source,
            tokens,
            pos: 0,
            ast: Ast::default(),
        }
    }

    pub fn parse(mut self) -> Result<Ast, Error> {
        let mut expressions = Vec::new();

        while !self.at_end() {
            let (expression, require_semicolon) = self.parse_expression_statement()?;
            expressions.push(expression);

            if require_semicolon {
                self.consume(Token::Semicolon)?;
            }
        }

        self.ast.block(expressions);

        Ok(self.ast)
    }

    fn at_end(&mut self) -> bool {
        self.peek_token() == Token::Eof
    }

    fn lexeme(&self, span: Span) -> &str {
        &self.source[Range::<usize>::from(span)]
    }

    fn peek(&mut self) -> (Token, Span) {
        self.tokens[self.pos]
    }

    fn peek_token(&mut self) -> Token {
        let (token, _) = self.tokens[self.pos];

        token
    }

    fn peek_span(&mut self) -> Span {
        let (_, span) = self.tokens[self.pos];

        span
    }

    fn consume(&mut self, expected: Token) -> Result<(), Error> {
        let (token, span) = self.peek();

        if token == expected {
            self.advance_token();
            Ok(())
        } else {
            Err(report_error!(
                span,
                "expected {} and found {}",
                expected,
                token
            ))
        }
    }

    fn advance_token(&mut self) {
        self.pos += 1;
    }

    fn parse_comma_separator<T>(
        &mut self,
        parse_item: fn(&mut Self) -> Result<T, Error>,
        terminator: Token,
    ) -> Result<Vec<T>, Error> {
        let mut items = Vec::new();

        while !self.at_end() && self.peek_token() != terminator {
            let item = parse_item(self)?;
            items.push(item);

            if self.peek_token() == terminator {
                break;
            }

            self.consume(Token::Comma)?;
        }

        Ok(items)
    }

    fn parse_expression_statement(&mut self) -> Result<(ExprId, bool), Error> {
        let token = self.peek_token();

        let require_semicolon = !matches!(
            token,
            Token::Function | Token::While | Token::For | Token::If
        );

        let expression = match token {
            Token::Function => self.parse_function(),
            Token::Native => self.parse_native_function(),
            Token::While => self.parse_while_loop(),
            Token::For => self.parse_for_loop(),
            Token::Break => self.parse_break(),
            Token::Continue => self.parse_continue(),
            Token::Return => self.parse_return(),
            Token::If => self.parse_if(),
            Token::Let => self.parse_variable(),
            Token::Mut => self.parse_mut(),
            _ => self.parse_expression(),
        }?;

        Ok((expression, require_semicolon))
    }

    fn parse_expression(&mut self) -> Result<ExprId, Error> {
        let assign = self.parse_assign()?;

        Ok(assign)
    }

    fn parse_return(&mut self) -> Result<ExprId, Error> {
        let span = self.peek_span();

        self.consume(Token::Return)?;

        let expression = self.parse_expression()?;

        Ok(self.ast.return_(expression, span))
    }

    fn parse_continue(&mut self) -> Result<ExprId, Error> {
        let span = self.peek_span();

        self.consume(Token::Continue)?;

        Ok(self.ast.continue_(span))
    }

    fn parse_break(&mut self) -> Result<ExprId, Error> {
        let span = self.peek_span();

        self.consume(Token::Break)?;

        Ok(self.ast.break_(span))
    }

    fn parse_block(&mut self) -> Result<ExprId, Error> {
        self.consume(Token::LeftBrace)?;

        let mut expressions = Vec::new();

        while !self.at_end() && self.peek_token() != Token::RightBrace {
            let (expression, requires_semicolon) = self.parse_expression_statement()?;
            expressions.push(expression);

            if self.peek_token() != Token::RightBrace && requires_semicolon {
                self.consume(Token::Semicolon)?;
            }
        }

        self.consume(Token::RightBrace)?;

        Ok(self.ast.block(expressions))
    }

    fn parse_if(&mut self) -> Result<ExprId, Error> {
        self.consume(Token::If)?;

        let condition = self.parse_expression()?;
        let then_branch = self.parse_block()?;

        if self.peek_token() != Token::Else {
            let empty_else_branch = self.ast.block(vec![]);

            return Ok(self.ast.if_(condition, then_branch, empty_else_branch));
        }

        self.advance_token();

        if self.peek_token() == Token::If {
            let else_branch = self.parse_if()?;

            return Ok(self.ast.if_(condition, then_branch, else_branch));
        }

        let else_branch = self.parse_block()?;

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

    fn parse_native_function(&mut self) -> Result<ExprId, Error> {
        self.consume(Token::Native)?;
        self.consume(Token::Function)?;

        let name = self.parse_identifier()?;

        self.consume(Token::LeftParen)?;

        let parameters = self.parse_comma_separator(Self::parse_identifier, Token::RightParen)?;

        self.consume(Token::RightParen)?;

        Ok(self.ast.native_function(name, parameters))
    }

    fn parse_function(&mut self) -> Result<ExprId, Error> {
        self.consume(Token::Function)?;

        let name = if self.peek_token() == Token::Identifier {
            Some(self.parse_identifier()?)
        } else {
            None
        };

        self.consume(Token::LeftParen)?;

        let parameters = self.parse_comma_separator(Self::parse_identifier, Token::RightParen)?;

        self.consume(Token::RightParen)?;

        let block = self.parse_block()?;

        Ok(self.ast.function(name, parameters, block))
    }

    fn parse_variable(&mut self) -> Result<ExprId, Error> {
        self.consume(Token::Let)?;

        let left = self.parse_identifier()?;

        self.consume(Token::Assign)?;

        let right = self.parse_expression()?;

        Ok(self.ast.variable(left, right))
    }

    fn parse_mut(&mut self) -> Result<ExprId, Error> {
        self.consume(Token::Let)?;

        let left = self.parse_identifier()?;

        self.consume(Token::Assign)?;

        let right = self.parse_expression()?;

        Ok(self.ast.mut_(left, right))
    }

    fn parse_assign(&mut self) -> Result<ExprId, Error> {
        let left = self.parse_or()?;

        let (token, span) = self.peek();

        let operator = match token {
            Token::AddAssign => AssignOp::AddAssign,
            Token::SubtractAssign => AssignOp::SubtractAssign,
            Token::MultiplyAssign => AssignOp::MultiplyAssign,
            Token::DivideAssign => AssignOp::DivideAssign,
            Token::ModuloAssign => AssignOp::ModuloAssign,
            Token::Assign => {
                self.advance_token();
                let right = self.parse_or()?;

                return Ok(self.ast.assign(left, right, span));
            }
            _ => return Ok(left),
        };

        self.advance_token();

        let right = self.parse_or()?;

        Ok(self.ast.compound_assign(operator, left, right, span))
    }

    fn parse_or(&mut self) -> Result<ExprId, Error> {
        let mut left = self.parse_and()?;

        while !self.at_end() {
            let (token, span) = self.peek();

            let Token::Or = token else {
                break;
            };

            self.advance_token();

            let right = self.parse_and()?;

            left = self.ast.logical_or(left, right, span);
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<ExprId, Error> {
        let mut left = self.parse_equality()?;

        while !self.at_end() {
            let (token, span) = self.peek();

            let Token::And = token else {
                break;
            };

            self.advance_token();

            let right = self.parse_equality()?;

            left = self.ast.logical_and(left, right, span);
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<ExprId, Error> {
        let mut left = self.parse_comparison()?;

        while !self.at_end() {
            let (token, span) = self.peek();

            let operator = match token {
                Token::Equal => BinaryOp::Equal,
                Token::NotEqual => BinaryOp::NotEqual,
                _ => break,
            };

            self.advance_token();

            let right = self.parse_comparison()?;

            left = self.ast.binary(operator, left, right, span);
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<ExprId, Error> {
        let mut left = self.parse_term()?;

        while !self.at_end() {
            let (token, span) = self.peek();

            let operator = match token {
                Token::Greater => BinaryOp::Greater,
                Token::GreaterEqual => BinaryOp::GreaterEqual,
                Token::Less => BinaryOp::Less,
                Token::LessEqual => BinaryOp::LessEqual,
                _ => break,
            };

            self.advance_token();

            let right = self.parse_term()?;

            left = self.ast.binary(operator, left, right, span);
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<ExprId, Error> {
        let mut left = self.parse_factor()?;

        while !self.at_end() {
            let (token, span) = self.peek();

            let operator = match token {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Subtract,
                _ => break,
            };

            self.advance_token();

            let right = self.parse_factor()?;

            left = self.ast.binary(operator, left, right, span);
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<ExprId, Error> {
        let mut left = self.parse_prefix_unary()?;

        while !self.at_end() {
            let (token, span) = self.peek();

            let operator = match token {
                Token::Multiply => BinaryOp::Multiply,
                Token::Divide => BinaryOp::Divide,
                Token::Modulo => BinaryOp::Modulo,
                _ => break,
            };

            self.advance_token();

            let right = self.parse_prefix_unary()?;

            left = self.ast.binary(operator, left, right, span);
        }

        Ok(left)
    }

    fn parse_prefix_unary(&mut self) -> Result<ExprId, Error> {
        let (token, span) = self.peek();

        let operator = match token {
            Token::Plus => {
                self.advance_token();

                return self.parse_prefix_unary();
            }
            Token::Not => {
                self.advance_token();

                let right = self.parse_or()?;

                return Ok(self.ast.logical_not(right, span));
            }
            Token::Minus => UnaryOp::Negate,
            _ => {
                let primary = self.parse_primary()?;

                return Ok(primary);
            }
        };

        self.advance_token();

        let right = self.parse_prefix_unary()?;

        Ok(self.ast.unary(operator, right, span))
    }

    fn parse_primary(&mut self) -> Result<ExprId, Error> {
        let (token, span) = self.peek();

        let primary = match token {
            Token::Function => self.parse_function()?,
            Token::If => self.parse_if()?,
            Token::LeftParen => {
                self.consume(Token::LeftParen)?;
                let expression = self.parse_expression()?;
                self.consume(Token::RightParen)?;

                expression
            }
            Token::NumberLiteral => {
                let span = self.peek_span();
                let lexeme = self.lexeme(span);

                let value = match lexeme.parse::<f64>() {
                    Ok(value) => Ok(value),
                    Err(..) => Err(report_error!(span, "failed to parse float")),
                }?;

                self.advance_token();

                self.ast.number_literal(value, span)
            }
            Token::True => {
                self.advance_token();

                self.ast.boolean_literal(true, span)
            }
            Token::False => {
                self.advance_token();

                self.ast.boolean_literal(false, span)
            }
            Token::StringLiteral => {
                let span = self.peek_span();
                self.advance_token();
                let lexeme = self.lexeme(span);

                let index = INTERNER
                    .lock()
                    .unwrap()
                    .get_or_intern(&lexeme[1..lexeme.len() - 1]);

                self.ast.string_literal(index, span)
            }
            Token::Identifier => {
                let identifier = self.parse_identifier()?;

                self.parse_postfix_unary(identifier)?
            }
            Token::LeftBrace => self.parse_dict_literal()?,
            _ => {
                let span = self.peek_span();

                return Err(report_error!(
                    span,
                    "expected a <operand> and found: {}",
                    token
                ));
            }
        };

        Ok(primary)
    }

    fn parse_identifier(&mut self) -> Result<ExprId, Error> {
        let span = self.peek_span();
        let lexeme = self.lexeme(span);

        let index = INTERNER.lock().unwrap().get_or_intern(lexeme);

        self.consume(Token::Identifier)?;

        Ok(self.ast.identifier(index, span))
    }

    fn parse_dict_literal_field(&mut self) -> Result<(ExprId, Option<ExprId>), Error> {
        let identifier = self.parse_expression()?;

        if self.peek_token() == Token::Colon {
            self.consume(Token::Colon)?;
            let expression = self.parse_expression()?;

            Ok((identifier, Some(expression)))
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
        let token = self.peek_token();

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

        let span = self.peek_span();
        let lexeme = self.lexeme(span);
        let index = INTERNER.lock().unwrap().get_or_intern(lexeme);

        let property = self.ast.string_literal(index, span);

        self.consume(Token::Identifier)?;

        let member_access = self.ast.member_access(object, property);

        self.parse_postfix_unary(member_access)
    }
}
