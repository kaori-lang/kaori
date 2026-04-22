use crate::{
    error::kaori_error::KaoriError,
    lexer::{span::Span, token_kind::TokenKind},
};

use super::{decl::Decl, parser::Parser};

impl<'a> Parser<'a> {
    pub fn parse_function_declaration(&mut self) -> Result<Decl, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::Function)?;

        let name = self.token_stream.lexeme().to_owned();

        self.token_stream.consume(TokenKind::Identifier)?;

        self.token_stream.consume(TokenKind::LeftParen)?;

        let parameters =
            self.parse_comma_separator(Parser::parse_function_parameter, TokenKind::RightParen)?;

        self.token_stream.consume(TokenKind::RightParen)?;

        let mut body = Vec::new();

        self.token_stream.consume(TokenKind::LeftBrace)?;

        while !self.token_stream.at_end() && self.token_stream.token_kind() != TokenKind::RightBrace
        {
            let statement = self.parse_statement()?;
            body.push(statement);
        }

        self.token_stream.consume(TokenKind::RightBrace)?;

        Ok(Decl::function(name, parameters, body, span))
    }

    pub fn parse_function_parameter(&mut self) -> Result<(String, Span), KaoriError> {
        let name = self.token_stream.lexeme().to_owned();
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::Identifier)?;

        Ok((name, span))
    }
}
