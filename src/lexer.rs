use std::collections::VecDeque;

use crate::token::Token;

#[derive(Debug, Clone)]

pub struct Lexer {
    source: VecDeque<char>,
    tokens: VecDeque<Token>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        let mut lexer = Self {
            source: source.chars().collect(),
            tokens: VecDeque::new(),
        };

        lexer.tokenize();

        return lexer;
    }

    pub fn get_tokens(&self) -> VecDeque<Token> {
        return self.tokens.clone();
    }

    fn skip_white_space(&mut self) {
        while let Some(c) = self.source.front() {
            if c.is_whitespace() {
                self.source.pop_front();
            } else {
                break;
            }
        }
    }

    fn get_next_number(&mut self, first_digit: char) -> Token {
        let mut number = first_digit.to_string();

        while let Some(c) = self.source.front() {
            if c.is_ascii_digit() {
                number.push(*c);
                self.source.pop_front();
            } else {
                break;
            }
        }

        if let Some(c) = self.source.front() {
            if *c == '.' {
                number.push('.');
                self.source.pop_front();
            } else {
                return Token::Number(number);
            }
        }

        while let Some(c) = self.source.front() {
            if c.is_ascii_digit() {
                number.push(*c);
                self.source.pop_front();
            } else {
                break;
            }
        }

        return Token::Number(number);
    }

    fn get_next_token(&mut self) -> Token {
        if let Some(c) = self.source.pop_front() {
            return match c {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Multiply,
                '/' => Token::Divide,
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                '0'..='9' => self.get_next_number(c),
                _ => Token::Unknown,
            };
        }

        return Token::EndOfFile;
    }

    fn tokenize(&mut self) {
        loop {
            self.skip_white_space();

            let token = self.get_next_token();

            if token == Token::EndOfFile {
                self.tokens.push_back(token);
                break;
            }

            self.tokens.push_back(token);
        }
    }
}
