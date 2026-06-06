use std::ops::Range;

use crate::{
    compiler::{Compiler, INTERNER},
    diagnostics::error::Error,
    report_error,
    syntax::{
        ast::{Ast, NodeId, Spanned},
        ops::{BinaryOp, UnaryOp},
        token::{Span, Token},
    },
    util::string_interner::Symbol,
};

pub struct Parser<'a> {
    source: &'a str,
    tokens: Vec<(Token, Span)>,
    pos: usize,
    ast: Ast,
    compiler: &'a Compiler,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, tokens: Vec<(Token, Span)>, compiler: &'a Compiler) -> Self {
        Self {
            source,
            tokens,
            pos: 0,
            ast: Ast::default(),
            compiler,
        }
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

    fn consume(&mut self, expected: Token) -> Result<Span, Error> {
        let (token, span) = self.peek();

        if token == expected {
            self.advance_token();
            Ok(span)
        } else {
            report_error!(
                span,
                self.compiler.path,
                "expected {} and found {}",
                expected,
                token
            )
        }
    }

    fn advance_token(&mut self) {
        self.pos += 1;
    }

    pub fn parse(mut self) -> Result<Ast, Error> {
        let mut statements = Vec::new();

        while !self.at_end() {
            let (statement, _) = self.parse_statement()?;
            statements.push(statement);
        }

        self.ast.block(statements, None, Span::default());

        Ok(self.ast)
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

    fn parse_expression(&mut self) -> Result<NodeId, Error> {
        let assign = self.parse_assign()?;

        Ok(assign)
    }

    fn parse_statement(&mut self) -> Result<(NodeId, bool), Error> {
        let token = self.peek_token();

        let statement = match token {
            Token::Use => self.parse_use(),
            Token::LeftBrace => self.parse_block(),
            Token::If => self.parse_if(),
            Token::Function => self.parse_function(),
            Token::While => self.parse_while_loop(),
            Token::For => self.parse_for_loop(),
            Token::Break => self.parse_break(),
            Token::Continue => self.parse_continue(),
            Token::Return => self.parse_return(),
            Token::Let => self.parse_variable(),
            Token::Const => self.parse_constant(),
            Token::Ref => self.parse_ref(),
            _ => self.parse_expression(),
        }?;

        let consumes_semicolon = self.peek_token() != Token::RightBrace
            && !matches!(
                token,
                Token::LeftBrace | Token::If | Token::Function | Token::While
            )
            || self.peek_token() == Token::Semicolon;

        if consumes_semicolon {
            self.consume(Token::Semicolon)?;
        }

        Ok((statement, consumes_semicolon))
    }

    fn parse_use(&mut self) -> Result<NodeId, Error> {
        let import_span = self.consume(Token::Use)?;

        let path = Vec::new();

        while !self.at_end() {
            let name = self.parse_name()?;

            if self.peek_token() != Token::Dot {
                break;
            }

            self.consume(Token::Dot)?;
        }

        if self.peek_token() != Token::Colon {
            let bindings = Vec::new();
            let span = import_span.merge(self.peek_span());

            return Ok(self.ast.use_(path, bindings, span));
        }

        self.consume(Token::Colon)?;

        let bindings = self.parse_comma_separator(Self::parse_name, Token::Semicolon)?;
        let span = import_span.merge(self.peek_span());

        Ok(self.ast.use_(path, bindings, span))
    }

    fn parse_return(&mut self) -> Result<NodeId, Error> {
        let span = self.consume(Token::Return)?;

        let expression = self.parse_expression()?;

        let span = span.merge(self.ast.span(expression));

        Ok(self.ast.return_(expression, span))
    }

    fn parse_continue(&mut self) -> Result<NodeId, Error> {
        let span = self.consume(Token::Continue)?;

        Ok(self.ast.continue_(span))
    }

    fn parse_break(&mut self) -> Result<NodeId, Error> {
        let span = self.consume(Token::Break)?;

        Ok(self.ast.break_(span))
    }

    fn parse_block(&mut self) -> Result<NodeId, Error> {
        let lbrace_span = self.consume(Token::LeftBrace)?;

        let mut statements = Vec::new();

        while !self.at_end() && self.peek_token() != Token::RightBrace {
            let statement = self.parse_statement()?;

            statements.push(statement);
        }

        let tail = if let Some((_, false)) = statements.last() {
            statements.pop().map(|(statement, _)| statement)
        } else {
            None
        };

        let statements = statements
            .into_iter()
            .map(|(statement, _)| statement)
            .collect();

        let rbrace_span = self.consume(Token::RightBrace)?;

        let span = lbrace_span.merge(rbrace_span);

        Ok(self.ast.block(statements, tail, span))
    }

    fn parse_if(&mut self) -> Result<NodeId, Error> {
        let if_span = self.consume(Token::If)?;

        let condition = self.parse_expression()?;
        let then_branch = self.parse_block()?;

        if self.peek_token() != Token::Else {
            let span = if_span.merge(self.ast.span(then_branch));

            return Ok(self.ast.if_(condition, then_branch, None, span));
        }

        self.advance_token();

        if self.peek_token() == Token::If {
            let else_branch = self.parse_if()?;

            let span = if_span.merge(self.ast.span(else_branch));

            return Ok(self
                .ast
                .if_(condition, then_branch, Some(else_branch), span));
        }

        let else_branch = self.parse_block()?;
        let span = if_span.merge(self.ast.span(else_branch));

        Ok(self
            .ast
            .if_(condition, then_branch, Some(else_branch), span))
    }

    fn parse_while_loop(&mut self) -> Result<NodeId, Error> {
        let while_span = self.consume(Token::While)?;

        let condition = self.parse_expression()?;
        let block = self.parse_block()?;

        let span = while_span.merge(self.ast.span(block));

        Ok(self.ast.while_loop(condition, block, span))
    }

    fn parse_for_loop(&mut self) -> Result<NodeId, Error> {
        todo!()
    }

    fn parse_function(&mut self) -> Result<NodeId, Error> {
        let function_span = self.consume(Token::Function)?;

        let name = self.parse_name()?;

        self.consume(Token::LeftParen)?;

        let parameters = self.parse_comma_separator(Self::parse_name, Token::RightParen)?;

        self.consume(Token::RightParen)?;

        let block = self.parse_block()?;

        let span = function_span.merge(self.ast.span(block));

        Ok(self.ast.function(name, parameters, block, span))
    }

    fn parse_lambda(&mut self) -> Result<NodeId, Error> {
        let pipe_span = self.consume(Token::Pipe)?;

        let parameters = self.parse_comma_separator(Self::parse_name, Token::Pipe)?;

        self.consume(Token::Pipe)?;

        let block = match self.peek_token() {
            Token::LeftBrace => self.parse_block()?,
            _ => self.parse_expression()?,
        };

        let span = pipe_span.merge(self.ast.span(block));

        Ok(self.ast.lambda(parameters, block, span))
    }

    fn parse_variable(&mut self) -> Result<NodeId, Error> {
        let let_span = self.consume(Token::Let)?;

        let left = self.parse_name()?;

        self.consume(Token::Assign)?;

        let right = self.parse_expression()?;

        let span = let_span.merge(self.ast.span(right));

        Ok(self.ast.variable(left, right, span))
    }

    fn parse_constant(&mut self) -> Result<NodeId, Error> {
        let let_span = self.consume(Token::Const)?;

        let left = self.parse_name()?;

        self.consume(Token::Assign)?;

        let right = self.parse_expression()?;

        let span = let_span.merge(self.ast.span(right));

        Ok(self.ast.constant(left, right, span))
    }

    fn parse_ref(&mut self) -> Result<NodeId, Error> {
        let ref_span = self.consume(Token::Ref)?;

        let left = self.parse_name()?;

        self.consume(Token::Assign)?;

        let right = self.parse_expression()?;

        let span = ref_span.merge(self.ast.span(right));

        Ok(self.ast.ref_(left, right, span))
    }

    fn parse_assign(&mut self) -> Result<NodeId, Error> {
        let left = self.parse_or()?;

        let token = self.peek_token();

        let operator = match token {
            Token::AddAssign => BinaryOp::Add,
            Token::SubtractAssign => BinaryOp::Subtract,
            Token::MultiplyAssign => BinaryOp::Multiply,
            Token::DivideAssign => BinaryOp::Divide,
            Token::ModuloAssign => BinaryOp::Modulo,
            Token::Assign => {
                self.advance_token();

                let right = self.parse_or()?;

                let span = self.ast.span(left).merge(self.ast.span(right));

                return Ok(self.ast.assign(left, right, span));
            }
            _ => return Ok(left),
        };

        self.advance_token();

        let right = self.parse_or()?;

        let span = self.ast.span(left).merge(self.ast.span(right));

        let right = self.ast.binary(operator, left, right, span);

        Ok(self.ast.assign(left, right, span))
    }

    fn parse_or(&mut self) -> Result<NodeId, Error> {
        let mut left = self.parse_and()?;

        while !self.at_end() {
            let token = self.peek_token();

            let Token::Or = token else {
                break;
            };

            self.advance_token();

            let right = self.parse_and()?;
            let span = self.ast.span(left).merge(self.ast.span(right));

            left = self.ast.logical_or(left, right, span);
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<NodeId, Error> {
        let mut left = self.parse_equality()?;

        while !self.at_end() {
            let token = self.peek_token();

            let Token::And = token else {
                break;
            };

            self.advance_token();

            let right = self.parse_equality()?;
            let span = self.ast.span(left).merge(self.ast.span(right));

            left = self.ast.logical_and(left, right, span);
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<NodeId, Error> {
        let mut left = self.parse_comparison()?;

        while !self.at_end() {
            let token = self.peek_token();

            let operator = match token {
                Token::Equal => BinaryOp::Equal,
                Token::NotEqual => BinaryOp::NotEqual,
                _ => break,
            };

            self.advance_token();

            let right = self.parse_comparison()?;
            let span = self.ast.span(left).merge(self.ast.span(right));

            left = self.ast.binary(operator, left, right, span);
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<NodeId, Error> {
        let mut left = self.parse_term()?;

        while !self.at_end() {
            let token = self.peek_token();

            let operator = match token {
                Token::Greater => BinaryOp::Greater,
                Token::GreaterEqual => BinaryOp::GreaterEqual,
                Token::Less => BinaryOp::Less,
                Token::LessEqual => BinaryOp::LessEqual,
                _ => break,
            };

            self.advance_token();

            let right = self.parse_term()?;
            let span = self.ast.span(left).merge(self.ast.span(right));

            left = self.ast.binary(operator, left, right, span);
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<NodeId, Error> {
        let mut left = self.parse_factor()?;

        while !self.at_end() {
            let token = self.peek_token();

            let operator = match token {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Subtract,
                _ => break,
            };

            self.advance_token();

            let right = self.parse_factor()?;
            let span = self.ast.span(left).merge(self.ast.span(right));

            left = self.ast.binary(operator, left, right, span);
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<NodeId, Error> {
        let mut left = self.parse_prefix_unary()?;

        while !self.at_end() {
            let token = self.peek_token();

            let operator = match token {
                Token::Multiply => BinaryOp::Multiply,
                Token::Divide => BinaryOp::Divide,
                Token::Modulo => BinaryOp::Modulo,
                _ => break,
            };

            self.advance_token();

            let right = self.parse_prefix_unary()?;
            let span = self.ast.span(left).merge(self.ast.span(right));

            left = self.ast.binary(operator, left, right, span);
        }

        Ok(left)
    }

    fn parse_prefix_unary(&mut self) -> Result<NodeId, Error> {
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
            Token::Caret => UnaryOp::Deref,
            _ => {
                let primary = self.parse_primary()?;

                return Ok(primary);
            }
        };

        self.advance_token();

        let operand = self.parse_prefix_unary()?;

        let span = span.merge(self.ast.span(operand));

        Ok(self.ast.unary(operator, operand, span))
    }

    fn parse_primary(&mut self) -> Result<NodeId, Error> {
        let (token, span) = self.peek();

        let primary = match token {
            Token::If => self.parse_if()?,
            Token::Pipe => self.parse_lambda()?,
            Token::LeftParen => {
                self.consume(Token::LeftParen)?;
                let expression = self.parse_expression()?;
                self.consume(Token::RightParen)?;

                expression
            }
            Token::LeftBrace => self.parse_block()?,
            Token::NumberLiteral => {
                let span = self.peek_span();
                let lexeme = self.lexeme(span);

                let value = match lexeme.parse::<f64>() {
                    Ok(value) => Ok(value),
                    Err(..) => report_error!(span, self.compiler.path, "failed to parse float"),
                }?;

                self.advance_token();

                self.ast.number(value, span)
            }
            Token::True => {
                self.advance_token();

                self.ast.boolean(true, span)
            }
            Token::False => {
                self.advance_token();

                self.ast.boolean(false, span)
            }
            Token::Nil => {
                self.advance_token();

                self.ast.nil(span)
            }
            Token::StringLiteral => {
                let span = self.peek_span();
                let lexeme = self.lexeme(span);
                let index = INTERNER
                    .lock()
                    .unwrap()
                    .get_or_intern(&lexeme[1..lexeme.len() - 1]);

                self.advance_token();

                self.ast.string(index, span)
            }
            Token::Identifier => {
                let identifier = self.parse_identifier()?;

                self.parse_postfix_unary(identifier)?
            }
            Token::Hash => self.parse_map_literal()?,
            _ => {
                let span = self.peek_span();

                return report_error!(
                    span,
                    self.compiler.path,
                    "expected a <operand> and found: {}",
                    token
                );
            }
        };

        Ok(primary)
    }

    fn parse_name(&mut self) -> Result<Spanned<Symbol>, Error> {
        let span = self.peek_span();
        let lexeme = self.lexeme(span);
        let symbol = INTERNER.lock().unwrap().get_or_intern(lexeme);

        self.consume(Token::Identifier)?;

        Ok(Spanned::new(symbol, span))
    }

    fn parse_identifier(&mut self) -> Result<NodeId, Error> {
        let name = self.parse_name()?;

        Ok(self.ast.identifier(name))
    }

    fn parse_map_entry(&mut self) -> Result<(NodeId, Option<NodeId>), Error> {
        let key = self.parse_expression()?;

        if self.peek_token() == Token::Colon {
            self.consume(Token::Colon)?;
            let value = self.parse_expression()?;

            return Ok((key, Some(value)));
        }

        Ok((key, None))
    }

    fn parse_map_literal(&mut self) -> Result<NodeId, Error> {
        let hash_span = self.consume(Token::Hash)?;
        self.consume(Token::LeftBrace)?;

        let entries = self.parse_comma_separator(Self::parse_map_entry, Token::RightBrace)?;

        let rbrace_span = self.consume(Token::RightBrace)?;

        let span = hash_span.merge(rbrace_span);

        Ok(self.ast.map(entries, span))
    }

    fn parse_postfix_unary(&mut self, operand: NodeId) -> Result<NodeId, Error> {
        let token = self.peek_token();

        Ok(match token {
            Token::LeftParen => self.parse_function_call(operand)?,
            Token::Dot => self.parse_member_access(operand)?,
            _ => operand,
        })
    }

    fn parse_function_call(&mut self, callee: NodeId) -> Result<NodeId, Error> {
        let lparen_span = self.consume(Token::LeftParen)?;

        let arguments = self.parse_comma_separator(Self::parse_expression, Token::RightParen)?;

        let rparen_span = self.consume(Token::RightParen)?;

        let span = lparen_span.merge(rparen_span);

        let function_call = self.ast.function_call(callee, arguments, span);

        self.parse_postfix_unary(function_call)
    }

    fn parse_member_access(&mut self, object: NodeId) -> Result<NodeId, Error> {
        let dot_span = self.consume(Token::Dot)?;

        let property = self.parse_name()?;

        let span = dot_span.merge(property.span);

        let member_access = self.ast.member_access(object, property, span);

        self.parse_postfix_unary(member_access)
    }
}
