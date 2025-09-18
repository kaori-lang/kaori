use crate::{
    error::kaori_error::KaoriError,
    lexer::{span::Span, token_kind::TokenKind},
};

use super::{decl::Decl, parser::Parser, ty::Ty};

impl Parser {
    pub fn parse_variable_declaration(&mut self) -> Result<Decl, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::Variable)?;

        let name = self.token_stream.lexeme().to_owned();

        self.token_stream.consume(TokenKind::Identifier)?;
        self.token_stream.consume(TokenKind::Colon)?;

        let ty = self.parse_type()?;

        self.token_stream.consume(TokenKind::Assign)?;

        let right = self.parse_expression()?;

        Ok(Decl::variable(name, right, ty, span))
    }

    pub fn parse_function_declaration(&mut self) -> Result<Decl, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::Function)?;

        let name = self.token_stream.lexeme().to_owned();

        self.token_stream.consume(TokenKind::Identifier)?;

        self.token_stream.consume(TokenKind::LeftParen)?;

        let parameters =
            self.parse_comma_separator(Parser::parse_function_parameter, TokenKind::RightParen)?;

        self.token_stream.consume(TokenKind::RightParen)?;

        let mut return_ty = None;

        if self.token_stream.token_kind() == TokenKind::ThinArrow {
            self.token_stream.consume(TokenKind::ThinArrow)?;
            return_ty = Some(self.parse_type()?);
        }

        let ty = Ty::function(
            parameters
                .iter()
                .map(|parameter| parameter.ty.to_owned())
                .collect(),
            return_ty,
            Span::default(),
        );

        let mut body = Vec::new();

        self.token_stream.consume(TokenKind::LeftBrace)?;

        while !self.token_stream.at_end() && self.token_stream.token_kind() != TokenKind::RightBrace
        {
            let node = self.parse_ast_node()?;
            body.push(node);
        }

        self.token_stream.consume(TokenKind::RightBrace)?;

        Ok(Decl::function(name, parameters, body, ty, span))
    }

    pub fn parse_function_parameter(&mut self) -> Result<Decl, KaoriError> {
        let start = self.token_stream.span();

        let name = self.token_stream.lexeme().to_owned();
        self.token_stream.consume(TokenKind::Identifier)?;
        self.token_stream.consume(TokenKind::Colon)?;

        let ty = self.parse_type()?;

        let end = self.token_stream.span();

        let span = Span::merge(start, end);

        Ok(Decl::parameter(name, ty, span))
    }

    pub fn parse_struct_field(&mut self) -> Result<Decl, KaoriError> {
        let start = self.token_stream.span();

        let name = self.token_stream.lexeme().to_owned();
        self.token_stream.consume(TokenKind::Identifier)?;
        self.token_stream.consume(TokenKind::Colon)?;
        let ty = self.parse_type()?;

        let end = self.token_stream.span();

        let span = Span::merge(start, end);

        Ok(Decl::field(name, ty, span))
    }

    pub fn parse_struct_declaration(&mut self) -> Result<Decl, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::Struct)?;

        let name = self.token_stream.lexeme().to_owned();

        self.token_stream.consume(TokenKind::Identifier)?;

        self.token_stream.consume(TokenKind::LeftBrace)?;

        let fields =
            self.parse_comma_separator(Parser::parse_struct_field, TokenKind::RightBrace)?;

        let ty = Ty::struct_(
            fields.iter().map(|field| field.ty.to_owned()).collect(),
            Span::default(),
        );

        self.token_stream.consume(TokenKind::RightBrace)?;

        Ok(Decl::struct_(name, fields, ty, span))
    }
}
