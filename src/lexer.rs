use crate::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    pos: usize,
    line: u32,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let mut lexer = Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            pos: 0,
            line: 0,
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

    pub fn get_next_symbol(&mut self) -> Option<Token> {
        let Some(c) = self.look_ahead() else {
            return None;
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
            '&' => {
                self.advance();

                if let Some('&') = self.look_ahead() {
                    Some(TokenType::And)
                } else {
                    should_advance = false;
                    Some(TokenType::Invalid)
                }
            }
            '|' => {
                self.advance();

                if let Some('|') = self.look_ahead() {
                    Some(TokenType::Or)
                } else {
                    should_advance = false;
                    Some(TokenType::Invalid)
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

            return Some(Token::new(token_type, self.line, None));
        }

        return None;
    }

    fn get_next_token(&mut self) -> Option<Token> {
        let Some(_) = self.look_ahead() else {
            return Some(Token::new(TokenType::EndOfFile, self.line, None));
        };

        if let Some(token) = self.get_next_symbol() {
            return Some(token);
        }

        if let Some(token) = self.get_next_number() {
            return Some(token);
        }

        return None;
    }

    fn tokenize(&mut self) {
        loop {
            self.skip_white_space();

            let Some(token) = self.get_next_token() else {
                println!("Invalid token found");
                break;
            };

            if token.ty == TokenType::EndOfFile {
                self.tokens.push(token);
                break;
            }

            self.tokens.push(token);
        }
    }
}
