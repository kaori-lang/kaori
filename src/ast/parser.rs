use crate::{
    ast::Expr,
    error::kaori_error::KaoriError,
    lexer::{token_kind::TokenKind, token_stream::TokenStream},
};

pub struct Parser<'a> {
    pub token_stream: TokenStream<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(token_stream: TokenStream<'a>) -> Self {
        Self { token_stream }
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>, KaoriError> {
        let mut functions = Vec::new();

        while !self.token_stream.at_end() {
            let function = self.parse_function()?;

            functions.push(function);
        }

        Ok(functions)
    }

    pub fn parse_statement_expression(&mut self) -> Result<Stmt, KaoriError> {
        let token_kind = self.token_stream.token_kind();

        let statement = match token_kind {
            TokenKind::Print => self.parse_print(),
            TokenKind::If => self.parse_if(),
            TokenKind::While => self.parse_while_loop(),
            TokenKind::For => self.parse_for_loop(),
            TokenKind::Break => self.parse_break(),
            TokenKind::Continue => self.parse_continue(),
            TokenKind::Return => self.parse_return(),
            TokenKind::Unchecked => self.parse_unchecked_block(),
            _ => {
                let statement = self.parse_expression();

                if statement.is_ok() {
                    self.token_stream.consume(TokenKind::Semicolon)?;
                    statement
                } else {
                    statement
                }
            }
        }?;

        match token_kind {
            TokenKind::Print | TokenKind::Break | TokenKind::Continue | TokenKind::Return => {
                self.token_stream.consume(TokenKind::Semicolon)?;
            }
            _ => (),
        };

        Ok(statement)
    }

    pub fn parse_comma_separator<T>(
        &mut self,
        parse_item: fn(&mut Self) -> Result<T, KaoriError>,
        terminator: TokenKind,
    ) -> Result<Vec<T>, KaoriError> {
        let mut items = Vec::new();

        while !self.token_stream.at_end() && self.token_stream.token_kind() != terminator {
            let item = parse_item(self)?;
            items.push(item);

            if self.token_stream.token_kind() == terminator {
                break;
            }

            self.token_stream.consume(TokenKind::Comma)?;
        }

        Ok(items)
    }
}
