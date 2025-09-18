use crate::{error::kaori_error::KaoriError, kaori_error};

use super::{span::Span, token::Token, token_kind::TokenKind};

#[derive(Clone)]
pub struct TokenStream {
    source: String,
    tokens: Vec<Token>,
    index: usize,
}

impl TokenStream {
    pub fn new(source: String, tokens: Vec<Token>) -> Self {
        Self {
            source,
            tokens,
            index: 0,
        }
    }

    pub fn at_end(&mut self) -> bool {
        self.token_kind() == TokenKind::EndOfFile
    }

    pub fn advance(&mut self) {
        self.index += 1;
    }

    pub fn consume(&mut self, expected: TokenKind) -> Result<(), KaoriError> {
        let found = self.token_kind();

        if expected == found {
            self.advance();
            Ok(())
        } else {
            let span = self.span();

            Err(kaori_error!(
                span,
                "expected {}, but found {}",
                expected,
                found,
            ))
        }
    }

    pub fn look_ahead(&mut self, expected: &[TokenKind]) -> bool {
        for (i, expected) in expected.iter().enumerate() {
            let j = self.index + i;

            let matched = match self.tokens.get(j) {
                Some(token) => token.kind == *expected,
                None => false,
            };

            if !matched {
                return false;
            }
        }

        true
    }

    pub fn token_kind(&self) -> TokenKind {
        let token = self.tokens.get(self.index).unwrap();

        token.kind
    }

    pub fn span(&self) -> Span {
        self.tokens[self.index].span
    }

    pub fn lexeme(&self) -> &str {
        let span = self.span();

        &self.source[span.start..span.end]
    }
}
