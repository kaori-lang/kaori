use crate::yf_error::{ErrorType, YFError};

use super::{token::Token, token_type::TokenType};

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    line: u32,
    position: usize,
}

impl Lexer {
    pub fn new(source: Vec<char>) -> Self {
        Self {
            source,
            line: 1,
            position: 0,
        }
    }

    fn advance(&mut self, steps: usize) {
        let end = self.position + steps;

        for i in self.position..end {
            if let Some('\n') = self.source.get(i) {
                self.line += 1;
            }

            self.position += 1;
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

    fn white_space(&mut self) {
        let size = self.position;

        while !self.at_end() {
            let c = self.source[self.position];

            if !c.is_whitespace() {
                break;
            }

            if c == '\n' {
                self.line += 1;
            }
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

        let ty = match self.source[start..self.position].iter().collect() {
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "for" => TokenType::For,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "return" => TokenType::Return,
            "def" => TokenType::Function,
            "print" => TokenType::Print,
            "true" | "false" => TokenType::Boolean,
            _ => TokenType::Identifier,
        };
    }

    fn get_next_number(&mut self) -> Option<Token> {
        let Some(number) = self.number_re.find(&self.curr) else {
            return None;
        };

        self.advance(number.as_str());

        let lexeme = number.as_str().to_string();
        let literal = Data::Float(lexeme.parse::<f64>().unwrap());

        return Some(Token {
            ty: TokenType::Literal,
            line: self.line,
            lexeme,
            literal,
        });
    }

    pub fn get_next_two_char_symbol(&mut self) -> Option<Token> {
        if self.curr.len() < 2 {
            return None;
        }

        if let Some(token_type) = match &self.curr[..2] {
            "&&" => Some(TokenType::And),
            "||" => Some(TokenType::Or),
            "!=" => Some(TokenType::NotEqual),
            "==" => Some(TokenType::Equal),
            ">=" => Some(TokenType::GreaterEqual),
            "<=" => Some(TokenType::LessEqual),
            _ => None,
        } {
            let lexeme = String::from(&self.curr[..2]);
            self.advance(&lexeme);

            return Some(Token {
                ty: token_type,
                line: self.line,
                lexeme,
                literal: Data::None,
            });
        }
        return None;
    }

    pub fn get_next_symbol(&mut self) -> Option<Token> {
        if let Some(token) = self.get_next_two_char_symbol() {
            return Some(token);
        }

        if self.curr.is_empty() {
            return None;
        }

        if let Some(token_type) = match &self.curr[..1] {
            "+" => Some(TokenType::Plus),
            "-" => Some(TokenType::Minus),
            "*" => Some(TokenType::Multiply),
            "/" => Some(TokenType::Divide),
            "%" => Some(TokenType::Remainder),
            "(" => Some(TokenType::LeftParen),
            ")" => Some(TokenType::RightParen),
            "{" => Some(TokenType::LeftBrace),
            "}" => Some(TokenType::RightBrace),
            "," => Some(TokenType::Comma),
            ";" => Some(TokenType::Semicolon),
            "!" => Some(TokenType::Not),
            "=" => Some(TokenType::Assign),
            ">" => Some(TokenType::Greater),
            "<" => Some(TokenType::Less),
            _ => None,
        } {
            let lexeme = String::from(&self.curr[..1]);
            self.advance(&lexeme);

            return Some(Token {
                ty: token_type,
                line: self.line,
                lexeme,
                literal: Data::None,
            });
        }

        return None;
    }

    fn get_next_token(&mut self) -> Result<Token, ErrorType> {
        if let Some(token) = self.get_next_symbol() {
            return Ok(token);
        }

        if let Some(token) = self.get_next_number() {
            return Ok(token);
        }

        if let Some(token) = self.get_next_string() {
            return Ok(token);
        }

        if let Some(token) = self.get_next_identifier() {
            return Ok(token);
        }

        return Err(ErrorType::SyntaxError);
    }

    fn reset(&mut self) {
        self.curr = &self.source[..];
        self.line = 1;
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, YFError> {
        let mut tokens = Vec::new();

        while !self.curr.is_empty() {
            if self.get_next_white_space() || self.get_next_new_line() {
                continue;
            }

            let token = match self.get_next_token() {
                Ok(token) => token,
                Err(error_type) => {
                    return Err(YFError {
                        error_type,
                        line: self.line,
                    });
                }
            };

            tokens.push(token);
        }

        self.reset();

        return Ok(tokens);
    }
}
