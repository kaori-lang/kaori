use crate::{
    ast::ast_node::{ASTNode, Literal},
    token::Token,
};

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

    fn consume(&mut self, expected: &Token) {
        let Some(token) = self.look_ahead() else {
            panic!("Unexpected end of input, expected {:?}", expected);
        };

        if token == *expected {
            self.advance();
        } else {
            panic!("Expected {:?}, but found {:?}", expected, token);
        }
    }

    fn parse_literal(&mut self) -> ASTNode {
        let Some(token) = self.look_ahead() else {
            panic!("Reached end of file and could not find a literal token");
        };

        return match token {
            Token::LeftParen => {
                self.consume(&Token::LeftParen);
                let expression: ASTNode = self.parse_expression();
                self.consume(&Token::RightParen);
                expression
            }
            Token::Number(value) => {
                self.consume(&Token::Number(value.clone()));
                ASTNode::Literal {
                    _type: Literal::Number,
                    value: String::from(value),
                }
            }
            _ => panic!("Found an unknown literal token"),
        };
    }

    fn parse_unary(&mut self) -> ASTNode {
        let Some(token) = self.look_ahead() else {
            panic!("Reached end of file and could not find a literal token");
        };

        if !matches!(token, Token::Plus | Token::Minus | Token::Not) {
            return self.parse_literal();
        }

        self.consume(&token);

        return ASTNode::UnaryOperator {
            _type: token,
            right: Box::new(self.parse_unary()),
        };
    }

    fn parse_factor(&mut self) -> ASTNode {
        let mut left = self.parse_unary();

        while let Some(token) = self.look_ahead() {
            if !matches!(token, Token::Multiply | Token::Divide) {
                break;
            }

            self.consume(&token);
            let right = self.parse_unary();

            left = ASTNode::BinaryOperator {
                left: Box::new(left),
                right: Box::new(right),
                _type: token,
            }
        }

        return left;
    }

    fn parse_term(&mut self) -> ASTNode {
        let mut left = self.parse_factor();

        while let Some(token) = self.look_ahead() {
            if !matches!(token, Token::Plus | Token::Minus) {
                break;
            }

            self.consume(&token);
            let right = self.parse_factor();

            left = ASTNode::BinaryOperator {
                left: Box::new(left),
                right: Box::new(right),
                _type: token,
            }
        }

        return left;
    }

    fn parse_comparison(&mut self) -> ASTNode {
        let mut left = self.parse_term();

        while let Some(token) = self.look_ahead() {
            if !matches!(
                token,
                Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual
            ) {
                break;
            }

            self.consume(&token);
            let right = self.parse_term();

            left = ASTNode::BinaryOperator {
                left: Box::new(left),
                right: Box::new(right),
                _type: token,
            }
        }

        return left;
    }
    fn parse_equality(&mut self) -> ASTNode {
        let mut left = self.parse_comparison();

        while let Some(token) = self.look_ahead() {
            if !matches!(token, Token::Equal | Token::NotEqual) {
                break;
            }

            self.consume(&token);
            let right = self.parse_comparison();

            left = ASTNode::BinaryOperator {
                left: Box::new(left),
                right: Box::new(right),
                _type: token,
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
