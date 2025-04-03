use crate::{ast::ast_node::ASTNode, token::Token};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn look_ahead(&mut self) -> Option<Token> {
        return self.tokens.get(self.pos).cloned();
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn consume(&mut self, expected: Token) {
        if let Some(token) = self.look_ahead() {
            if token == expected {
                self.advance();
            } else {
                panic!("Expected {:?}, but found {:?}", expected, token);
            }
        } else {
            panic!("Unexpected end of input, expected {:?}", expected);
        }
    }

    fn parse_literal(&mut self) -> ASTNode {
        if let Some(token) = self.look_ahead() {
            return match token {
                Token::LeftParen => {
                    self.consume(Token::LeftParen);
                    let expression: ASTNode = self.parse_expression();
                    self.consume(Token::RightParen);
                    expression
                }
                Token::Number(value) => {
                    self.consume(Token::Number(value.clone()));
                    ASTNode::Literal {
                        _type: Token::Number("number".to_string()),
                        value: String::from(value),
                    }
                }
                _ => panic!("Found an unknown literal token"),
            };
        } else {
            panic!("Reached end of file and could not find a literal token");
        }
    }

    fn parse_unary(&mut self) -> ASTNode {
        if let Some(operator) = self.look_ahead() {
            return match operator {
                Token::Plus => {
                    self.consume(Token::Plus);
                    ASTNode::Unary {
                        _type: Token::Plus,
                        left: Box::new(self.parse_unary()),
                    }
                }
                Token::Minus => {
                    self.consume(Token::Minus);
                    ASTNode::Unary {
                        _type: Token::Minus,
                        left: Box::new(self.parse_unary()),
                    }
                }
                _ => self.parse_literal(),
            };
        } else {
            panic!("Reached end of file and could not find a literal token");
        }
    }

    fn parse_factor(&mut self) -> ASTNode {
        let mut left = self.parse_unary();

        while let Some(operator) = self.look_ahead() {
            match operator {
                Token::Multiply => {
                    self.consume(Token::Multiply);
                    let right = self.parse_unary();
                    left = ASTNode::BinaryOperator {
                        left: Box::new(left),
                        right: Box::new(right),
                        _type: Token::Multiply,
                    };
                }
                Token::Divide => {
                    self.consume(Token::Divide);
                    let right = self.parse_unary();
                    left = ASTNode::BinaryOperator {
                        left: Box::new(left),
                        right: Box::new(right),
                        _type: Token::Divide,
                    };
                }
                _ => break,
            }
        }

        return left;
    }

    fn parse_term(&mut self) -> ASTNode {
        let mut left = self.parse_factor();

        while let Some(operator) = self.look_ahead() {
            match operator {
                Token::Plus => {
                    self.consume(Token::Plus);
                    let right = self.parse_factor();
                    left = ASTNode::BinaryOperator {
                        left: Box::new(left),
                        right: Box::new(right),
                        _type: Token::Plus,
                    };
                }
                Token::Divide => {
                    self.consume(Token::Minus);
                    let right = self.parse_factor();
                    left = ASTNode::BinaryOperator {
                        left: Box::new(left),
                        right: Box::new(right),
                        _type: Token::Minus,
                    };
                }
                _ => break,
            }
        }

        return left;
    }

    pub fn parse_expression(&mut self) -> ASTNode {
        let ast = self.parse_term();
        self.pos = 0;

        return ast;
    }
}
