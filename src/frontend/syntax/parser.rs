use crate::{
    error::kaori_error::KaoriError,
    frontend::lexer::{token_kind::TokenKind, token_stream::TokenStream},
    kaori_error,
};

use super::{ast_node::AstNode, decl::Decl, stmt::Stmt};

pub struct Parser {
    pub token_stream: TokenStream,
}

impl Parser {
    pub fn new(token_stream: TokenStream) -> Self {
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

    pub fn parse_ast_node(&mut self) -> Result<AstNode, KaoriError> {
        let declaration = self.parse_declaration()?;

        Ok(if let Some(declaration) = declaration {
            AstNode::from(declaration)
        } else {
            let statement = self.parse_statement()?;
            AstNode::from(statement)
        })
    }

    pub fn parse_declaration(&mut self) -> Result<Option<Decl>, KaoriError> {
        let token_kind = self.token_stream.token_kind();

        let declaration = match self.token_stream.token_kind() {
            TokenKind::Function => Some(self.parse_function_declaration()?),
            TokenKind::Struct => Some(self.parse_struct_declaration()?),
            TokenKind::Variable => Some(self.parse_variable_declaration()?),
            _ => None,
        };

        #[allow(clippy::single_match)]
        match token_kind {
            TokenKind::Variable => {
                self.token_stream.consume(TokenKind::Semicolon)?;
            }
            _ => (),
        };

        Ok(declaration)
    }

    pub fn parse_statement(&mut self) -> Result<Stmt, KaoriError> {
        let token_kind = self.token_stream.token_kind();

        let statement = match token_kind {
            TokenKind::Print => self.parse_print_statement(),
            TokenKind::LeftBrace => self.parse_block_statement(),
            TokenKind::If => self.parse_if_statement(),
            TokenKind::While => self.parse_while_loop_statement(),
            TokenKind::For => self.parse_for_loop_statement(),
            TokenKind::Break => self.parse_break_statement(),
            TokenKind::Continue => self.parse_continue_statement(),
            TokenKind::Return => self.parse_return_statement(),

            _ => {
                let statement = self.parse_expression_statement();
                self.token_stream.consume(TokenKind::Semicolon)?;

                statement
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
