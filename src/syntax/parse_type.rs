use crate::{error::kaori_error::KaoriError, kaori_error, lexer::token_kind::TokenKind};

use super::{parser::Parser, ty::Ty};

impl Parser {
    pub fn parse_type(&mut self) -> Result<Ty, KaoriError> {
        let span = self.token_stream.span();
        let kind = self.token_stream.token_kind();

        Ok(match kind {
            TokenKind::Identifier => self.parse_identifier_type()?,
            TokenKind::Number => {
                self.token_stream.advance();
                Ty::number(span)
            }
            TokenKind::Bool => {
                self.token_stream.advance();
                Ty::bool(span)
            }
            _ => {
                return Err(kaori_error!(
                    span,
                    "expected a valid type, but found: {}",
                    kind,
                ));
            }
        })
    }

    fn parse_identifier_type(&mut self) -> Result<Ty, KaoriError> {
        let span = self.token_stream.span();
        let name = self.token_stream.lexeme().to_owned();

        self.token_stream.consume(TokenKind::Identifier)?;

        let identifier = Ty::identifier(name, span);

        Ok(identifier)
    }
}
