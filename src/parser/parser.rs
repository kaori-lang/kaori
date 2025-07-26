use crate::{
    ast::{
        expression::{
            AssignOperator, BinaryOperator, Expression, Identifier, Literal, UnaryOperator,
        },
        statement::{
            BlockStatement, ExpressionStatement, ForLoopStatement, IfStatement, PrintStatement,
            Statement, VariableDeclStatement, WhileLoopStatement,
        },
    },
    lexer::{data::Data, token::Token, token_stream::TokenStream, token_type::TokenType},
    yf_error::{ErrorType, YFError},
};

pub struct Parser {
    token_stream: TokenStream,
}

impl Parser {
    pub fn new(token_stream: TokenStream) -> Self {
        Self { token_stream }
    }

    fn parse_primary(&mut self) -> Result<Box<dyn Expression>, ErrorType> {
        let ty = self.token_stream.current_kind();

        match ty {
            TokenType::LeftParen => {
                self.token_stream.consume(TokenType::LeftParen)?;
                let expression = self.parse_expression()?;
                self.token_stream.consume(TokenType::RightParen)?;
                Ok(expression)
            }
            TokenType::Literal => {
                self.token_stream.consume(TokenType::Literal)?;
                Ok(Box::new(Literal {
                    value: Data::Float(1.5),
                }))
            }
            TokenType::Identifier => {
                self.token_stream.consume(TokenType::Identifier)?;
                Ok(Box::new(Identifier {
                    value: String::from("identifier"),
                }))
            }
            _ => Err(ErrorType::SyntaxError),
        }
    }

    fn parse_unary(&mut self) -> Result<Box<dyn Expression>, ErrorType> {
        let ty = self.token_stream.current_kind();

        match ty {
            TokenType::Plus => {
                self.token_stream.consume(TokenType::Plus)?;
                self.parse_unary()
            }
            TokenType::Minus | TokenType::Not => {
                self.token_stream.consume(ty)?;
                Ok(Box::new(UnaryOperator {
                    ty,
                    right: self.parse_unary()?,
                }))
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_factor(&mut self) -> Result<Box<dyn Expression>, ErrorType> {
        let mut left = self.parse_unary()?;

        while !self.token_stream.at_end() {
            let operator = self.token_stream.current_kind();
            self.token_stream.advance();
            let right = self.parse_unary()?;

           match operator {
                TokenType::Multiply =>  {
                    Expression::BinaryOperator {
                        operator,
                        left,
                        right
                    }
                }
                TokenType::
                

            };

          
            

            left = Box::new(BinaryOperator {
                ty: token.ty,
                left,
                right,
            });
        }

        return Ok(left);
    }

    fn parse_term(&mut self) -> Result<Box<dyn Expression>, ErrorType> {
        let mut left = self.parse_factor()?;

        while let Some(token) = self.look_ahead() {
            if !matches!(token.ty, TokenType::Plus | TokenType::Minus) {
                break;
            }

            self.token_stream.consume(&token.ty)?;
            let right = self.parse_factor()?;

            left = Box::new(BinaryOperator {
                ty: token.ty,
                left,
                right,
            });
        }

        return Ok(left);
    }

    fn parse_comparison(&mut self) -> Result<Box<dyn Expression>, ErrorType> {
        let mut left = self.parse_term()?;

        while let Some(token) = self.look_ahead() {
            if !matches!(
                token.ty,
                TokenType::Greater
                    | TokenType::GreaterEqual
                    | TokenType::Less
                    | TokenType::LessEqual
            ) {
                break;
            }

            self.token_stream.consume(&token.ty)?;
            let right = self.parse_term()?;

            left = Box::new(BinaryOperator {
                ty: token.ty,
                left,
                right,
            });
        }

        return Ok(left);
    }

    fn parse_equality(&mut self) -> Result<Box<dyn Expression>, ErrorType> {
        let mut left = self.parse_comparison()?;

        while let Some(token) = self.look_ahead() {
            if !matches!(token.ty, TokenType::Equal | TokenType::NotEqual) {
                break;
            }

            self.token_stream.consume(&token.ty)?;
            let right = self.parse_comparison()?;

            left = Box::new(BinaryOperator {
                ty: token.ty,
                left,
                right,
            });
        }

        return Ok(left);
    }

    fn parse_and(&mut self) -> Result<Box<dyn Expression>, ErrorType> {
        let mut left = self.parse_equality()?;

        while let Some(token) = self.look_ahead() {
            if token.ty != TokenType::And {
                break;
            }

            self.token_stream.consume(&TokenType::And)?;

            let right = self.parse_equality()?;

            left = Box::new(BinaryOperator {
                ty: token.ty,
                left,
                right,
            });
        }

        return Ok(left);
    }

    fn parse_or(&mut self) -> Result<Box<dyn Expression>, ErrorType> {
        let mut left = self.parse_and()?;

        while let Some(token) = self.look_ahead() {
            if token.ty != TokenType::Or {
                break;
            }

            self.token_stream.consume(&TokenType::Or)?;

            let right = self.parse_and()?;

            left = Box::new(BinaryOperator {
                ty: token.ty,
                left,
                right,
            });
        }

        return Ok(left);
    }

    fn parse_assign(&mut self) -> Result<Box<dyn Expression>, ErrorType> {
        let expression = self.parse_or()?;

        match expression.as_any().downcast_ref::<Identifier>() {
            Some(identifier) => {
                let Some(Token {
                    ty: TokenType::Assign,
                    ..
                }) = self.look_ahead()
                else {
                    return Ok(expression);
                };

                self.token_stream.consume(&TokenType::Assign)?;

                Ok(Box::new(AssignOperator {
                    identifier: identifier.clone(),
                    right: self.parse_assign()?,
                }))
            }
            _ => Ok(expression),
        }
    }

    fn parse_expression(&mut self) -> Result<Box<dyn Expression>, ErrorType> {
        return self.parse_assign();
    }

    fn parse_expression_statement(&mut self) -> Result<Box<dyn Statement>, ErrorType> {
        let expression = self.parse_expression()?;
        self.token_stream.consume(&TokenType::Semicolon)?;

        return Ok(Box::new(ExpressionStatement {
            expression,
            line: self.line,
        }));
    }

    fn parse_print_statement(&mut self) -> Result<Box<dyn Statement>, ErrorType> {
        self.token_stream.consume(&TokenType::Print)?;
        self.token_stream.consume(&TokenType::LeftParen)?;
        let expression = self.parse_expression()?;
        self.token_stream.consume(&TokenType::RightParen)?;
        self.token_stream.consume(&TokenType::Semicolon)?;

        return Ok(Box::new(PrintStatement {
            expression,
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

        let data = self.parse_expression()?;

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
        let condition = self.parse_expression()?;
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

        let condition = self.parse_expression()?;

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

        let condition = self.parse_expression()?;

        self.token_stream.consume(&TokenType::Semicolon)?;

        let increment = self.parse_expression()?;

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
            _ => self.parse_expression_statement(),
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

    /*
    fn recover_from_error(&mut self) {
        while let Some(token) = self.look_ahead() {
            let _ = self.token_stream.consume(&token.ty);

            self.token_stream.current_kind() {
                TokenType::Semicolon => break,
                _ => (),
            };
        }
    } */

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
    }
}
