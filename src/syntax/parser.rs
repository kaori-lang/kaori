use crate::{
    error::syntax_error::{Syntax, SyntaxError},
    lexer::{token_stream::TokenStream, token_type::TokenType},
};

use super::{
    ast_node::ASTNode,
    declaration_ast::DeclarationAST,
    expression_ast::{BinaryOp, ExpressionAST, UnaryOp},
    statement_ast::StatementAST,
    type_ast::TypeAST,
};

pub struct Parser {
    token_stream: TokenStream,
}

impl Parser {
    pub fn new(token_stream: TokenStream) -> Self {
        Self { token_stream }
    }

    pub fn execute(&mut self) -> Result<Vec<ASTNode>, SyntaxError> {
        let mut declarations: Vec<ASTNode> = Vec::new();

        while !self.token_stream.at_end() {
            let declaration = self.parse_declaration()?;
            declarations.push(declaration);
        }

        Ok(declarations)
    }

    /* Declarations */
    fn parse_declaration(&mut self) -> Result<ASTNode, SyntaxError> {
        let declaration = match self.token_stream.current_type() {
            _ => {
                if !self
                    .token_stream
                    .look_ahead(&[TokenType::Identifier, TokenType::Colon])
                {
                    return Ok(ASTNode::Statement(self.parse_statement()?));
                }

                let variable_declaration = ASTNode::Declaration(self.parse_variable_declaration()?);
                self.token_stream.consume(TokenType::Semicolon)?;
                variable_declaration
            }
        };

        Ok(declaration)
    }

    fn parse_variable_declaration(&mut self) -> Result<DeclarationAST, SyntaxError> {
        let line = self.token_stream.current_line();

        let identifier = self.parse_identifier()?;

        self.token_stream.consume(TokenType::Colon)?;

        let ty = self.parse_type()?;

        self.token_stream.consume(TokenType::Assign)?;

        let right = self.parse_expression()?;

        return Ok(DeclarationAST::Variable {
            identifier,
            right,
            ty,
            line,
        });
    }

    /* Statements */
    fn parse_statement(&mut self) -> Result<StatementAST, SyntaxError> {
        Ok(match self.token_stream.current_type() {
            TokenType::Print => self.parse_print_statement(),
            TokenType::LeftBrace => self.parse_block_statement(),
            TokenType::If => self.parse_if_statement(),
            TokenType::While => self.parse_while_loop_statement(),
            _ => self.parse_expression_statement(),
        }?)
    }

    fn parse_expression_statement(&mut self) -> Result<StatementAST, SyntaxError> {
        let line = self.token_stream.current_line();

        let expression = self.parse_expression()?;
        self.token_stream.consume(TokenType::Semicolon)?;

        return Ok(StatementAST::Expression { expression, line });
    }

    fn parse_print_statement(&mut self) -> Result<StatementAST, SyntaxError> {
        let line = self.token_stream.current_line();

        self.token_stream.consume(TokenType::Print)?;
        self.token_stream.consume(TokenType::LeftParen)?;
        let expression = self.parse_expression()?;
        self.token_stream.consume(TokenType::RightParen)?;
        self.token_stream.consume(TokenType::Semicolon)?;

        return Ok(StatementAST::Print { expression, line });
    }

    fn parse_if_statement(&mut self) -> Result<StatementAST, SyntaxError> {
        let line = self.token_stream.current_line();

        self.token_stream.consume(TokenType::If)?;

        let condition = self.parse_expression()?;

        let then_branch = Box::new(self.parse_block_statement()?);

        if self.token_stream.current_type() != TokenType::Else {
            return Ok(StatementAST::If {
                condition,
                then_branch,
                else_branch: None,
                line,
            });
        }

        self.token_stream.advance();

        if self.token_stream.current_type() == TokenType::If {
            return Ok(StatementAST::If {
                condition,
                then_branch,
                else_branch: Some(Box::new(self.parse_if_statement()?)),
                line,
            });
        }

        return Ok(StatementAST::If {
            condition,
            then_branch,
            else_branch: Some(Box::new(self.parse_block_statement()?)),
            line,
        });
    }

    fn parse_while_loop_statement(&mut self) -> Result<StatementAST, SyntaxError> {
        let line = self.token_stream.current_line();

        self.token_stream.consume(TokenType::While)?;

        let condition = self.parse_expression()?;
        let block = Box::new(self.parse_block_statement()?);

        return Ok(StatementAST::WhileLoop {
            condition,
            block,
            line,
        });
    }

    fn parse_block_statement(&mut self) -> Result<StatementAST, SyntaxError> {
        let line = self.token_stream.current_line();

        let mut declarations: Vec<ASTNode> = Vec::new();

        self.token_stream.consume(TokenType::LeftBrace)?;

        while !self.token_stream.at_end()
            && self.token_stream.current_type() != TokenType::RightBrace
        {
            let declaration = self.parse_declaration()?;
            declarations.push(declaration);
        }

        self.token_stream.consume(TokenType::RightBrace)?;

        return Ok(StatementAST::Block { declarations, line });
    }

    /* Expressions */
    fn parse_expression(&mut self) -> Result<Box<ExpressionAST>, SyntaxError> {
        if self
            .token_stream
            .look_ahead(&[TokenType::Identifier, TokenType::Assign])
        {
            return self.parse_assign();
        }

        return self.parse_or();
    }

    fn parse_assign(&mut self) -> Result<Box<ExpressionAST>, SyntaxError> {
        let identifier = self.parse_identifier()?;

        self.token_stream.consume(TokenType::Assign)?;

        let expression = self.parse_expression()?;

        return Ok(Box::new(ExpressionAST::Assign {
            identifier,
            right: expression,
        }));
    }

    fn parse_or(&mut self) -> Result<Box<ExpressionAST>, SyntaxError> {
        let mut left = self.parse_and()?;

        while !self.token_stream.at_end() {
            let ty = self.token_stream.current_type();

            if ty != TokenType::Or {
                break;
            }

            self.token_stream.advance();

            let right = self.parse_and()?;

            left = Box::new(ExpressionAST::Binary {
                operator: BinaryOp::Or,
                left,
                right,
            });
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Box<ExpressionAST>, SyntaxError> {
        let mut left = self.parse_equality()?;

        while !self.token_stream.at_end() {
            let ty = self.token_stream.current_type();

            if ty != TokenType::And {
                break;
            }

            self.token_stream.advance();
            let right = self.parse_equality()?;

            left = Box::new(ExpressionAST::Binary {
                operator: BinaryOp::And,
                left,
                right,
            });
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Box<ExpressionAST>, SyntaxError> {
        let mut left = self.parse_comparison()?;

        while !self.token_stream.at_end() {
            let ty = self.token_stream.current_type();

            let operator = match ty {
                TokenType::Equal => BinaryOp::Equal,
                TokenType::NotEqual => BinaryOp::NotEqual,
                _ => break,
            };

            self.token_stream.advance();
            let right = self.parse_comparison()?;

            left = Box::new(ExpressionAST::Binary {
                operator,
                left,
                right,
            });
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Box<ExpressionAST>, SyntaxError> {
        let mut left = self.parse_term()?;

        while !self.token_stream.at_end() {
            let ty = self.token_stream.current_type();

            let operator = match ty {
                TokenType::Greater => BinaryOp::Greater,
                TokenType::GreaterEqual => BinaryOp::GreaterEqual,
                TokenType::Less => BinaryOp::Less,
                TokenType::LessEqual => BinaryOp::LessEqual,
                _ => break,
            };

            self.token_stream.advance();
            let right = self.parse_term()?;

            left = Box::new(ExpressionAST::Binary {
                operator,
                left,
                right,
            });
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Box<ExpressionAST>, SyntaxError> {
        let mut left = self.parse_factor()?;

        while !self.token_stream.at_end() {
            let ty = self.token_stream.current_type();

            let operator = match ty {
                TokenType::Plus => BinaryOp::Plus,
                TokenType::Minus => BinaryOp::Minus,
                _ => break,
            };

            self.token_stream.advance();
            let right = self.parse_factor()?;

            left = Box::new(ExpressionAST::Binary {
                operator,
                left,
                right,
            });
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Box<ExpressionAST>, SyntaxError> {
        let mut left = self.parse_unary()?;

        while !self.token_stream.at_end() {
            let ty = self.token_stream.current_type();

            let operator = match ty {
                TokenType::Multiply => BinaryOp::Multiply,
                TokenType::Divide => BinaryOp::Divide,
                TokenType::Remainder => BinaryOp::Remainder,
                _ => break,
            };

            self.token_stream.advance();
            let right = self.parse_unary()?;

            left = Box::new(ExpressionAST::Binary {
                operator,
                left,
                right,
            })
        }

        return Ok(left);
    }

    fn parse_unary(&mut self) -> Result<Box<ExpressionAST>, SyntaxError> {
        let ty = self.token_stream.current_type();

        let operator = match ty {
            TokenType::Plus => {
                self.token_stream.advance();
                return self.parse_unary();
            }
            TokenType::Minus => UnaryOp::Negate,
            TokenType::Not => UnaryOp::Not,
            _ => return self.parse_primary(),
        };

        self.token_stream.advance();

        Ok(Box::new(ExpressionAST::Unary {
            operator,
            right: self.parse_unary()?,
        }))
    }

    fn parse_primary(&mut self) -> Result<Box<ExpressionAST>, SyntaxError> {
        let ty = self.token_stream.current_type();

        Ok(match ty {
            TokenType::LeftParen => {
                self.token_stream.consume(TokenType::LeftParen)?;
                let expr = self.parse_expression()?;
                self.token_stream.consume(TokenType::RightParen)?;
                expr
            }
            TokenType::NumberLiteral => {
                let lexeme = self.token_stream.lexeme();
                let number_literal = lexeme.parse::<f64>().unwrap();

                self.token_stream.advance();
                Box::new(ExpressionAST::NumberLiteral(number_literal))
            }
            TokenType::BooleanLiteral => {
                let lexeme = self.token_stream.lexeme();
                let bool_literal = lexeme.parse::<bool>().unwrap();

                self.token_stream.advance();
                Box::new(ExpressionAST::BooleanLiteral(bool_literal))
            }
            TokenType::StringLiteral => {
                let lexeme = self.token_stream.lexeme();

                self.token_stream.advance();
                Box::new(ExpressionAST::StringLiteral(lexeme))
            }
            TokenType::Identifier => Box::new(ExpressionAST::Identifier(self.parse_identifier()?)),
            _ => {
                return Err(SyntaxError {
                    error_type: Syntax::InvalidOperand(ty),
                    line: self.token_stream.current_line(),
                });
            }
        })
    }

    fn parse_identifier(&mut self) -> Result<String, SyntaxError> {
        let identifier = self.token_stream.lexeme();

        self.token_stream.consume(TokenType::Identifier)?;

        Ok(identifier)
    }

    /* Types */
    pub fn parse_type(&mut self) -> Result<TypeAST, SyntaxError> {
        match self.token_stream.current_type() {
            TokenType::Identifier => self.parse_primitive_type(),
            _ => Err(SyntaxError {
                error_type: Syntax::UnexpectedEof,
                line: self.token_stream.current_line(),
            }),
        }
    }

    pub fn parse_primitive_type(&mut self) -> Result<TypeAST, SyntaxError> {
        let primitive = match self.token_stream.lexeme().as_str() {
            "bool" => TypeAST::Boolean,
            "str" => TypeAST::String,
            "number" => TypeAST::Number,
            _ => {
                return Err(SyntaxError {
                    error_type: Syntax::UnexpectedEof,
                    line: self.token_stream.current_line(),
                });
            }
        };

        self.token_stream.advance();

        Ok(primitive)
    }

    /*    fn parse_for_loop_statement(&mut self) -> Result<StatementAST, SyntaxError> {
           let line = self.token_stream.current_line();

           self.token_stream.consume(&TokenType::For)?;

           let declaration = self.parse_variable_decl_statement()?;

           let condition = self.parse_ExpressionAST()?;

           self.token_stream.consume(&TokenType::Semicolon)?;

           let increment = self.parse_ExpressionAST()?;

           self.token_stream.consume(&TokenType::RightParen)?;



           let block = self.parse_block_statement()?;

           return Ok(Box::new(ForLoopStatement {
               declaration,
               condition,
               increment,
               block,
               line,
           }));
       }

    */
}
