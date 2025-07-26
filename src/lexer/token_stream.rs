use super::{
    token::{Token, TokenType},
    token_type::TokenType,
};

pub struct TokenStream {
    tokens: Vec<Token>,
    index: usize,
    line: u32
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0,
            line: 1
        }
    }

    pub fn at_end(&mut self) -> bool {
        return self.index >= self.tokens.len();
    }

    pub fn current_kind(&mut self) -> TokenType {
        if let Some(token) = self.tokens.get(self.index) {
            return token.ty.clone();
        }

        return TokenType::Eof;
    }

    pub fn advance(&mut self) {
        self.index += 1;

        if let Some(token) = self.tokens.get(self.index) {
            self.in
        }
    }
}
