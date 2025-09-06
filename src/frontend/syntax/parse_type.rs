use crate::{error::kaori_error::KaoriError, frontend::lexer::token_kind::TokenKind, kaori_error};

use super::{parser::Parser, ty::Ty};

impl Parser {
    pub fn parse_type(&mut self) -> Result<Ty, KaoriError> {
        match self.token_stream.token_kind() {
            TokenKind::Identifier => self.parse_primitive_type(),
            _ => Err(kaori_error!(
                self.token_stream.span(),
                "expected a valid type, but found: {}",
                self.token_stream.token_kind(),
            )),
        }
    }

    fn parse_primitive_type(&mut self) -> Result<Ty, KaoriError> {
        let span = self.token_stream.span();
        let name = self.token_stream.lexeme();

        let primitive = match name {
            "bool" => Ty::boolean(span),
            "number" => Ty::number(span),
            _ => Ty::custom(name.to_owned(), span),
        };

        self.token_stream.advance();

        Ok(primitive)
    }
}
