use std::collections::VecDeque;

use crate::token::Token;

pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self { tokens }
    }

    pub fn show_tokens(&self) -> VecDeque<Token> {
        return self.tokens.clone();
    }
}
