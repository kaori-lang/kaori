use crate::{error::kaori_error::KaoriError, frontend::lexer::token_kind::TokenKind, kaori_error};

use super::{parser::Parser, ty::Ty};

impl Parser {
    pub fn parse_type(&mut self) -> Result<Ty, KaoriError> {
        let span = self.token_stream.span();
        let kind = self.token_stream.token_kind();

        match kind {
            TokenKind::Identifier => self.parse_identifier_type(),
            _ => Err(kaori_error!(
                span,
                "expected a valid type, but found: {}",
                kind,
            )),
        }
    }

    fn parse_identifier_type(&mut self) -> Result<Ty, KaoriError> {
        let span = self.token_stream.span();
        let name = self.token_stream.lexeme().to_owned();

        let identifier = Ty::identifier(name, span);

        self.token_stream.consume(TokenKind::Identifier)?;

        Ok(identifier)
    }
}
