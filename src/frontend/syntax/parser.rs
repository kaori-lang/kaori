use crate::{
    error::kaori_error::KaoriError,
    frontend::scanner::{span::Span, token_kind::TokenKind, token_stream::TokenStream},
    kaori_error,
};

use super::{
    ast_node::AstNode,
    decl::{Decl, Parameter},
    expr::Expr,
    operator::{BinaryOp, UnaryOp},
    stmt::Stmt,
    ty::Ty,
};

pub struct Parser {
    token_stream: TokenStream,
}

impl Parser {
    pub fn new(token_stream: TokenStream) -> Self {
        Self { token_stream }
    }

    pub fn parse(&mut self) -> Result<Vec<Decl>, KaoriError> {
        let mut declarations = Vec::new();

        while !self.token_stream.at_end() {
            let declaration = self.parse_declaration()?;
            declarations.push(declaration);
        }

        Ok(declarations)
    }

    fn parse_declaration(&mut self) -> Result<Decl, KaoriError> {
        let declaration = match self.token_stream.token_kind() {
            TokenKind::Function => self.parse_function_declaration(),
            _ => Err(kaori_error!(
                self.token_stream.span(),
                "invalid declaration at global scope"
            )),
        }?;

        Ok(declaration)
    }

    fn parse_ast_node(&mut self) -> Result<AstNode, KaoriError> {
        let stmt = match self.token_stream.token_kind() {
            TokenKind::Print => self.parse_print_statement(),
            TokenKind::LeftBrace => self.parse_block_statement(),
            TokenKind::If => self.parse_if_statement(),
            TokenKind::While => self.parse_while_loop_statement(),
            TokenKind::For => self.parse_for_loop_statement(),
            TokenKind::Break => self.parse_break_statement(),
            TokenKind::Continue => self.parse_continue_statement(),
            TokenKind::Return => self.parse_return_statement(),
            _ => {
                if self
                    .token_stream
                    .look_ahead(&[TokenKind::Identifier, TokenKind::Colon])
                {
                    let declaration = self.parse_variable_declaration()?;
                    self.token_stream.consume(TokenKind::Semicolon)?;
                    return Ok(AstNode::Declaration(declaration));
                } else {
                    let statement = self.parse_expression_statement();
                    self.token_stream.consume(TokenKind::Semicolon)?;

                    statement
                }
            }
        }?;

        Ok(AstNode::Statement(stmt))
    }

    /* Declarations */
    fn parse_variable_declaration(&mut self) -> Result<Decl, KaoriError> {
        let span = self.token_stream.span();
        let name = self.token_stream.lexeme().to_owned();

        self.token_stream.consume(TokenKind::Identifier)?;
        self.token_stream.consume(TokenKind::Colon)?;

        let ty = self.parse_type()?;

        self.token_stream.consume(TokenKind::Assign)?;

        let right = self.parse_expression()?;

        Ok(Decl::variable(name, right, ty, span))
    }

    fn parse_function_declaration(&mut self) -> Result<Decl, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::Function)?;

        let name = self.token_stream.lexeme().to_owned();

        self.token_stream.consume(TokenKind::Identifier)?;

        self.token_stream.consume(TokenKind::LeftParen)?;

        let mut parameters: Vec<Parameter> = Vec::new();

        while !self.token_stream.at_end() && self.token_stream.token_kind() != TokenKind::RightParen
        {
            let span = self.token_stream.span();
            let name = self.token_stream.lexeme().to_owned();
            self.token_stream.consume(TokenKind::Identifier)?;
            self.token_stream.consume(TokenKind::Colon)?;

            let ty = self.parse_type()?;

            let parameter = Parameter { name, ty, span };

            parameters.push(parameter);

            if self.token_stream.token_kind() == TokenKind::RightParen {
                break;
            }

            self.token_stream.consume(TokenKind::Comma)?;
        }

        self.token_stream.consume(TokenKind::RightParen)?;

        let mut return_type = Ty::Void;

        if self.token_stream.token_kind() == TokenKind::ThinArrow {
            self.token_stream.consume(TokenKind::ThinArrow)?;
            return_type = self.parse_type()?;
        }

        let mut body = Vec::new();

        self.token_stream.consume(TokenKind::LeftBrace)?;

        while !self.token_stream.at_end() && self.token_stream.token_kind() != TokenKind::RightBrace
        {
            let node = self.parse_ast_node()?;
            body.push(node);
        }

        self.token_stream.consume(TokenKind::RightBrace)?;

        Ok(Decl::function(name, parameters, body, return_type, span))
    }

    /* Statements */
    fn parse_return_statement(&mut self) -> Result<Stmt, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::Return)?;

        if self.token_stream.token_kind() == TokenKind::Semicolon {
            let expression = None;
            self.token_stream.consume(TokenKind::Semicolon)?;
            return Ok(Stmt::return_(expression, span));
        }

        let expression = Some(self.parse_expression()?);

        self.token_stream.consume(TokenKind::Semicolon)?;

        Ok(Stmt::return_(expression, span))
    }

    fn parse_continue_statement(&mut self) -> Result<Stmt, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::Continue)?;
        self.token_stream.consume(TokenKind::Semicolon)?;

        Ok(Stmt::continue_(span))
    }

    fn parse_break_statement(&mut self) -> Result<Stmt, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::Break)?;
        self.token_stream.consume(TokenKind::Semicolon)?;

        Ok(Stmt::break_(span))
    }

    fn parse_expression_statement(&mut self) -> Result<Stmt, KaoriError> {
        let span = self.token_stream.span();
        let expression = self.parse_expression()?;

        Ok(Stmt::expression(expression, span))
    }

    fn parse_print_statement(&mut self) -> Result<Stmt, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::Print)?;
        self.token_stream.consume(TokenKind::LeftParen)?;
        let expression = self.parse_expression()?;
        self.token_stream.consume(TokenKind::RightParen)?;
        self.token_stream.consume(TokenKind::Semicolon)?;

        Ok(Stmt::print(expression, span))
    }

    fn parse_block_statement(&mut self) -> Result<Stmt, KaoriError> {
        let span = self.token_stream.span();

        let mut nodes: Vec<AstNode> = Vec::new();

        self.token_stream.consume(TokenKind::LeftBrace)?;

        while !self.token_stream.at_end() && self.token_stream.token_kind() != TokenKind::RightBrace
        {
            let node = self.parse_ast_node()?;
            nodes.push(node);
        }

        self.token_stream.consume(TokenKind::RightBrace)?;

        Ok(Stmt::block(nodes, span))
    }

    fn parse_if_statement(&mut self) -> Result<Stmt, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::If)?;

        let condition = self.parse_expression()?;

        let then_branch = self.parse_block_statement()?;

        if self.token_stream.token_kind() != TokenKind::Else {
            return Ok(Stmt::if_(condition, then_branch, None, span));
        }

        self.token_stream.advance();

        if self.token_stream.token_kind() == TokenKind::If {
            let else_branch = Some(self.parse_if_statement()?);

            return Ok(Stmt::if_(condition, then_branch, else_branch, span));
        }

        let else_branch = Some(self.parse_block_statement()?);

        Ok(Stmt::if_(condition, then_branch, else_branch, span))
    }

    fn parse_while_loop_statement(&mut self) -> Result<Stmt, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::While)?;

        let condition = self.parse_expression()?;
        let block = self.parse_block_statement()?;

        Ok(Stmt::while_loop(condition, block, span))
    }

    fn parse_for_loop_statement(&mut self) -> Result<Stmt, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::For)?;

        let declaration = self.parse_variable_declaration()?;

        self.token_stream.consume(TokenKind::Semicolon)?;

        let condition = self.parse_expression()?;

        self.token_stream.consume(TokenKind::Semicolon)?;

        let increment = self.parse_expression_statement()?;

        let block = self.parse_block_statement()?;

        Ok(Stmt::for_loop(
            declaration,
            condition,
            increment,
            block,
            span,
        ))
    }

    /* Exprs */
    fn parse_expression(&mut self) -> Result<Expr, KaoriError> {
        if self
            .token_stream
            .look_ahead(&[TokenKind::Identifier, TokenKind::Assign])
        {
            return self.parse_assign();
        }

        self.parse_or()
    }

    fn parse_assign(&mut self) -> Result<Expr, KaoriError> {
        let left = self.parse_identifier()?;

        let span = self.token_stream.span();
        self.token_stream.consume(TokenKind::Assign)?;

        let right = self.parse_expression()?;

        Ok(Expr::assign(left, right, span))
    }

    fn parse_or(&mut self) -> Result<Expr, KaoriError> {
        let mut left = self.parse_and()?;

        while !self.token_stream.at_end() {
            let kind = self.token_stream.token_kind();
            let span = self.token_stream.span();

            let operator = match kind {
                TokenKind::Or => BinaryOp::Or,
                _ => break,
            };

            self.token_stream.advance();

            let right = self.parse_and()?;

            left = Expr::binary(operator, left, right, span);
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, KaoriError> {
        let mut left = self.parse_equality()?;

        while !self.token_stream.at_end() {
            let kind = self.token_stream.token_kind();
            let span = self.token_stream.span();

            let operator = match kind {
                TokenKind::And => BinaryOp::And,
                _ => break,
            };

            self.token_stream.advance();
            let right = self.parse_equality()?;

            left = Expr::binary(operator, left, right, span);
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr, KaoriError> {
        let mut left = self.parse_comparison()?;

        while !self.token_stream.at_end() {
            let kind = self.token_stream.token_kind();
            let span = self.token_stream.span();

            let operator = match kind {
                TokenKind::Equal => BinaryOp::Equal,
                TokenKind::NotEqual => BinaryOp::NotEqual,
                _ => break,
            };

            self.token_stream.advance();
            let right = self.parse_comparison()?;

            left = Expr::binary(operator, left, right, span);
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, KaoriError> {
        let mut left = self.parse_term()?;

        while !self.token_stream.at_end() {
            let kind = self.token_stream.token_kind();
            let span = self.token_stream.span();

            let operator = match kind {
                TokenKind::Greater => BinaryOp::Greater,
                TokenKind::GreaterEqual => BinaryOp::GreaterEqual,
                TokenKind::Less => BinaryOp::Less,
                TokenKind::LessEqual => BinaryOp::LessEqual,
                _ => break,
            };

            self.token_stream.advance();
            let right = self.parse_term()?;

            left = Expr::binary(operator, left, right, span);
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, KaoriError> {
        let mut left = self.parse_factor()?;

        while !self.token_stream.at_end() {
            let kind = self.token_stream.token_kind();
            let span = self.token_stream.span();

            let operator = match kind {
                TokenKind::Plus => BinaryOp::Add,
                TokenKind::Minus => BinaryOp::Subtract,
                _ => break,
            };

            self.token_stream.advance();
            let right = self.parse_factor()?;

            left = Expr::binary(operator, left, right, span);
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, KaoriError> {
        let mut left = self.parse_prefix_unary()?;

        while !self.token_stream.at_end() {
            let kind = self.token_stream.token_kind();
            let span = self.token_stream.span();

            let operator = match kind {
                TokenKind::Multiply => BinaryOp::Multiply,
                TokenKind::Divide => BinaryOp::Divide,
                TokenKind::Modulo => BinaryOp::Modulo,
                _ => break,
            };

            self.token_stream.advance();
            let right = self.parse_prefix_unary()?;

            left = Expr::binary(operator, left, right, span);
        }

        Ok(left)
    }

    fn parse_prefix_unary(&mut self) -> Result<Expr, KaoriError> {
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

    fn parse_primary(&mut self) -> Result<Expr, KaoriError> {
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

                return Err(kaori_error!(span, "{:?} is a invalid operand", kind));
            }
        })
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
        let span = self.token_stream.span();

        Ok(match kind {
            TokenKind::Increment => {
                self.token_stream.advance();
                Expr::increment(identifier, span)
            }
            TokenKind::Decrement => {
                self.token_stream.advance();
                Expr::decrement(identifier, span)
            }
            TokenKind::LeftParen => self.parse_function_call(identifier)?,
            _ => identifier,
        })
    }

    fn parse_function_call(&mut self, callee: Expr) -> Result<Expr, KaoriError> {
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

        let span = Span {
            start: callee.span.start,
            end: self.token_stream.span().end,
        };

        self.token_stream.consume(TokenKind::RightParen)?;

        self.parse_function_call(Expr::function_call(callee, arguments, span))
    }

    /* Types */
    pub fn parse_type(&mut self) -> Result<Ty, KaoriError> {
        match self.token_stream.token_kind() {
            TokenKind::Identifier => self.parse_primitive_type(),
            _ => Err(kaori_error!(
                self.token_stream.span(),
                "invalid type annotation: {:?}",
                self.token_stream.token_kind(),
            )),
        }
    }

    fn parse_primitive_type(&mut self) -> Result<Ty, KaoriError> {
        let sub = self.token_stream.lexeme();

        let primitive = match sub {
            "bool" => Ty::Boolean,
            "str" => Ty::String,
            "num" => Ty::Number,
            _ => {
                return Err(kaori_error!(
                    self.token_stream.span(),
                    "invalid primitive type: {:?}",
                    self.token_stream.token_kind(),
                ));
            }
        };

        self.token_stream.advance();

        Ok(primitive)
    }
}
