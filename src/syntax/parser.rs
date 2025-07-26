use crate::{
    error::error_type::ErrorType,
    lexer::{token_stream::TokenStream, token_type::TokenType},
};

use super::{
    declaration_ast::DeclarationAST,
    expression_ast::{BinaryOp, ExpressionAST, UnaryOp},
    statement_ast::StatementAST,
};

pub struct Parser {
    token_stream: TokenStream,
}

impl Parser {
    pub fn new(token_stream: TokenStream) -> Self {
        Self { token_stream }
    }

    /* Expressions */
    fn parse_expression(&mut self) -> Result<Box<ExpressionAST>, ErrorType> {
        return self.parse_or();
    }

    fn parse_assign(&mut self) -> Result<Box<ExpressionAST>, ErrorType> {
        let identifier = self.parse_identifier()?;

        self.token_stream.consume(TokenType::Assign)?;

        let expression = self.parse_expression()?;

        return Ok(Box::new(ExpressionAST::Assign {
            identifier,
            right: expression,
        }));
    }

    fn parse_or(&mut self) -> Result<Box<ExpressionAST>, ErrorType> {
        let mut left = self.parse_and()?;

        while !self.token_stream.at_end() {
            let ty = self.token_stream.current_kind();

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

    fn parse_and(&mut self) -> Result<Box<ExpressionAST>, ErrorType> {
        let mut left = self.parse_equality()?;

        while !self.token_stream.at_end() {
            let ty = self.token_stream.current_kind();

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

    fn parse_equality(&mut self) -> Result<Box<ExpressionAST>, ErrorType> {
        let mut left = self.parse_comparison()?;

        while !self.token_stream.at_end() {
            let ty = self.token_stream.current_kind();

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

    fn parse_comparison(&mut self) -> Result<Box<ExpressionAST>, ErrorType> {
        let mut left = self.parse_term()?;

        while !self.token_stream.at_end() {
            let ty = self.token_stream.current_kind();

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

    fn parse_term(&mut self) -> Result<Box<ExpressionAST>, ErrorType> {
        let mut left = self.parse_factor()?;

        while !self.token_stream.at_end() {
            let ty = self.token_stream.current_kind();

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

    fn parse_factor(&mut self) -> Result<Box<ExpressionAST>, ErrorType> {
        let mut left = self.parse_unary()?;

        while !self.token_stream.at_end() {
            let ty = self.token_stream.current_kind();

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

    fn parse_unary(&mut self) -> Result<Box<ExpressionAST>, ErrorType> {
        let ty = self.token_stream.current_kind();

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

    fn parse_primary(&mut self) -> Result<Box<ExpressionAST>, ErrorType> {
        let ty = self.token_stream.current_kind();

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
                let str_literal = lexeme;

                self.token_stream.advance();
                Box::new(ExpressionAST::StringLiteral(str_literal))
            }
            TokenType::Identifier => Box::new(ExpressionAST::Identifier(self.parse_identifier()?)),
            _ => return Err(ErrorType::SyntaxError(String::from("invalid primary"))),
        })
    }

    fn parse_identifier(&mut self) -> Result<String, ErrorType> {
        self.token_stream.consume(TokenType::Identifier)?;

        Ok(String::from("identifier"))
    }

    /* Statements */
    fn parse_statements(&mut self) -> Result<Vec<Box<StatementAST>>, ErrorType> {
        let mut statements: Vec<Box<StatementAST>> = Vec::new();

        while !self.token_stream.at_end() {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        return Ok(statements);
    }

    fn parse_statement(&mut self) -> Result<Box<StatementAST>, ErrorType> {
        Ok(match self.token_stream.current_kind() {
            TokenType::Print => self.parse_print_statement(),
            TokenType::LeftBrace => self.parse_block_statement(),
            TokenType::If => self.parse_if_statement(),
            TokenType::While => self.parse_while_loop_statement(),
            TokenType::For => self.parse_for_loop_statement(),
            _ => self.parse_expression_statement(),
        })
    }

    pub fn parse_expression_statement(&mut self) -> Result<Box<StatementAST>, ErrorType> {
        let line = self.token_stream.current_line();

        let expression = self.parse_expression()?;
        self.token_stream.consume(TokenType::Semicolon)?;

        return Ok(Box::new(StatementAST::Expression { expression, line }));
    }

    fn parse_print_statement(&mut self) -> Result<Box<StatementAST>, ErrorType> {
        let line = self.token_stream.current_line();

        self.token_stream.consume(TokenType::Print)?;
        self.token_stream.consume(TokenType::LeftParen)?;
        let expression = self.parse_expression()?;
        self.token_stream.consume(TokenType::RightParen)?;

        return Ok(Box::new(StatementAST::Print { expression, line }));
    }

    fn parse_if_statement(&mut self) -> Result<Box<StatementAST>, ErrorType> {
        let line = self.token_stream.current_line();

        self.token_stream.consume(TokenType::If)?;

        let condition = self.parse_expression()?;

        let then_branch = self.parse_block_statement()?;

        let if_statement = StatementAST::If {
            condition,
            then_branch,
            else_branch: None,
            line,
        };

        return Ok(Box::new(if_statement));
    }

    fn parse_while_loop_statement(&mut self) -> Result<Box<StatementAST>, ErrorType> {
        let line = self.token_stream.current_line();

        self.token_stream.consume(TokenType::While)?;

        let condition = self.parse_expression()?;
        let block = self.parse_block_statement()?;

        return Ok(Box::new(StatementAST::WhileLoop {
            condition,
            block,
            line,
        }));
    }

    fn parse_block_statement(&mut self) -> Result<Box<StatementAST>, ErrorType> {
        let line = self.token_stream.current_line();

        let mut statements: Vec<Box<StatementAST>> = Vec::new();

        self.token_stream.consume(TokenType::LeftBrace)?;

        while !self.token_stream.at_end()
            && self.token_stream.current_kind() != TokenType::RightBrace
        {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        self.token_stream.consume(TokenType::RightBrace)?;

        return Ok(Box::new(StatementAST::Block { statements, line }));
    }

    /*    fn parse_for_loop_statement(&mut self) -> Result<Box<StatementAST>, ErrorType> {
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

       fn parse_variable_declaration(&mut self) -> Result<Box<dyn Statement>, ErrorType> {
        let Some(declaration) = self.look_ahead() else {
            return Err(ErrorType::SyntaxError);
        };

        let data_type = match declaration.ty {
            TokenType::String | TokenType::Float | TokenType::Boolean => declaration.ty,
            _ => return Err(ErrorType::SyntaxError),
        };

        self.token_stream.consume(&data_type)?;

        let Some(identifier) = self.look_ahead() else {
            return Err(ErrorType::SyntaxError);
        };

        self.token_stream.consume(&TokenType::Identifier)?;

        self.token_stream.consume(&TokenType::Assign)?;

        let data = self.parse_ExpressionAST()?;

        self.token_stream.consume(&TokenType::Semicolon)?;

        return Ok(Box::new(VariableDeclStatement {
            data_type,
            identifier: identifier.lexeme,
            data,
            line: self.line,
        }));
    }

    pub fn execute(&mut self) -> Result<Vec<Box<DeclarationAST>>, ErrorType> {
        return self.pars;
    }*/
}
