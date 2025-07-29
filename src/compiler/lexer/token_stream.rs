use crate::{
    error::kaori_error::{self, KaoriError},
    kaori_error,
};

use super::{span::Span, token::Token, token_kind::TokenKind};

#[derive(Debug, Clone)]
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
        return self.token_kind() == TokenKind::EndOfFile;
    }

    pub fn advance(&mut self) {
        self.index += 1;
    }

    pub fn consume(&mut self, expected: TokenKind) -> Result<(), KaoriError> {
        let found = self.token_kind();

        if expected == found {
            self.advance();
            return Ok(());
        } else {
            let span = self.span();

            return Err(kaori_error!(
                span,
                "expected {:?}, but found {:?}",
                expected,
                found,
            ));
        }
    }

    pub fn look_ahead(&mut self, expected: &[TokenKind]) -> bool {
        for i in 0..expected.len() {
            let j = self.index + i;

            if j >= self.tokens.len() {
                return false;
            }

            if self.tokens[j].kind == expected[i] {
                continue;
            }

            return false;
        }

        return true;
    }

    pub fn token_kind(&mut self) -> TokenKind {
        let token = self.tokens.get(self.index).unwrap();

        return token.kind.clone();
    }

    pub fn span(&mut self) -> Span {
        return self.tokens[self.index].span;
    }

    pub fn lexeme(&mut self) -> String {
        let span = self.span();

        return self.source[span.start..span.end].to_string();
    }
}
