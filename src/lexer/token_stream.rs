use crate::error::syntax_error::{Syntax, SyntaxError};

use super::{token::Token, token_type::TokenType};

#[derive(Debug, Clone)]
pub struct TokenStream {
    source: String,
    tokens: Vec<Token>,
    index: usize,
    line: u32,
}

impl TokenStream {
    pub fn new(source: String, tokens: Vec<Token>) -> Self {
        Self {
            source,
            tokens,
            index: 0,
            line: 1,
        }
    }

    pub fn current_type(&mut self) -> TokenType {
        if let Some(token) = self.tokens.get(self.index) {
            return token.ty.clone();
        }

        return TokenType::Eof;
    }

    pub fn at_end(&mut self) -> bool {
        return self.current_type() == TokenType::Eof;
    }

    pub fn advance(&mut self) {
        self.index += 1;

        if let Some(token) = self.tokens.get(self.index) {
            self.line = token.line;
        }
    }

    pub fn current_line(&mut self) -> u32 {
        return self.line;
    }

    pub fn lexeme(&mut self) -> String {
        let current_token = &self.tokens[self.index];
        let start = current_token.position;
        let end = current_token.position + current_token.size;

        return self.source[start..end].to_string();
    }

    pub fn consume(&mut self, expected: TokenType) -> Result<(), SyntaxError> {
        let found = self.current_type();

        if expected == found {
            self.advance();
            return Ok(());
        } else {
            let err = SyntaxError {
                error_type: Syntax::ExpectedToken(expected, found),
                line: self.current_line(),
            };

            return Err(err);
        }
    }

    pub fn look_ahead(&mut self, expected: &[TokenType]) -> bool {
        for i in 0..expected.len() {
            let j = self.index + i;

            if j >= self.tokens.len() {
                return false;
            }

            if self.tokens[j].ty == expected[i] {
                continue;
            }

            return false;
        }

        return true;
    }
}
