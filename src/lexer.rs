use regex::Regex;

use crate::token::{Token, TokenType};

#[derive(Debug)]
pub enum LexerError {
    InvalidToken { line: u32, character: char },
    InvalidString { line: u32, character: char },
}

#[derive(Debug)]
pub struct Lexer {
    source: &'static str,
    curr: &'static str,
    line: u32,
    number_re: Regex,
    identifier_re: Regex,
    string_re: Regex,
}

impl Lexer {
    pub fn new(source: &'static str) -> Self {
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
            TokenType::String,
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
                    TokenType::DataType,
                    self.line,
                    " number".to_string(),
                ))
            }
            "String" => {
                return Some(Token::new(
                    TokenType::DataType,
                    self.line,
                    "string".to_string(),
                ))
            }
            "Boolean" => {
                return Some(Token::new(
                    TokenType::DataType,
                    self.line,
                    "boolean".to_string(),
                ))
            }
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "return" => TokenType::Return,
            "def" => TokenType::Def,
            "true" => {
                return Some(Token::new(
                    TokenType::Boolean,
                    self.line,
                    "true".to_string(),
                ));
            }
            "false" => {
                return Some(Token::new(
                    TokenType::Boolean,
                    self.line,
                    "true".to_string(),
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
            TokenType::Number,
            self.line,
            number.as_str().to_string(),
        ));
    }

    pub fn get_next_symbol(&mut self) -> Option<Token> {
        let symbol = {
            if let Some(_) = self.advance("&&") {
                Some(TokenType::And)
            } else if let Some(_) = self.advance("||") {
                Some(TokenType::Or)
            } else if let Some(_) = self.advance("!=") {
                Some(TokenType::NotEqual)
            } else if let Some(_) = self.advance("==") {
                Some(TokenType::Equal)
            } else if let Some(_) = self.advance(">=") {
                Some(TokenType::GreaterEqual)
            } else if let Some(_) = self.advance("<=") {
                Some(TokenType::LessEqual)
            } else if let Some(_) = self.advance("+") {
                Some(TokenType::Plus)
            } else if let Some(_) = self.advance("-") {
                Some(TokenType::Minus)
            } else if let Some(_) = self.advance("*") {
                Some(TokenType::Multiply)
            } else if let Some(_) = self.advance("/") {
                Some(TokenType::Divide)
            } else if let Some(_) = self.advance("(") {
                Some(TokenType::LeftParen)
            } else if let Some(_) = self.advance(")") {
                Some(TokenType::RightParen)
            } else if let Some(_) = self.advance("{") {
                Some(TokenType::LeftBrace)
            } else if let Some(_) = self.advance("}") {
                Some(TokenType::RightBrace)
            } else if let Some(_) = self.advance(",") {
                Some(TokenType::Comma)
            } else if let Some(_) = self.advance(";") {
                Some(TokenType::Semicolon)
            } else if let Some(_) = self.advance("!") {
                Some(TokenType::Not)
            } else if let Some(_) = self.advance("=") {
                Some(TokenType::Assign)
            } else if let Some(_) = self.advance(">") {
                Some(TokenType::Greater)
            } else if let Some(_) = self.advance("<") {
                Some(TokenType::Less)
            } else {
                None
            }
        };

        if let Some(token_type) = symbol {
            return Some(Token::new(token_type, self.line, "".to_string()));
        }

        return None;
    }

    fn get_next_token(&mut self) -> Result<Token, LexerError> {
        if self.curr.is_empty() {
            return Ok(Token::new(
                TokenType::EndOfFile,
                self.line,
                "\0".to_string(),
            ));
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

        return Err(LexerError::InvalidToken {
            line: self.line,
            character: '\n',
        });
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

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
