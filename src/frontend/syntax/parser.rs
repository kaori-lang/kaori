use crate::{
    error::kaori_error::KaoriError,
    frontend::lexer::{token_kind::TokenKind, token_stream::TokenStream},
    kaori_error,
};

use super::{ast_node::AstNode, decl::Decl};

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
            let declaration = match self.token_stream.token_kind() {
                TokenKind::Function => self.parse_function_declaration(),
                TokenKind::Struct => self.parse_struct_declaration(),
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
        let stmt = match self.token_stream.token_kind() {
            TokenKind::Print => self.parse_print_statement(),
            TokenKind::LeftBrace => self.parse_block_statement(),
            TokenKind::If => self.parse_if_statement(),
            TokenKind::While => self.parse_while_loop_statement(),
            TokenKind::For => self.parse_for_loop_statement(),
            TokenKind::Break => self.parse_break_statement(),
            TokenKind::Continue => self.parse_continue_statement(),
            TokenKind::Return => self.parse_return_statement(),
            _ => {
                if self
                    .token_stream
                    .look_ahead(&[TokenKind::Identifier, TokenKind::Colon])
                {
                    let declaration = self.parse_variable_declaration()?;
                    self.token_stream.consume(TokenKind::Semicolon)?;
                    return Ok(AstNode::Declaration(declaration));
                } else {
                    let statement = self.parse_expression_statement();
                    self.token_stream.consume(TokenKind::Semicolon)?;

                    statement
                }
            }
        }?;

        Ok(AstNode::Statement(stmt))
    }
}
