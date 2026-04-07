use crate::{
    error::kaori_error::KaoriError,
    kaori_error,
    lexer::{token_kind::TokenKind, token_stream::TokenStream},
};

use super::{decl::Decl, node::Node, stmt::Stmt};

pub struct Parser<'a> {
    pub token_stream: TokenStream<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(token_stream: TokenStream<'a>) -> Self {
        Self { token_stream }
    }

    pub fn parse(&mut self) -> Result<Vec<Decl>, KaoriError> {
        let mut declarations = Vec::new();

        while !self.token_stream.at_end() {
            let declaration = self.parse_declaration()?;

            let declaration = match declaration {
                Some(decl) => Ok(decl),
                _ => Err(kaori_error!(
                    self.token_stream.span(),
                    "invalid declaration at global scope"
                )),
            }?;

            declarations.push(declaration);
        }

        Ok(declarations)
    }

    pub fn parse_ast_node(&mut self) -> Result<Node, KaoriError> {
        let declaration = self.parse_declaration()?;

        Ok(if let Some(declaration) = declaration {
            Node::from(declaration)
        } else {
            let statement = self.parse_statement()?;
            Node::from(statement)
        })
    }

    pub fn parse_declaration(&mut self) -> Result<Option<Decl>, KaoriError> {
        let declaration = match self.token_stream.token_kind() {
            TokenKind::Function => Some(self.parse_function_declaration()?),

            _ => None,
        };

        Ok(declaration)
    }

    pub fn parse_statement(&mut self) -> Result<Stmt, KaoriError> {
        let token_kind = self.token_stream.token_kind();

        let statement = match token_kind {
            TokenKind::Print => self.parse_print_statement(),
            TokenKind::If => self.parse_if_statement(),
            TokenKind::While => self.parse_while_loop_statement(),
            TokenKind::For => self.parse_for_loop_statement(),
            TokenKind::Break => self.parse_break_statement(),
            TokenKind::Continue => self.parse_continue_statement(),
            TokenKind::Return => self.parse_return_statement(),

            _ => {
                let statement = self.parse_expression_statement();

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
