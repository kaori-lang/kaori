use crate::token::Token;

#[derive(Debug, Clone)]

pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    pos: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let mut lexer = Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            pos: 0,
        };

        lexer.tokenize();

        return lexer;
    }

    fn look_ahead(&mut self) -> Option<char> {
        return self.source.get(self.pos).copied();
    }

    fn advance(&mut self) -> Option<char> {
        if self.pos < self.source.len() {
            let c = self.look_ahead();
            self.pos += 1;

            return c;
        } else {
            return None;
        }
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        return self.tokens.clone();
    }

    fn skip_white_space(&mut self) {
        while let Some(c) = self.look_ahead() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn get_next_number(&mut self, first_digit: char) -> Token {
        let mut number = first_digit.to_string();

        while let Some(c) = self.look_ahead() {
            if c.is_ascii_digit() {
                number.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if let Some(c) = self.look_ahead() {
            if c == '.' {
                number.push('.');
                self.advance();
            } else {
                return Token::Number(number);
            }
        }

        while let Some(c) = self.look_ahead() {
            if c.is_ascii_digit() {
                number.push(c);
                self.advance();
            } else {
                break;
            }
        }

        return Token::Number(number);
    }

    fn get_next_token(&mut self) -> Token {
        let Some(c) = self.advance() else {
            return Token::EndOfFile;
        };

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

    fn tokenize(&mut self) {
        loop {
            self.skip_white_space();

            let token = self.get_next_token();

            if token == Token::EndOfFile {
                self.tokens.push(token);
                break;
            }

            self.tokens.push(token);
        }
    }
}
