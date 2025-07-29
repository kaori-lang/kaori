use crate::{compilation_error, error::compilation_error::CompilationError};

use super::{span::Span, token::Token, token_kind::TokenKind};

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    position: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
            tokens: Vec::new(),
        }
    }

    fn at_end(&mut self) -> bool {
        return self.position >= self.source.len();
    }

    fn look_ahead(&mut self, expected: &[char]) -> bool {
        for i in 0..expected.len() {
            let j = self.position + i;

            if j >= self.source.len() {
                return false;
            }

            if expected[i] == self.source[j] {
                continue;
            }

            return false;
        }

        return true;
    }

    fn create_token(&mut self, kind: TokenKind, start: usize, end: usize) -> Token {
        let span = Span { start, end };

        let token = Token { kind, span };

        return token;
    }

    fn white_space(&mut self) {
        while !self.at_end() && self.source[self.position].is_whitespace() {
            self.position += 1;
        }
    }

    fn comment(&mut self) {
        self.position += 2;

        while !self.at_end() && !self.look_ahead(&['*', '/']) {
            self.position += 1;
        }

        self.position += 2;
    }

    fn identifier_or_keyword(&mut self) {
        let start = self.position;

        while !self.at_end() && self.source[self.position].is_alphabetic() {
            self.position += 1;
        }

        while !self.at_end()
            && (self.source[self.position].is_alphanumeric() || self.source[self.position] == '_')
        {
            self.position += 1;
        }
        let sub: String = self.source[start..self.position].iter().collect();

        let kind = match sub.as_str() {
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "return" => TokenKind::Return,
            "def" => TokenKind::Function,
            "print" => TokenKind::Print,
            "true" | "false" => TokenKind::BooleanLiteral,
            _ => TokenKind::Identifier,
        };

        let end = self.position - 1;
        let token = self.create_token(kind, start, end);
        self.tokens.push(token);
    }

    fn number_literal(&mut self) {
        let start = self.position;

        while !self.at_end() && self.source[self.position].is_ascii_digit() {
            self.position += 1;
        }

        if !self.at_end() && self.source[self.position] == '.' {
            self.position += 1;
        }

        while !self.at_end() && self.source[self.position].is_ascii_digit() {
            self.position += 1;
        }

        let end = self.position - 1;
        let token = self.create_token(TokenKind::NumberLiteral, start, end);

        self.tokens.push(token);
    }

    fn string_literal(&mut self) -> Result<(), CompilationError> {
        let start = self.position;

        self.position += 1;

        while !self.at_end() && self.source[self.position] != '"' {
            self.position += 1;
        }

        if self.at_end() {
            let end = self.position - 1;
            let span = Span { start, end };

            return Err(compilation_error!(span, "unfinished string literal"));
        }

        self.position += 1;

        let end = self.position - 1;
        let token = self.create_token(TokenKind::StringLiteral, start, end);

        self.tokens.push(token);

        Ok(())
    }

    pub fn symbol(&mut self) -> Result<(), CompilationError> {
        let start = self.position;

        let curr_char = self.source[self.position];

        let kind = match curr_char {
            '+' => {
                if self.look_ahead(&['+', '+']) {
                    TokenKind::Increment
                } else {
                    TokenKind::Plus
                }
            }

            '-' => {
                if self.look_ahead(&['-', '-']) {
                    TokenKind::Decrement
                } else if self.look_ahead(&['-', '>']) {
                    TokenKind::ThinArrow
                } else {
                    TokenKind::Minus
                }
            }
            '*' => TokenKind::Multiply,
            '/' => TokenKind::Divide,
            '%' => TokenKind::Remainder,
            '&' => {
                if self.look_ahead(&['&', '&']) {
                    TokenKind::And
                } else {
                    TokenKind::Invalid
                }
            }
            '|' => {
                if self.look_ahead(&['|', '|']) {
                    TokenKind::Or
                } else {
                    TokenKind::Invalid
                }
            }
            '!' => {
                if self.look_ahead(&['!', '=']) {
                    TokenKind::NotEqual
                } else {
                    TokenKind::Not
                }
            }
            '=' => {
                if self.look_ahead(&['=', '=']) {
                    TokenKind::Equal
                } else {
                    TokenKind::Assign
                }
            }
            '>' => {
                if self.look_ahead(&['>', '=']) {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }
            '<' => {
                if self.look_ahead(&['<', '=']) {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semicolon,
            ':' => TokenKind::Colon,
            _ => TokenKind::Invalid,
        };

        if kind == TokenKind::Invalid {
            let end = self.position - 1;
            let span = Span { start, end };

            return Err(compilation_error!(
                span,
                "{} is not a valid token",
                curr_char
            ));
        }

        let size = match kind {
            TokenKind::Increment
            | TokenKind::Decrement
            | TokenKind::And
            | TokenKind::Or
            | TokenKind::NotEqual
            | TokenKind::Equal
            | TokenKind::GreaterEqual
            | TokenKind::LessEqual
            | TokenKind::ThinArrow => 2,
            _ => 1,
        };

        self.position += size;

        let end = self.position - 1;
        let token = self.create_token(kind, start, end);

        self.tokens.push(token);
        Ok(())
    }

    pub fn get_next_token(&mut self) -> Result<(), CompilationError> {
        let c = self.source[self.position];

        if c == '"' {
            self.string_literal()?;
        } else if self.look_ahead(&['/', '*']) {
            self.comment();
        } else if c.is_alphabetic() {
            self.identifier_or_keyword();
        } else if c.is_ascii_digit() {
            self.number_literal();
        } else if c.is_whitespace() {
            self.white_space();
        } else {
            self.symbol()?;
        }

        Ok(())
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, CompilationError> {
        while !self.at_end() {
            self.get_next_token()?;
        }

        let span = Span {
            start: self.position - 1,
            end: self.position - 1,
        };

        let token = Token {
            kind: TokenKind::EndOfFile,
            span,
        };

        self.tokens.push(token);

        Ok(self.tokens.clone())
    }
}
