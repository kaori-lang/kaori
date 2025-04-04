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
                return Some(Token::Number(number));
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

        return Some(Token::Number(number));
    }

    pub fn get_next_symbol(&mut self) -> Option<Token> {
        let Some(c) = self.look_ahead() else {
            return None;
        };

        let symbol = match c {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Multiply),
            '/' => Some(Token::Divide),
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            '{' => Some(Token::LeftBrace),
            '}' => Some(Token::RightBrace),
            '&' => {
                self.advance();

                if let Some('&') = self.look_ahead() {
                    Some(Token::And)
                } else {
                    None
                }
            }
            '|' => {
                self.advance();

                if let Some('|') = self.look_ahead() {
                    Some(Token::Or)
                } else {
                    None
                }
            }
            '!' => {
                self.advance();

                if let Some('=') = self.look_ahead() {
                    Some(Token::NotEqual)
                } else {
                    Some(Token::Not)
                }
            }
            '=' => {
                self.advance();

                if let Some('=') = self.look_ahead() {
                    Some(Token::Equal)
                } else {
                    Some(Token::Assign)
                }
            }
            '>' => {
                self.advance();

                if let Some('=') = self.look_ahead() {
                    Some(Token::GreaterEqual)
                } else {
                    Some(Token::Greater)
                }
            }
            '<' => {
                self.advance();

                if let Some('=') = self.look_ahead() {
                    Some(Token::LessEqual)
                } else {
                    Some(Token::Less)
                }
            }
            _ => None,
        };

        if let Some(token) = symbol {
            self.advance();

            return Some(token);
        }

        return None;
    }

    fn get_next_token(&mut self) -> Token {
        let Some(_c) = self.look_ahead() else {
            return Token::EndOfFile;
        };

        if let Some(token) = self.get_next_symbol() {
            return token;
        }

        if let Some(token) = self.get_next_number() {
            return token;
        }

        return Token::Unknown;
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
