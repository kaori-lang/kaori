use std::ops::Range;

use crate::{
    diagnostics::error::Error,
    interpreter::INTERNER,
    report_error,
    syntax::{
        ast::{Ast, Expr, ExprId, Spanned, Stmt, StmtId},
        ops::{BinaryOp, CompoundOp, UnaryOp},
        token::{Span, Token},
    },
    util::string_interner::Symbol,
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

    fn expr_requires_semicolon(&self, id: ExprId) -> bool {
        !matches!(
            self.ast.expr(id).node,
            Expr::Block { .. } | Expr::If { .. } | Expr::Function { .. }
        )
    }

    pub fn parse(mut self) -> Result<Ast, Error> {
        let mut statements = Vec::new();

        while !self.at_end() {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        self.ast.stmt_block(statements, Span::default());

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

    fn parse_statement(&mut self) -> Result<StmtId, Error> {
        let token = self.peek_token();

        match token {
            Token::Native => self.parse_native_function(),
            Token::While => self.parse_while_loop(),
            Token::For => self.parse_for_loop(),
            Token::Break => self.parse_break(),
            Token::Continue => self.parse_continue(),
            Token::Return => self.parse_return(),
            Token::Let => self.parse_variable(),
            Token::Mut => self.parse_mut(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression_statement(&mut self) -> Result<StmtId, Error> {
        let token = self.peek_token();

        let expr = match token {
            Token::Function => self.parse_function()?,
            Token::If => self.parse_if()?,
            _ => {
                let expr = self.parse_expression()?;

                if self.expr_requires_semicolon(expr) {
                    self.consume(Token::Semicolon)?;
                } else if self.peek_token() == Token::Semicolon {
                    self.advance_token();
                }

                let span = self.ast.expr(expr).span;

                return Ok(self.ast.stmt_expr(expr, span));
            }
        };

        if self.peek_token() == Token::Semicolon {
            self.advance_token();
        }

        let span = self.ast.expr(expr).span;

        Ok(self.ast.stmt_expr(expr, span))
    }

    fn parse_expression(&mut self) -> Result<ExprId, Error> {
        self.parse_assign()
    }

    fn parse_return(&mut self) -> Result<StmtId, Error> {
        let start = self.consume(Token::Return)?;

        let expression = self.parse_expression()?;

        self.consume(Token::Semicolon)?;

        let span = start.merge(self.ast.expr(expression).span);

        Ok(self.ast.return_(expression, span))
    }

    fn parse_continue(&mut self) -> Result<StmtId, Error> {
        let span = self.consume(Token::Continue)?;

        self.consume(Token::Semicolon)?;

        Ok(self.ast.continue_(span))
    }

    fn parse_break(&mut self) -> Result<StmtId, Error> {
        let span = self.consume(Token::Break)?;

        self.consume(Token::Semicolon)?;

        Ok(self.ast.break_(span))
    }

    fn parse_stmt_block(&mut self) -> Result<StmtId, Error> {
        let lbrace_span = self.consume(Token::LeftBrace)?;

        let mut statements = Vec::new();

        while !self.at_end() && self.peek_token() != Token::RightBrace {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        let rbrace_span = self.consume(Token::RightBrace)?;

        let span = lbrace_span.merge(rbrace_span);

        Ok(self.ast.stmt_block(statements, span))
    }

    fn parse_block(&mut self) -> Result<ExprId, Error> {
        let lbrace_span = self.consume(Token::LeftBrace)?;

        let mut statements = Vec::new();
        let mut tail = None;

        while !self.at_end() && self.peek_token() != Token::RightBrace {
            let token = self.peek_token();

            match token {
                Token::Function => {
                    let expr = self.parse_function()?;
                    let span = self.ast.expr(expr).span;

                    if self.peek_token() == Token::Semicolon {
                        self.advance_token();
                        statements.push(self.ast.stmt_expr(expr, span));
                    } else {
                        tail = Some(expr);
                        break;
                    }
                }
                Token::If => {
                    let expr = self.parse_if()?;
                    let span = self.ast.expr(expr).span;

                    if self.peek_token() == Token::Semicolon {
                        self.advance_token();
                        statements.push(self.ast.stmt_expr(expr, span));
                    } else {
                        tail = Some(expr);
                        break;
                    }
                }
                Token::Native => {
                    statements.push(self.parse_native_function()?);
                }
                Token::While => {
                    statements.push(self.parse_while_loop()?);
                }
                Token::For => {
                    statements.push(self.parse_for_loop()?);
                }
                Token::Break => {
                    statements.push(self.parse_break()?);
                }
                Token::Continue => {
                    statements.push(self.parse_continue()?);
                }
                Token::Return => {
                    statements.push(self.parse_return()?);
                }
                Token::Let => {
                    statements.push(self.parse_variable()?);
                }
                Token::Mut => {
                    statements.push(self.parse_mut()?);
                }
                _ => {
                    let expr = self.parse_expression()?;
                    let span = self.ast.expr(expr).span;

                    if self.peek_token() == Token::Semicolon {
                        self.advance_token();
                        statements.push(self.ast.stmt_expr(expr, span));
                    } else if self.peek_token() == Token::RightBrace {
                        tail = Some(expr);
                        break;
                    } else {
                        return Err(report_error!(
                            span,
                            "expected `;` after expression, only the last expression in a block expression can omit it"
                        ));
                    }
                }
            }
        }

        let rbrace_span = self.consume(Token::RightBrace)?;

        let span = lbrace_span.merge(rbrace_span);

        Ok(self.ast.block(statements, tail, span))
    }

    fn parse_if(&mut self) -> Result<ExprId, Error> {
        let if_span = self.consume(Token::If)?;

        let condition = self.parse_expression()?;
        let then_branch = self.parse_block()?;

        if self.peek_token() != Token::Else {
            let span = if_span.merge(self.ast.expr(then_branch).span);

            return Ok(self.ast.if_(condition, then_branch, None, span));
        }

        self.advance_token();

        if self.peek_token() == Token::If {
            let else_branch = self.parse_if()?;
            let span = if_span.merge(self.ast.expr(else_branch).span);

            return Ok(self
                .ast
                .if_(condition, then_branch, Some(else_branch), span));
        }

        let else_branch = self.parse_block()?;
        let span = if_span.merge(self.ast.expr(else_branch).span);

        Ok(self
            .ast
            .if_(condition, then_branch, Some(else_branch), span))
    }

    fn parse_while_loop(&mut self) -> Result<StmtId, Error> {
        let while_span = self.consume(Token::While)?;

        let condition = self.parse_expression()?;
        let block = self.parse_stmt_block()?;

        let span = while_span.merge(self.ast.stmt(block).span);

        Ok(self.ast.while_loop(condition, block, span))
    }

    fn parse_for_loop(&mut self) -> Result<StmtId, Error> {
        todo!()
    }

    fn parse_native_function(&mut self) -> Result<StmtId, Error> {
        let start = self.consume(Token::Native)?;
        self.consume(Token::Function)?;

        let name = self.parse_name()?;

        self.consume(Token::LeftParen)?;

        let parameters = self.parse_comma_separator(Self::parse_name, Token::RightParen)?;

        let rparen_span = self.consume(Token::RightParen)?;

        self.consume(Token::Semicolon)?;

        let span = start.merge(rparen_span);

        Ok(self.ast.native_function(name, parameters, span))
    }

    fn parse_function(&mut self) -> Result<ExprId, Error> {
        let start = self.consume(Token::Function)?;

        let name = if self.peek_token() == Token::Identifier {
            Some(self.parse_name()?)
        } else {
            None
        };

        self.consume(Token::LeftParen)?;

        let parameters = self.parse_comma_separator(Self::parse_name, Token::RightParen)?;

        self.consume(Token::RightParen)?;

        let block = self.parse_stmt_block()?;

        let span = start.merge(self.ast.stmt(block).span);

        Ok(self.ast.function(name, parameters, block, span))
    }

    fn parse_variable(&mut self) -> Result<StmtId, Error> {
        let start = self.consume(Token::Let)?;

        let left = self.parse_name()?;

        self.consume(Token::Assign)?;

        let right = self.parse_expression()?;

        self.consume(Token::Semicolon)?;

        let span = start.merge(self.ast.expr(right).span);

        Ok(self.ast.variable(left, right, span))
    }

    fn parse_mut(&mut self) -> Result<StmtId, Error> {
        let start = self.consume(Token::Mut)?;

        let left = self.parse_name()?;

        self.consume(Token::Assign)?;

        let right = self.parse_expression()?;

        self.consume(Token::Semicolon)?;

        let span = start.merge(self.ast.expr(right).span);

        Ok(self.ast.mut_(left, right, span))
    }

    fn parse_assign(&mut self) -> Result<ExprId, Error> {
        let left = self.parse_or()?;

        let token = self.peek_token();

        let operator = match token {
            Token::AddAssign => CompoundOp::Add,
            Token::SubtractAssign => CompoundOp::Subtract,
            Token::MultiplyAssign => CompoundOp::Multiply,
            Token::DivideAssign => CompoundOp::Divide,
            Token::ModuloAssign => CompoundOp::Modulo,
            Token::Assign => {
                self.advance_token();

                let right = self.parse_or()?;
                let span = self.ast.expr(left).span.merge(self.ast.expr(right).span);

                return Ok(self.ast.assign(left, right, span));
            }
            _ => return Ok(left),
        };

        self.advance_token();

        let right = self.parse_or()?;
        let span = self.ast.expr(left).span.merge(self.ast.expr(right).span);

        Ok(self.ast.compound_assign(operator, left, right, span))
    }

    fn parse_or(&mut self) -> Result<ExprId, Error> {
        let mut left = self.parse_and()?;

        while !self.at_end() {
            let token = self.peek_token();

            let Token::Or = token else {
                break;
            };

            self.advance_token();

            let right = self.parse_and()?;
            let span = self.ast.expr(left).span.merge(self.ast.expr(right).span);

            left = self.ast.logical_or(left, right, span);
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<ExprId, Error> {
        let mut left = self.parse_equality()?;

        while !self.at_end() {
            let token = self.peek_token();

            let Token::And = token else {
                break;
            };

            self.advance_token();

            let right = self.parse_equality()?;
            let span = self.ast.expr(left).span.merge(self.ast.expr(right).span);

            left = self.ast.logical_and(left, right, span);
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<ExprId, Error> {
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
            let span = self.ast.expr(left).span.merge(self.ast.expr(right).span);

            left = self.ast.binary(operator, left, right, span);
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<ExprId, Error> {
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
            let span = self.ast.expr(left).span.merge(self.ast.expr(right).span);

            left = self.ast.binary(operator, left, right, span);
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<ExprId, Error> {
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
            let span = self.ast.expr(left).span.merge(self.ast.expr(right).span);

            left = self.ast.binary(operator, left, right, span);
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<ExprId, Error> {
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
            let span = self.ast.expr(left).span.merge(self.ast.expr(right).span);

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
                let span = span.merge(self.ast.expr(right).span);

                return Ok(self.ast.logical_not(right, span));
            }
            Token::Minus => UnaryOp::Negate,
            _ => {
                return self.parse_primary();
            }
        };

        self.advance_token();

        let right = self.parse_prefix_unary()?;
        let span = span.merge(self.ast.expr(right).span);

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
            Token::Nil => {
                self.advance_token();

                self.ast.nil_literal(span)
            }
            Token::StringLiteral => {
                let span = self.peek_span();
                let lexeme = self.lexeme(span);
                let index = INTERNER
                    .lock()
                    .unwrap()
                    .get_or_intern(&lexeme[1..lexeme.len() - 1]);

                self.advance_token();

                self.ast.string_literal(index, span)
            }
            Token::Identifier => {
                let identifier = self.parse_identifier()?;

                self.parse_postfix_unary(identifier)?
            }
            Token::LeftBrace => self.parse_dict_literal()?,
            _ => {
                return Err(report_error!(
                    span,
                    "expected a <operand> and found: {}",
                    token
                ));
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

    fn parse_identifier(&mut self) -> Result<ExprId, Error> {
        let name = self.parse_name()?;

        Ok(self.ast.identifier(name))
    }

    fn parse_dict_literal_field(&mut self) -> Result<(ExprId, ExprId), Error> {
        let key = self.parse_expression()?;
        self.consume(Token::Colon)?;
        let value = self.parse_expression()?;

        Ok((key, value))
    }

    fn parse_dict_literal(&mut self) -> Result<ExprId, Error> {
        let lbrace_span = self.consume(Token::LeftBrace)?;

        let fields =
            self.parse_comma_separator(Self::parse_dict_literal_field, Token::RightBrace)?;

        let rbrace_span = self.consume(Token::RightBrace)?;

        let span = lbrace_span.merge(rbrace_span);

        Ok(self.ast.dict_literal(fields, span))
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

        let rparen_span = self.consume(Token::RightParen)?;

        let span = self.ast.expr(callee).span.merge(rparen_span);

        let function_call = self.ast.function_call(callee, arguments, span);

        self.parse_postfix_unary(function_call)
    }

    fn parse_member_access(&mut self, object: ExprId) -> Result<ExprId, Error> {
        self.consume(Token::Dot)?;

        let property = self.parse_name()?;

        let span = self.ast.expr(object).span.merge(property.span);

        let member_access = self.ast.member_access(object, property, span);

        self.parse_postfix_unary(member_access)
    }
}
