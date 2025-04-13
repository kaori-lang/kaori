use regex::Regex;

use crate::{
    token::{DataType, Token, TokenType},
    yf_error::{ErrorType, YFError},
};

#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    curr: &'a str,
    line: u32,
    number_re: Regex,
    identifier_re: Regex,
    string_re: Regex,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            curr: source,
            line: 1,
            number_re: Regex::new(r"^\d+(\.\d*)?").unwrap(),
            identifier_re: Regex::new(r"^[_a-zA-Z][_a-zA-Z0-9]*").unwrap(),
            string_re: Regex::new(r#"^"[^"]*""#).unwrap(),
        }
    }

    fn advance(&mut self, prefix: &str) -> Option<&str> {
        if let Some(new_curr) = self.curr.strip_prefix(prefix) {
            self.curr = new_curr;
            return Some(self.curr);
        } else {
            return None;
        }
    }

    fn skip_white_space(&mut self) {
        while !self.curr.is_empty() {
            if self.curr.starts_with('\n') {
                self.line += 1;
                self.curr = &self.curr[1..];
            } else if self.curr.starts_with(' ') {
                self.curr = &self.curr[1..];
            } else {
                break;
            }
        }
    }

    fn get_next_string(&mut self) -> Option<Token> {
        let Some(string) = self.string_re.find(&self.curr) else {
            return None;
        };

        self.advance(string.as_str());

        let raw_string = &string.as_str()[1..string.as_str().len() - 1];

        return Some(Token::new(
            TokenType::Literal(DataType::String),
            self.line,
            raw_string.to_string(),
        ));
    }

    fn get_next_identifier(&mut self) -> Option<Token> {
        let Some(identifier) = self.identifier_re.find(&self.curr) else {
            return None;
        };

        self.advance(identifier.as_str());

        let token_type = match identifier.as_str() {
            "Number" => {
                return Some(Token::new(
                    TokenType::VariableDecl(DataType::Number),
                    self.line,
                    "".to_string(),
                ))
            }
            "String" => {
                return Some(Token::new(
                    TokenType::VariableDecl(DataType::String),
                    self.line,
                    "".to_string(),
                ))
            }
            "Boolean" => {
                return Some(Token::new(
                    TokenType::VariableDecl(DataType::Boolean),
                    self.line,
                    "".to_string(),
                ))
            }
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "return" => TokenType::Return,
            "def" => TokenType::Def,
            "print" => TokenType::Print,
            "true" => {
                return Some(Token::new(
                    TokenType::Literal(DataType::Boolean),
                    self.line,
                    "true".to_string(),
                ));
            }
            "false" => {
                return Some(Token::new(
                    TokenType::Literal(DataType::Boolean),
                    self.line,
                    "false".to_string(),
                ));
            }
            _ => {
                return Some(Token::new(
                    TokenType::Identifier,
                    self.line,
                    identifier.as_str().to_string(),
                ))
            }
        };

        return Some(Token::new(token_type, self.line, "".to_string()));
    }

    fn get_next_number(&mut self) -> Option<Token> {
        let Some(number) = self.number_re.find(&self.curr) else {
            return None;
        };

        self.advance(number.as_str());

        return Some(Token::new(
            TokenType::Literal(DataType::Number),
            self.line,
            number.as_str().to_string(),
        ));
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
            self.curr = &self.curr[2..];
            return Some(Token::new(token_type, self.line, "".to_string()));
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
            self.curr = &self.curr[1..];
            return Some(Token::new(token_type, self.line, "".to_string()));
        }

        return None;
    }

    fn get_next_token(&mut self) -> Result<Token, ErrorType> {
        if self.curr.is_empty() {
            return Err(ErrorType::EndOfFile);
        }

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

        return Err(ErrorType::InvalidToken(self.curr.chars().next().unwrap()));
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, YFError> {
        let mut tokens = Vec::new();

        loop {
            self.skip_white_space();

            let token = match self.get_next_token() {
                Ok(token) => token,
                Err(ErrorType::EndOfFile) => break,
                Err(error_type) => {
                    return Err(YFError {
                        error_type,
                        line: self.line,
                    })
                }
            };

            tokens.push(token);
        }

        return Ok(tokens);
    }
}
