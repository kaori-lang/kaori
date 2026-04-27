use crate::{
    ast::{
        expr::Expr,
        ops::{AssignOp, BinaryOp, UnaryOp},
    },
    error::kaori_error::KaoriError,
    kaori_error,
    lexer::{span::Span, token_kind::TokenKind, token_stream::TokenStream},
};

pub struct Parser<'a> {
    pub token_stream: TokenStream<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(token_stream: TokenStream<'a>) -> Self {
        Self { token_stream }
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>, KaoriError> {
        let mut functions = Vec::new();

        while !self.token_stream.at_end() {
            let function = self.parse_function()?;

            functions.push(function);
        }

        Ok(functions)
    }

    fn parse_expression_statement(&mut self) -> Result<Expr, KaoriError> {
        let token_kind = self.token_stream.token_kind();

        let expression = match token_kind {
            TokenKind::Print => self.parse_print(),
            TokenKind::If => self.parse_if(),
            TokenKind::While => self.parse_while_loop(),
            TokenKind::For => self.parse_for_loop(),
            TokenKind::Break => self.parse_break(),
            TokenKind::Continue => self.parse_continue(),
            TokenKind::Return => self.parse_return(),
            TokenKind::Unchecked => self.parse_unchecked_block(),
            _ => {
                let expression = self.parse_expression();

                if expression.is_ok() {
                    self.token_stream.consume(TokenKind::Semicolon)?;
                    expression
                } else {
                    expression
                }
            }
        }?;

        match token_kind {
            TokenKind::Print | TokenKind::Break | TokenKind::Continue | TokenKind::Return => {
                self.token_stream.consume(TokenKind::Semicolon)?;
            }
            _ => (),
        };

        Ok(expression)
    }

    fn parse_comma_separator<T>(
        &mut self,
        parse_item: fn(&mut Self) -> Result<T, KaoriError>,
        terminator: TokenKind,
    ) -> Result<Vec<T>, KaoriError> {
        let mut items = Vec::new();

        while !self.token_stream.at_end() && self.token_stream.token_kind() != terminator {
            let item = parse_item(self)?;
            items.push(item);

            if self.token_stream.token_kind() == terminator {
                break;
            }

            self.token_stream.consume(TokenKind::Comma)?;
        }

        Ok(items)
    }

    fn parse_return(&mut self) -> Result<Expr, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::Return)?;

        if self.token_stream.token_kind() == TokenKind::Semicolon {
            return Ok(Expr::return_(None, span));
        }

        let expression = Some(self.parse_expression()?);

        Ok(Expr::return_(expression, span))
    }

    fn parse_continue(&mut self) -> Result<Expr, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::Continue)?;

        Ok(Expr::continue_(span))
    }

    fn parse_break(&mut self) -> Result<Expr, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::Break)?;

        Ok(Expr::break_(span))
    }

    fn parse_print(&mut self) -> Result<Expr, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::Print)?;
        self.token_stream.consume(TokenKind::LeftParen)?;
        let expression = self.parse_expression()?;
        self.token_stream.consume(TokenKind::RightParen)?;

        Ok(Expr::print(expression, span))
    }

    fn parse_block(&mut self) -> Result<Expr, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::LeftBrace)?;

        let mut expressions = Vec::new();

        while !self.token_stream.at_end() && self.token_stream.token_kind() != TokenKind::RightBrace
        {
            let expression = self.parse_expression_statement()?;

            expressions.push(expression);
        }

        let tail = expressions.pop();

        self.token_stream.consume(TokenKind::RightBrace)?;

        Ok(Expr::block(expressions, tail, span))
    }

    fn parse_unchecked_block(&mut self) -> Result<Expr, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::Unchecked)?;
        self.token_stream.consume(TokenKind::LeftBrace)?;

        let mut expressions = Vec::new();

        while !self.token_stream.at_end() && self.token_stream.token_kind() != TokenKind::RightBrace
        {
            let expression = self.parse_expression_statement()?;

            expressions.push(expression);
        }

        let tail = expressions.pop();

        self.token_stream.consume(TokenKind::RightBrace)?;

        Ok(Expr::unchecked_block(expressions, tail, span))
    }

    fn parse_if(&mut self) -> Result<Expr, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::If)?;

        let condition = self.parse_expression()?;
        let then_branch = self.parse_block()?;

        if self.token_stream.token_kind() != TokenKind::Else {
            return Ok(Expr::if_(condition, then_branch, None, span));
        }

        self.token_stream.advance();

        if self.token_stream.token_kind() == TokenKind::If {
            let else_branch = Some(self.parse_if()?);
            return Ok(Expr::if_(condition, then_branch, else_branch, span));
        }

        let else_branch = Some(self.parse_block()?);

        Ok(Expr::if_(condition, then_branch, else_branch, span))
    }

    fn parse_while_loop(&mut self) -> Result<Expr, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::While)?;

        let condition = self.parse_expression()?;
        let block = self.parse_block()?;

        Ok(Expr::while_loop(condition, block, span))
    }

    fn parse_for_loop(&mut self) -> Result<Expr, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::For)?;

        let start = self.parse_expression()?;

        let down_to = match self.token_stream.token_kind() {
            TokenKind::To => false,
            TokenKind::DownTo => true,
            _ => {
                return Err(kaori_error!(
                    span,
                    "expected {} or {} and found {}",
                    TokenKind::To,
                    TokenKind::DownTo,
                    self.token_stream.token_kind(),
                ));
            }
        };

        self.token_stream.advance();

        let end = self.parse_expression()?;

        let block = self.parse_block()?;

        Ok(Expr::for_loop(start, end, block, span))
    }

    fn parse_function(&mut self) -> Result<Expr, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::Function)?;

        let name = self.token_stream.lexeme().to_owned();

        self.token_stream.consume(TokenKind::Identifier)?;

        self.token_stream.consume(TokenKind::LeftParen)?;

        let parameters =
            self.parse_comma_separator(Self::parse_identifier, TokenKind::RightParen)?;

        self.token_stream.consume(TokenKind::RightParen)?;

        let captures = if self.token_stream.token_kind() == TokenKind::Pipe {
            let captures = self.parse_comma_separator(Self::parse_identifier, TokenKind::Pipe)?;
            self.token_stream.consume(TokenKind::Pipe);
            captures
        } else {
            Vec::new()
        };

        let mut body = Vec::new();

        self.token_stream.consume(TokenKind::LeftBrace)?;

        while !self.token_stream.at_end() && self.token_stream.token_kind() != TokenKind::RightBrace
        {
            let statement = self.parse_expression_statement()?;
            body.push(statement);
        }

        self.token_stream.consume(TokenKind::RightBrace)?;

        Ok(Expr::function(name, parameters, captures, body, span))
    }

    fn build_binary_operator(&mut self) -> BinaryOp {
        let token_kind = self.token_stream.token_kind();

        match token_kind {
            TokenKind::Plus => BinaryOp::Add,
            TokenKind::Minus => BinaryOp::Subtract,
            TokenKind::Multiply => BinaryOp::Multiply,
            TokenKind::Divide => BinaryOp::Divide,
            TokenKind::Modulo => BinaryOp::Modulo,
            TokenKind::Equal => BinaryOp::Equal,
            TokenKind::NotEqual => BinaryOp::NotEqual,
            TokenKind::Greater => BinaryOp::Greater,
            TokenKind::GreaterEqual => BinaryOp::GreaterEqual,
            TokenKind::Less => BinaryOp::Less,
            TokenKind::LessEqual => BinaryOp::LessEqual,
            _ => unreachable!(),
        }
    }

    fn build_unary_operator(&mut self) -> UnaryOp {
        let token_kind = self.token_stream.token_kind();

        match token_kind {
            TokenKind::Minus => UnaryOp::Negate,
            _ => unreachable!(),
        }
    }

    fn parse_expression(&mut self) -> Result<Expr, KaoriError> {
        let assign = self.parse_assign()?;

        Ok(assign)
    }

    fn parse_assign(&mut self) -> Result<Expr, KaoriError> {
        let left = self.parse_or()?;

        let token_kind = self.token_stream.token_kind();

        let operator = match token_kind {
            TokenKind::Assign => AssignOp::Assign,
            TokenKind::AddAssign => AssignOp::AddAssign,
            TokenKind::SubtractAssign => AssignOp::SubtractAssign,
            TokenKind::MultiplyAssign => AssignOp::MultiplyAssign,
            TokenKind::DivideAssign => AssignOp::DivideAssign,
            TokenKind::ModuloAssign => AssignOp::ModuloAssign,
            TokenKind::DeclareAssign => {
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
        let mut left = self.parse_prefix_unary()?;

        while !self.token_stream.at_end() {
            let token_kind = self.token_stream.token_kind();

            let operator = match token_kind {
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
            TokenKind::If => self.parse_if()?,
            TokenKind::Function => self.parse_function()?,
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
            self.parse_comma_separator(Self::parse_dict_literal_field, TokenKind::RightBrace)?;

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
            self.parse_comma_separator(Self::parse_expression, TokenKind::RightParen)?;

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
