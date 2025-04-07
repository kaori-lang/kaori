use crate::{
    ast::{
        binary_operator::BinaryOperator, expr::Expr, literal::Literal,
        unary_operator::UnaryOperator,
    },
    token::{Token, TokenType},
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

    fn consume(&mut self, expected: &TokenType) {
        let Some(token) = self.look_ahead() else {
            panic!("Unexpected end of input, expected {:?}", expected);
        };

        if token.ty == *expected {
            self.advance();
        } else {
            panic!("Expected {:?}, but found {:?}", expected, token);
        }
    }

    fn parse_literal(&mut self) -> Box<dyn Expr> {
        let Some(token) = self.look_ahead() else {
            panic!("Reached end of file and could not find a literal token");
        };

        match token.ty {
            TokenType::LeftParen => {
                self.consume(&TokenType::LeftParen);
                let expression = self.parse_expression();
                self.consume(&TokenType::RightParen);
                expression
            }
            TokenType::Number => {
                self.consume(&TokenType::Number);
                Box::new(Literal::new(token.ty, token.value.unwrap()))
            }
            _ => panic!("Found an unknown literal token"),
        }
    }

    fn parse_unary(&mut self) -> Box<dyn Expr> {
        let Some(token) = self.look_ahead() else {
            panic!("Reached end of file and could not find a literal token");
        };

        if token.ty == TokenType::Plus {
            self.consume(&token.ty);
            return self.parse_unary();
        }

        if !matches!(token.ty, TokenType::Minus | TokenType::Not) {
            return self.parse_literal();
        }

        self.consume(&token.ty);

        return Box::new(UnaryOperator::new(token.ty, self.parse_unary()));
    }

    fn parse_factor(&mut self) -> Box<dyn Expr> {
        let mut left = self.parse_unary();

        while let Some(token) = self.look_ahead() {
            if !matches!(token.ty, TokenType::Multiply | TokenType::Divide) {
                break;
            }

            self.consume(&token.ty);
            let right = self.parse_unary();

            left = Box::new(BinaryOperator::new(token.ty, left, right));
        }

        return left;
    }

    fn parse_term(&mut self) -> Box<dyn Expr> {
        let mut left = self.parse_factor();

        while let Some(token) = self.look_ahead() {
            if !matches!(token.ty, TokenType::Plus | TokenType::Minus) {
                break;
            }

            self.consume(&token.ty);
            let right = self.parse_factor();

            left = Box::new(BinaryOperator::new(token.ty, left, right));
        }

        return left;
    }

    fn parse_comparison(&mut self) -> Box<dyn Expr> {
        let mut left = self.parse_term();

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

            self.consume(&token.ty);
            let right = self.parse_term();

            left = Box::new(BinaryOperator::new(token.ty, left, right));
        }

        return left;
    }
    fn parse_equality(&mut self) -> Box<dyn Expr> {
        let mut left = self.parse_comparison();

        while let Some(token) = self.look_ahead() {
            if !matches!(token.ty, TokenType::Equal | TokenType::NotEqual) {
                break;
            }

            self.consume(&token.ty);
            let right = self.parse_comparison();

            left = Box::new(BinaryOperator::new(token.ty, left, right));
        }

        return left;
    }

    fn parse_expression(&mut self) -> Box<dyn Expr> {
        return self.parse_term();
    }

    pub fn get_ast(&mut self) -> Box<dyn Expr> {
        let ast = self.parse_equality();
        self.pos = 0;

        return ast;
    }
}
