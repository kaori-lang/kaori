use regex::Regex;

use crate::yf_error::{ErrorType, YFError};

use super::{
    data::Data,
    token::{Token, TokenType},
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

        return Some(Token {
            ty: TokenType::Literal,
            line: self.line,
            lexeme: raw_string.to_string(),
            literal: Data::String(raw_string.to_string()),
        });
    }

    fn get_next_identifier(&mut self) -> Option<Token> {
        let Some(identifier) = self.identifier_re.find(&self.curr) else {
            return None;
        };

        self.advance(identifier.as_str());

        let token_type = match identifier.as_str() {
            "String" => TokenType::String,
            "Number" => TokenType::Number,
            "Boolean" => TokenType::Boolean,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "return" => TokenType::Return,
            "def" => TokenType::Def,
            "print" => TokenType::Print,
            "true" => {
                return Some(Token {
                    ty: TokenType::Literal,
                    line: self.line,
                    lexeme: "true".to_string(),
                    literal: Data::Boolean(true),
                });
            }
            "false" => {
                return Some(Token {
                    ty: TokenType::Literal,
                    line: self.line,
                    lexeme: "false".to_string(),
                    literal: Data::Boolean(false),
                });
            }
            _ => TokenType::Identifier,
        };

        return Some(Token {
            ty: token_type,
            line: self.line,
            lexeme: identifier.as_str().to_string(),
            literal: Data::None,
        });
    }

    fn get_next_number(&mut self) -> Option<Token> {
        let Some(number) = self.number_re.find(&self.curr) else {
            return None;
        };

        self.advance(number.as_str());

        let literal = Data::Number(number.as_str().parse::<f64>().unwrap());

        return Some(Token {
            ty: TokenType::Literal,
            line: self.line,
            lexeme: number.as_str().to_string(),
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
            self.curr = &self.curr[2..];

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
            self.curr = &self.curr[1..];

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

        return Err(ErrorType::SyntaxError);
    }

    fn reset(&mut self) {
        self.curr = &self.source[..];
        self.line = 1;
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

        self.reset();

        return Ok(tokens);
    }
}
