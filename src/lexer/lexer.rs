use crate::{error::kaori_error::KaoriError, kaori_error};

use super::{span::Span, token::Token, token_kind::TokenKind};

pub struct Lexer {
    source: Vec<char>,
    index: usize,
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            index: 0,
            tokens: Vec::new(),
        }
    }

    fn at_end(&mut self) -> bool {
        self.index >= self.source.len()
    }

    fn look_ahead(&mut self, expected: &str) -> bool {
        for (i, expected) in expected.chars().enumerate() {
            let j = self.index + i;

            if Some(&expected) != self.source.get(j) {
                return false;
            }
        }

        true
    }

    fn white_space(&mut self) {
        while !self.at_end() && self.source[self.index].is_whitespace() {
            self.index += 1;
        }
    }

    fn multiline_comment(&mut self) {
        self.index += 2;

        while !self.at_end() && !self.look_ahead("*/") {
            self.index += 1;
        }

        self.index += 2;
    }

    fn line_comment(&mut self) {
        self.index += 2;

        while !self.at_end() && !self.look_ahead("\n") {
            self.index += 1;
        }

        self.index += 1;
    }

    fn identifier_or_keyword(&mut self) {
        let start = self.index;

        while !self.at_end() && self.source[self.index].is_alphabetic() {
            self.index += 1;
        }

        while !self.at_end()
            && (self.source[self.index].is_alphanumeric() || self.source[self.index] == '_')
        {
            self.index += 1;
        }
        let sub: String = self.source[start..self.index].iter().collect();

        let kind = match sub.as_str() {
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "return" => TokenKind::Return,
            "def" => TokenKind::Function,
            "struct" => TokenKind::Struct,
            "print" => TokenKind::Print,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "bool" => TokenKind::Bool,
            "number" => TokenKind::Number,
            "var" => TokenKind::Variable,
            _ => TokenKind::Identifier,
        };

        let end = self.index;
        let token = Token::new(kind, start, end);
        self.tokens.push(token);
    }

    fn number_literal(&mut self) {
        let start = self.index;

        while !self.at_end() && self.source[self.index].is_ascii_digit() {
            self.index += 1;
        }

        if !self.at_end() && self.source[self.index] == '.' {
            self.index += 1;
        }

        while !self.at_end() && self.source[self.index].is_ascii_digit() {
            self.index += 1;
        }

        let end = self.index;
        let token = Token::new(TokenKind::NumberLiteral, start, end);

        self.tokens.push(token);
    }

    fn string_literal(&mut self) -> Result<(), KaoriError> {
        let start = self.index;

        self.index += 1;

        while !self.at_end() && self.source[self.index] != '"' {
            self.index += 1;
        }

        if self.at_end() {
            let end = self.index;
            let span = Span { start, end };

            return Err(kaori_error!(span, "invalid unfinished string literal"));
        }

        self.index += 1;

        let end = self.index;
        let token = Token::new(TokenKind::StringLiteral, start, end);

        self.tokens.push(token);

        Ok(())
    }

    pub fn symbol(&mut self) -> Result<(), KaoriError> {
        let start = self.index;

        let curr_char = self.source[self.index];

        let kind = match curr_char {
            '+' => {
                if self.look_ahead("+=") {
                    TokenKind::AddAssign
                } else {
                    TokenKind::Plus
                }
            }
            '-' => {
                if self.look_ahead("-=") {
                    TokenKind::SubtractAssign
                } else if self.look_ahead("->") {
                    TokenKind::ThinArrow
                } else {
                    TokenKind::Minus
                }
            }
            '*' => {
                if self.look_ahead("*=") {
                    TokenKind::MultiplyAssign
                } else {
                    TokenKind::Multiply
                }
            }
            '/' => {
                if self.look_ahead("/=") {
                    TokenKind::DivideAssign
                } else {
                    TokenKind::Divide
                }
            }
            '%' => {
                if self.look_ahead("%=") {
                    TokenKind::ModuloAssign
                } else {
                    TokenKind::Modulo
                }
            }
            '&' => {
                if self.look_ahead("&&") {
                    TokenKind::And
                } else {
                    TokenKind::Invalid
                }
            }
            '|' => {
                if self.look_ahead("||") {
                    TokenKind::Or
                } else {
                    TokenKind::Invalid
                }
            }
            '!' => {
                if self.look_ahead("!=") {
                    TokenKind::NotEqual
                } else {
                    TokenKind::Not
                }
            }
            '=' => {
                if self.look_ahead("==") {
                    TokenKind::Equal
                } else {
                    TokenKind::Assign
                }
            }
            '>' => {
                if self.look_ahead(">=") {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }
            '<' => {
                if self.look_ahead("<=") {
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
            let end = self.index;
            let span = Span { start, end };

            return Err(kaori_error!(span, "{} is not a valid token", curr_char));
        }

        let size = match kind {
            TokenKind::AddAssign
            | TokenKind::SubtractAssign
            | TokenKind::MultiplyAssign
            | TokenKind::DivideAssign
            | TokenKind::ModuloAssign
            | TokenKind::And
            | TokenKind::Or
            | TokenKind::NotEqual
            | TokenKind::Equal
            | TokenKind::GreaterEqual
            | TokenKind::LessEqual
            | TokenKind::ThinArrow => 2,
            _ => 1,
        };

        self.index += size;

        let end = self.index;
        let token = Token::new(kind, start, end);

        self.tokens.push(token);
        Ok(())
    }

    pub fn get_next_token(&mut self) -> Result<(), KaoriError> {
        let c = self.source[self.index];
        match c {
            '"' => self.string_literal()?,
            '/' if self.look_ahead("/*") => self.multiline_comment(),
            '/' if self.look_ahead("//") => self.line_comment(),
            c if c.is_alphabetic() => self.identifier_or_keyword(),
            '0'..='9' => self.number_literal(),
            c if c.is_whitespace() => self.white_space(),
            _ => self.symbol()?,
        };
        Ok(())
    }

    pub fn tokenize(&mut self) -> Result<(), KaoriError> {
        while !self.at_end() {
            self.get_next_token()?;
        }

        let span = Span {
            start: self.index - 1,
            end: self.index - 1,
        };

        let token = Token {
            kind: TokenKind::EndOfFile,
            span,
        };

        self.tokens.push(token);

        Ok(())
    }
}
