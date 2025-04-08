use std::{iter::Peekable, str::Chars};

use crate::token::{Token, TokenType};

#[derive(Debug)]
pub enum LexerError {
    InvalidToken { line: u32, character: char },
    InvalidString { line: u32, character: char },
}

#[derive(Debug)]
pub struct Lexer {
    source: &'static str,
    curr: Peekable<Chars<'static>>,
    line: u32,
}

impl Lexer {
    pub fn new(source: &'static str) -> Self {
        Self {
            source,
            curr: source.chars().peekable(),
            line: 1,
        }
    }

    fn look_ahead(&mut self) -> Option<char> {
        return self.curr.peek().copied();
    }

    fn advance(&mut self) {
        self.curr.next();
    }

    fn skip_white_space(&mut self) {
        while let Some(c) = self.look_ahead() {
            if c == '\n' {
                self.line += 1;
                self.advance();
            } else if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn get_next_identifier(&mut self) -> Option<Token> {
        let Some('a'..='z' | 'A'..='Z' | '_') = self.look_ahead() else {
            return None;
        };

        let mut identifier = String::new();

        while let Some(c) = self.look_ahead() {
            if !matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9') {
                break;
            }
            self.advance();
            identifier.push(c);
        }

        let token_type = match identifier.as_str() {
            "Number" => {
                return Some(Token::new(
                    TokenType::DataType,
                    self.line,
                    Some(String::from("number")),
                ))
            }
            "String" => {
                return Some(Token::new(
                    TokenType::DataType,
                    self.line,
                    Some(String::from("string")),
                ))
            }
            "Boolean" => {
                return Some(Token::new(
                    TokenType::DataType,
                    self.line,
                    Some(String::from("boolean")),
                ))
            }
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "return" => TokenType::Return,
            "def" => TokenType::Def,
            "true" => TokenType::Boolean,
            "false" => TokenType::Boolean,
            _ => {
                return Some(Token::new(
                    TokenType::Identifier,
                    self.line,
                    Some(identifier),
                ))
            }
        };

        return Some(Token::new(token_type, self.line, None));
    }

    fn get_next_number(&mut self) -> Option<Token> {
        let Some('0'..='9') = self.look_ahead() else {
            return None;
        };

        let mut number = String::new();

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
                return Some(Token::new(TokenType::Number, self.line, Some(number)));
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

        return Some(Token::new(TokenType::Number, self.line, Some(number)));
    }

    pub fn get_next_symbol(&mut self) -> Result<Option<Token>, LexerError> {
        let Some(c) = self.look_ahead() else {
            return Ok(None);
        };

        let mut should_advance = true;

        let symbol = match c {
            '+' => Some(TokenType::Plus),
            '-' => Some(TokenType::Minus),
            '*' => Some(TokenType::Multiply),
            '/' => Some(TokenType::Divide),
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            ',' => Some(TokenType::Comma),
            ';' => Some(TokenType::Semicolon),
            '&' => {
                self.advance();

                if let Some('&') = self.look_ahead() {
                    Some(TokenType::And)
                } else {
                    return Err(LexerError::InvalidToken {
                        line: self.line,
                        character: c,
                    });
                }
            }
            '|' => {
                self.advance();

                if let Some('|') = self.look_ahead() {
                    Some(TokenType::Or)
                } else {
                    return Err(LexerError::InvalidToken {
                        line: self.line,
                        character: c,
                    });
                }
            }
            '!' => {
                self.advance();

                if let Some('=') = self.look_ahead() {
                    Some(TokenType::NotEqual)
                } else {
                    should_advance = false;
                    Some(TokenType::Not)
                }
            }
            '=' => {
                self.advance();

                if let Some('=') = self.look_ahead() {
                    Some(TokenType::Equal)
                } else {
                    should_advance = false;
                    Some(TokenType::Assign)
                }
            }
            '>' => {
                self.advance();

                if let Some('=') = self.look_ahead() {
                    Some(TokenType::GreaterEqual)
                } else {
                    should_advance = false;
                    Some(TokenType::Greater)
                }
            }
            '<' => {
                self.advance();

                if let Some('=') = self.look_ahead() {
                    Some(TokenType::LessEqual)
                } else {
                    should_advance = false;
                    Some(TokenType::Less)
                }
            }
            _ => None,
        };

        if let Some(token_type) = symbol {
            if should_advance {
                self.advance();
            }

            return Ok(Some(Token::new(token_type, self.line, None)));
        }

        return Ok(None);
    }

    fn get_next_token(&mut self) -> Result<Token, LexerError> {
        let Some(c) = self.look_ahead() else {
            return Ok(Token::new(TokenType::EndOfFile, self.line, None));
        };

        let symbol = self.get_next_symbol()?;

        if let Some(token) = symbol {
            return Ok(token);
        }

        if let Some(token) = self.get_next_number() {
            return Ok(token);
        }

        if let Some(token) = self.get_next_identifier() {
            return Ok(token);
        }

        return Err(LexerError::InvalidToken {
            line: self.line,
            character: c,
        });
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        self.curr = self.source.chars().peekable();

        loop {
            self.skip_white_space();

            let token = self.get_next_token()?;

            if token.ty == TokenType::EndOfFile {
                tokens.push(token);
                break;
            }

            tokens.push(token);
        }

        return Ok(tokens);
    }
}
