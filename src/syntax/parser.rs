use crate::{
    error::error_type::ErrorType,
    lexer::{token_stream::TokenStream, token_type::TokenType},
};

use super::{
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

    fn parse_identifier(&mut self) -> Result<String, ErrorType> {
        self.token_stream.consume(TokenType::Identifier)?;

        Ok(String::from("identifier"))
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
                println!("{:#?}", lexeme);
                let number_literal = lexeme.parse::<f64>().unwrap();

                self.token_stream.advance();
                Box::new(ExpressionAST::NumberLiteral(number_literal))
            }
            TokenType::BooleanLiteral => {
                let lexeme = self.token_stream.lexeme();
                println!("{:#?}", lexeme);
                let bool_literal = lexeme.parse::<bool>().unwrap();

                self.token_stream.advance();
                Box::new(ExpressionAST::BooleanLiteral(bool_literal))
            }
            TokenType::StringLiteral => {
                let lexeme = self.token_stream.lexeme();
                println!("{:#?}", lexeme);

                self.token_stream.advance();
                let str_literal = lexeme;

                Box::new(ExpressionAST::StringLiteral(str_literal))
            }
            TokenType::Identifier => Box::new(ExpressionAST::Identifier(self.parse_identifier()?)),
            _ => return Err(ErrorType::SyntaxError(String::from("invalid primary"))),
        })
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

    fn parse_assign(&mut self) -> Result<Box<ExpressionAST>, ErrorType> {
        let identifier = self.parse_identifier()?;

        self.token_stream.consume(TokenType::Assign)?;

        let expression = self.parse_expression()?;

        return Ok(Box::new(ExpressionAST::Assign {
            identifier,
            right: expression,
        }));
    }

    fn parse_expression(&mut self) -> Result<Box<ExpressionAST>, ErrorType> {
        return self.parse_or();
    }

    pub fn parse_expression_statement(&mut self) -> Result<Box<StatementAST>, ErrorType> {
        let line = self.token_stream.current_line();

        let expression = self.parse_expression()?;
        self.token_stream.consume(TokenType::Semicolon)?;

        return Ok(Box::new(StatementAST::Expression { expression, line }));
    }

    /*    fn parse_print_statement(&mut self) -> Result<Box<dyn Statement>, ErrorType> {
        self.token_stream.consume(&TokenType::Print)?;
        self.token_stream.consume(&TokenType::LeftParen)?;
        let ExpressionAST = self.parse_ExpressionAST()?;
        self.token_stream.consume(&TokenType::RightParen)?;
        self.token_stream.consume(&TokenType::Semicolon)?;

        return Ok(Box::new(PrintStatement {
            ExpressionAST,
            line: self.line,
        }));
    }

    fn parse_variable_decl_statement(&mut self) -> Result<Box<dyn Statement>, ErrorType> {
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

    fn parse_if_statement(&mut self) -> Result<Box<dyn Statement>, ErrorType> {
        self.token_stream.consume(&TokenType::If)?;
        self.token_stream.consume(&TokenType::LeftParen)?;
        let condition = self.parse_ExpressionAST()?;
        self.token_stream.consume(&TokenType::RightParen)?;

        let then_branch = self.parse_block_statement()?;

        let mut if_statement = IfStatement {
            condition,
            then_branch,
            else_branch: None,
            line: self.line,
        };

        let Some(Token {
            ty: TokenType::Else,
            ..
        }) = self.look_ahead()
        else {
            return Ok(Box::new(if_statement));
        };

        self.token_stream.consume(&TokenType::Else)?;

        let Some(token) = self.look_ahead() else {
            return Err(ErrorType::SyntaxError);
        };

        if_statement.else_branch = self.token_stream.current_kind() {
            TokenType::LeftBrace => Some(self.parse_block_statement()?),
            TokenType::If => Some(self.parse_if_statement()?),
            _ => return Err(ErrorType::SyntaxError),
        };

        return Ok(Box::new(if_statement));
    }

    fn parse_while_loop_statement(&mut self) -> Result<Box<dyn Statement>, ErrorType> {
        self.token_stream.consume(&TokenType::While)?;

        let condition = self.parse_ExpressionAST()?;

        let line = self.line;

        let block = self.parse_block_statement()?;

        return Ok(Box::new(WhileLoopStatement {
            condition,
            block,
            line,
        }));
    }

    fn parse_for_loop_statement(&mut self) -> Result<Box<dyn Statement>, ErrorType> {
        self.token_stream.consume(&TokenType::For)?;
        self.token_stream.consume(&TokenType::LeftParen)?;

        let declaration = self.parse_variable_decl_statement()?;

        let condition = self.parse_ExpressionAST()?;

        self.token_stream.consume(&TokenType::Semicolon)?;

        let increment = self.parse_ExpressionAST()?;

        self.token_stream.consume(&TokenType::RightParen)?;

        let line = self.line;

        let block = self.parse_block_statement()?;

        return Ok(Box::new(ForLoopStatement {
            declaration,
            condition,
            increment,
            block,
            line,
        }));
    }

    fn parse_block_statement(&mut self) -> Result<Box<dyn Statement>, ErrorType> {
        let mut statements: Vec<Box<dyn Statement>> = Vec::new();

        self.token_stream.consume(&TokenType::LeftBrace)?;

        let line = self.line;

        while let Some(token) = self.look_ahead() {
            if token.ty == TokenType::RightBrace {
                break;
            }

            let statement = self.parse_statement(token)?;
            statements.push(statement);
        }

        self.token_stream.consume(&TokenType::RightBrace)?;

        return Ok(Box::new(BlockStatement { statements, line }));
    }

    fn parse_statement(&mut self, token: Token) -> Result<Box<dyn Statement>, ErrorType> {
        self.token_stream.current_kind() {
            TokenType::Float | TokenType::Boolean | TokenType::String => {
                self.parse_variable_decl_statement()
            }
            TokenType::Print => self.parse_print_statement(),
            TokenType::LeftBrace => self.parse_block_statement(),
            TokenType::If => self.parse_if_statement(),
            TokenType::While => self.parse_while_loop_statement(),
            TokenType::For => self.parse_for_loop_statement(),
            _ => self.parse_ExpressionAST_statement(),
        }
    }

    fn parse_statements(&mut self) -> Result<Vec<Box<dyn Statement>>, ErrorType> {
        let mut statements: Vec<Box<dyn Statement>> = Vec::new();

        while let Some(token) = self.look_ahead() {
            let statement = self.parse_statement(token)?;
            statements.push(statement);
        }

        return Ok(statements);
    }



    pub fn execute(&mut self) -> Result<Vec<Box<dyn Statement>>, YFError> {
        let statements = match self.parse_statements() {
            Ok(statements) => Ok(statements),
            Err(error_type) => {
                let error = YFError {
                    error_type,
                    line: self.line,
                };

                Err(error)
            }
        };

        self.pos = 0;

        return statements;
    }  */
}
