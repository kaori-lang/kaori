use crate::error::error_type::{ErrorType, SyntaxError};

use super::{token::Token, token_stream::TokenStream, token_type::TokenType};

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    line: u32,
    position: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            line: 1,
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

    fn create_token(&mut self, ty: TokenType, position: usize, size: usize) {
        let token = Token {
            ty,
            line: self.line,
            position,
            size,
        };

        self.tokens.push(token);
    }

    fn white_space(&mut self) {
        while !self.at_end() {
            let c = self.source[self.position];

            if !c.is_whitespace() {
                break;
            }

            if c == '\n' {
                self.line += 1;
            }

            self.position += 1;
        }
    }

    fn identifier_or_keyword(&mut self) {
        let start = self.position;

        while !self.at_end() && self.source[self.position].is_alphabetic() {
            self.position += 1;
        }

        while !self.at_end() && self.source[self.position].is_alphanumeric()
            || self.source[self.position] == '_'
        {
            self.position += 1;
        }
        let sub: String = self.source[start..self.position].iter().collect();

        let ty = match sub.as_str() {
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "for" => TokenType::For,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "return" => TokenType::Return,
            "def" => TokenType::Function,
            "print" => TokenType::Print,
            "true" | "false" => TokenType::BooleanLiteral,
            _ => TokenType::Identifier,
        };

        let size = self.position - start;

        self.create_token(ty, start, size);
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

        let size = self.position - start;
        let ty = TokenType::NumberLiteral;

        self.create_token(ty, start, size);
    }

    fn string_literal(&mut self) -> Result<(), ErrorType> {
        self.position += 1;

        let start = self.position;

        while !self.at_end() {
            let c = self.source[self.position];

            if c == '"' {
                break;
            }

            if c == '\n' {
                self.line += 1;
            }

            self.position += 1;
        }

        if self.at_end() {
            let err_type = ErrorType::Syntax(SyntaxError::UnexpectedEof);
            return Err(ErrorType::Syntax(SyntaxError::UnexpectedEof));
        }

        self.position += 1;

        let size = self.position - start - 1;
        let ty = TokenType::StringLiteral;
        self.create_token(ty, start, size);

        Ok(())
    }

    pub fn symbol(&mut self) -> Result<(), ErrorType> {
        let start = self.position;

        let ty = match self.source[self.position] {
            '+' => {
                if self.look_ahead(&['+', '+']) {
                    TokenType::Increment
                } else {
                    TokenType::Plus
                }
            }
            '-' => {
                if self.look_ahead(&['-', '-']) {
                    TokenType::Decrement
                } else if self.look_ahead(&['-', '>']) {
                    TokenType::ThinArrow
                } else {
                    TokenType::Minus
                }
            }
            '*' => TokenType::Multiply,
            '/' => TokenType::Divide,
            '%' => TokenType::Remainder,

            '&' => {
                if self.look_ahead(&['&', '&']) {
                    TokenType::And
                } else {
                    TokenType::Invalid
                }
            }
            '|' => {
                if self.look_ahead(&['|', '|']) {
                    TokenType::Or
                } else {
                    TokenType::Invalid
                }
            }
            '!' => {
                if self.look_ahead(&['!', '=']) {
                    TokenType::NotEqual
                } else {
                    TokenType::Not
                }
            }
            '=' => {
                if self.look_ahead(&['=', '=']) {
                    TokenType::Equal
                } else {
                    TokenType::Assign
                }
            }
            '>' => {
                if self.look_ahead(&['>', '=']) {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }
            '<' => {
                if self.look_ahead(&['<', '=']) {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            }
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            ';' => TokenType::Semicolon,
            ':' => TokenType::Colon,
            _ => TokenType::Invalid,
        };

        if ty == TokenType::Invalid {
            let lexeme = self.source[self.position];
            return Err(ErrorType::Syntax(SyntaxError::InvalidToken(lexeme)));
        }

        let size = match ty {
            TokenType::Increment
            | TokenType::Decrement
            | TokenType::And
            | TokenType::Or
            | TokenType::NotEqual
            | TokenType::Equal
            | TokenType::GreaterEqual
            | TokenType::LessEqual
            | TokenType::ThinArrow => 2,
            _ => 1,
        };

        self.position += size;

        self.create_token(ty, start, size);
        Ok(())
    }

    pub fn get_next_token(&mut self) -> Result<(), ErrorType> {
        let c = self.source[self.position];

        if c == '"' {
            self.string_literal()?;
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

    pub fn tokenize(&mut self) -> Result<Vec<Token>, ErrorType> {
        while !self.at_end() {
            self.get_next_token()?;
        }

        Ok(self.tokens.clone())
    }
}
