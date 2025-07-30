use crate::{
    compiler::lexer::{token_kind::TokenKind, token_stream::TokenStream},
    error::kaori_error::KaoriError,
    kaori_error,
};

use super::{
    ast_node::ASTNode,
    declaration::{Decl, DeclKind},
    expression::Expr,
    operator::{BinaryOp, UnaryOp},
    r#type::Type,
    statement::{Stmt, StmtKind},
};

pub struct Parser {
    token_stream: TokenStream,
}

impl Parser {
    pub fn new(token_stream: TokenStream) -> Self {
        Self { token_stream }
    }

    pub fn declarations(&mut self) -> Result<Vec<ASTNode>, KaoriError> {
        let mut declarations: Vec<ASTNode> = Vec::new();

        while !self.token_stream.at_end() {
            let declaration = self.parse_declaration()?;
            declarations.push(declaration);
        }

        Ok(declarations)
    }

    /* Declarations */
    fn parse_declaration(&mut self) -> Result<ASTNode, KaoriError> {
        let declaration = match self.token_stream.token_kind() {
            TokenKind::Function => self.parse_function_declaration()?,
            _ => {
                if !self
                    .token_stream
                    .look_ahead(&[TokenKind::Identifier, TokenKind::Colon])
                {
                    return Ok(ASTNode::Statement(self.parse_statement()?));
                }

                let variable_declaration = self.parse_variable_declaration()?;
                self.token_stream.consume(TokenKind::Semicolon)?;
                variable_declaration
            }
        };

        Ok(ASTNode::Declaration(declaration))
    }

    fn parse_variable_declaration(&mut self) -> Result<Decl, KaoriError> {
        let span = self.token_stream.span();
        let name = self.token_stream.lexeme();

        self.token_stream.consume(TokenKind::Identifier)?;
        self.token_stream.consume(TokenKind::Colon)?;

        let type_annotation = self.parse_type()?;

        self.token_stream.consume(TokenKind::Assign)?;

        let right = self.parse_expr()?;

        return Ok(Decl::variable(name, right, type_annotation, span));
    }

    fn parse_function_declaration(&mut self) -> Result<Decl, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::Function)?;

        let name = self.token_stream.lexeme();

        self.token_stream.consume(TokenKind::Identifier);

        self.token_stream.consume(TokenKind::LeftParen);

        let mut parameters: Vec<Decl> = Vec::new();

        while !self.token_stream.at_end() && self.token_stream.token_kind() != TokenKind::RightParen
        {
            let parameter = self.parse_variable_declaration()?;
            parameters.push(parameter);

            if self.token_stream.token_kind() == TokenKind::RightParen {
                break;
            }

            self.token_stream.consume(TokenKind::Comma)?;
        }

        self.token_stream.consume(TokenKind::RightParen)?;
        self.token_stream.consume(TokenKind::ThinArrow)?;

        let return_type = self.parse_type()?;

        let block = self.parse_block_statement()?;

        let type_annotation = Type::Function {
            parameters: parameters
                .iter()
                .map(|param| match &param.kind {
                    DeclKind::Variable {
                        type_annotation, ..
                    } => type_annotation.clone(),
                    _ => unreachable!(),
                })
                .collect(),
            return_type: Box::new(return_type),
        };

        Ok(Decl::function(
            name,
            parameters,
            block,
            type_annotation,
            span,
        ))
    }

    /* Statements */
    fn parse_statement(&mut self) -> Result<Stmt, KaoriError> {
        let statement = match self.token_stream.token_kind() {
            TokenKind::Print => self.parse_print_statement(),
            TokenKind::LeftBrace => self.parse_block_statement(),
            TokenKind::If => self.parse_if_statement(),
            TokenKind::While => self.parse_while_loop_statement(),
            TokenKind::For => self.parse_for_loop_statement(),
            _ => self.parse_expr_statement(),
        }?;

        match statement.kind {
            StmtKind::Print(..) | StmtKind::Expression(..) => {
                self.token_stream.consume(TokenKind::Semicolon)?
            }
            _ => (),
        };

        return Ok(statement);
    }

    fn parse_expr_statement(&mut self) -> Result<Stmt, KaoriError> {
        let span = self.token_stream.span();
        let expression = self.parse_expr()?;

        return Ok(Stmt::expression(expression, span));
    }

    fn parse_print_statement(&mut self) -> Result<Stmt, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::Print)?;
        self.token_stream.consume(TokenKind::LeftParen)?;
        let expression = self.parse_expr()?;
        self.token_stream.consume(TokenKind::RightParen)?;

        return Ok(Stmt::print(expression, span));
    }

    fn parse_block_statement(&mut self) -> Result<Stmt, KaoriError> {
        let span = self.token_stream.span();

        let mut declarations: Vec<ASTNode> = Vec::new();

        self.token_stream.consume(TokenKind::LeftBrace)?;

        while !self.token_stream.at_end() && self.token_stream.token_kind() != TokenKind::RightBrace
        {
            let declaration = self.parse_declaration()?;
            declarations.push(declaration);
        }

        self.token_stream.consume(TokenKind::RightBrace)?;

        return Ok(Stmt::block(declarations, span));
    }

    fn parse_if_statement(&mut self) -> Result<Stmt, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::If)?;

        let condition = self.parse_expr()?;

        let then_branch = Box::new(self.parse_block_statement()?);

        if self.token_stream.token_kind() != TokenKind::Else {
            return Ok(Stmt::if_(condition, then_branch, None, span));
        }

        self.token_stream.advance();

        if self.token_stream.token_kind() == TokenKind::If {
            let else_branch = Some(Box::new(self.parse_if_statement()?));

            return Ok(Stmt::if_(condition, then_branch, else_branch, span));
        }

        let else_branch = Some(Box::new(self.parse_block_statement()?));

        return Ok(Stmt::if_(condition, then_branch, else_branch, span));
    }

    fn parse_while_loop_statement(&mut self) -> Result<Stmt, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::While)?;

        let condition = self.parse_expr()?;
        let block = Box::new(self.parse_block_statement()?);

        return Ok(Stmt::while_loop(condition, block, span));
    }

    fn parse_for_loop_statement(&mut self) -> Result<Stmt, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::For)?;

        let declaration = self.parse_variable_declaration()?;

        self.token_stream.consume(TokenKind::Semicolon)?;

        let condition = self.parse_expr()?;

        self.token_stream.consume(TokenKind::Semicolon)?;

        let increment = self.parse_expr_statement()?;

        let mut block = self.parse_block_statement()?;

        if let StmtKind::Block(declarations) = &mut block.kind {
            declarations.push(ASTNode::Statement(increment));
        }

        let while_loop = Stmt::while_loop(condition, Box::new(block), span);

        let mut declarations: Vec<ASTNode> = Vec::new();

        declarations.push(ASTNode::Declaration(declaration));
        declarations.push(ASTNode::Statement(while_loop));

        return Ok(Stmt::block(declarations, span));
    }

    /* Exprs */
    fn parse_expr(&mut self) -> Result<Box<Expr>, KaoriError> {
        if self
            .token_stream
            .look_ahead(&[TokenKind::Identifier, TokenKind::Assign])
        {
            return self.parse_assign();
        }

        return self.parse_or();
    }

    fn parse_assign(&mut self) -> Result<Box<Expr>, KaoriError> {
        let identifier = self.parse_identifier()?;

        let span = self.token_stream.span();
        self.token_stream.consume(TokenKind::Assign)?;

        let right = self.parse_expr()?;

        return Ok(Box::new(Expr::assign(identifier, right, span)));
    }

    fn parse_or(&mut self) -> Result<Box<Expr>, KaoriError> {
        let mut left = self.parse_and()?;

        while !self.token_stream.at_end() {
            let kind = self.token_stream.token_kind();
            let span = self.token_stream.span();

            if kind != TokenKind::Or {
                break;
            }

            self.token_stream.advance();

            let right = self.parse_and()?;

            left = Box::new(Expr::binary(BinaryOp::Or, left, right, span));
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Box<Expr>, KaoriError> {
        let mut left = self.parse_equality()?;

        while !self.token_stream.at_end() {
            let kind = self.token_stream.token_kind();
            let span = self.token_stream.span();

            if kind != TokenKind::And {
                break;
            }

            self.token_stream.advance();
            let right = self.parse_equality()?;

            left = Box::new(Expr::binary(BinaryOp::And, left, right, span));
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Box<Expr>, KaoriError> {
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

            left = Box::new(Expr::binary(operator, left, right, span));
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Box<Expr>, KaoriError> {
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

            left = Box::new(Expr::binary(operator, left, right, span));
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Box<Expr>, KaoriError> {
        let mut left = self.parse_factor()?;

        while !self.token_stream.at_end() {
            let kind = self.token_stream.token_kind();
            let span = self.token_stream.span();

            let operator = match kind {
                TokenKind::Plus => BinaryOp::Plus,
                TokenKind::Minus => BinaryOp::Minus,
                _ => break,
            };

            self.token_stream.advance();
            let right = self.parse_factor()?;

            left = Box::new(Expr::binary(operator, left, right, span));
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Box<Expr>, KaoriError> {
        let mut left = self.parse_unary()?;

        while !self.token_stream.at_end() {
            let kind = self.token_stream.token_kind();
            let span = self.token_stream.span();

            let operator = match kind {
                TokenKind::Multiply => BinaryOp::Multiply,
                TokenKind::Divide => BinaryOp::Divide,
                TokenKind::Remainder => BinaryOp::Remainder,
                _ => break,
            };

            self.token_stream.advance();
            let right = self.parse_unary()?;

            left = Box::new(Expr::binary(operator, left, right, span))
        }

        return Ok(left);
    }

    fn parse_unary(&mut self) -> Result<Box<Expr>, KaoriError> {
        let kind = self.token_stream.token_kind();
        let span = self.token_stream.span();

        let operator = match kind {
            TokenKind::Plus => {
                self.token_stream.advance();
                return self.parse_unary();
            }
            TokenKind::Minus => UnaryOp::Negate,
            TokenKind::Not => UnaryOp::Not,
            _ => return self.parse_primary(),
        };

        self.token_stream.advance();

        Ok(Box::new(Expr::unary(operator, self.parse_unary()?, span)))
    }

    fn parse_primary(&mut self) -> Result<Box<Expr>, KaoriError> {
        let kind = self.token_stream.token_kind();
        let span = self.token_stream.span();

        Ok(match kind {
            TokenKind::LeftParen => {
                self.token_stream.consume(TokenKind::LeftParen)?;
                let expr = self.parse_expr()?;
                self.token_stream.consume(TokenKind::RightParen)?;
                expr
            }
            TokenKind::NumberLiteral => {
                let lexeme = self.token_stream.lexeme();
                let value = lexeme.parse::<f64>().unwrap();

                self.token_stream.advance();
                Box::new(Expr::number_literal(value, span))
            }
            TokenKind::BooleanLiteral => {
                let lexeme = self.token_stream.lexeme();
                let value = lexeme.parse::<bool>().unwrap();

                self.token_stream.advance();
                Box::new(Expr::boolean_literal(value, span))
            }
            TokenKind::StringLiteral => {
                let lexeme = self.token_stream.lexeme();

                self.token_stream.advance();
                Box::new(Expr::string_literal(lexeme, span))
            }
            TokenKind::Identifier => self.parse_identifier()?,
            _ => {
                let span = self.token_stream.span();
                return Err(kaori_error!(span, "{:?} is a invalid operand", kind));
            }
        })
    }

    fn parse_identifier(&mut self) -> Result<Box<Expr>, KaoriError> {
        let name = self.token_stream.lexeme();
        let span = self.token_stream.span();

        let identifier = Box::new(Expr::identifier(name, span));

        self.token_stream.consume(TokenKind::Identifier)?;

        Ok(identifier)
    }

    /* Types */
    pub fn parse_type(&mut self) -> Result<Type, KaoriError> {
        match self.token_stream.token_kind() {
            TokenKind::Identifier => self.parse_primitive_type(),
            _ => Err(kaori_error!(
                self.token_stream.span(),
                "invalid type annotation: {:?}",
                self.token_stream.token_kind(),
            )),
        }
    }

    fn parse_primitive_type(&mut self) -> Result<Type, KaoriError> {
        let sub = self.token_stream.lexeme();

        let primitive = match sub.as_str() {
            "bool" => Type::Boolean,
            "str" => Type::String,
            "number" => Type::Number,
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
