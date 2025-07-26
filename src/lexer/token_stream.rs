use crate::yf_error::ErrorType;

use super::{token::Token, token_type::TokenType};

#[derive(Debug, Clone)]
pub struct TokenStream {
    tokens: Vec<Token>,
    index: usize,
    line: u32,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0,
            line: 1,
        }
    }

    pub fn current_kind(&mut self) -> TokenType {
        if let Some(token) = self.tokens.get(self.index) {
            return token.ty.clone();
        }

        return TokenType::Eof;
    }

    pub fn at_end(&mut self) -> bool {
        return self.current_kind() == TokenType::Eof;
    }

    pub fn advance(&mut self) {
        self.index += 1;

        if let Some(token) = self.tokens.get(self.index) {
            self.line = token.line;
        }
    }

    pub fn consume(&mut self, expected: TokenType) -> Result<(), ErrorType> {
        let Some(token) = self.tokens.get(self.index) else {
            return Err(ErrorType::SyntaxError);
        };

        if expected == token.ty {
            self.advance();
            return Ok(());
        } else {
            return Err(ErrorType::SyntaxError);
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
