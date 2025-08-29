use crate::{
    error::kaori_error::KaoriError,
    frontend::scanner::{span::Span, token_kind::TokenKind, token_stream::TokenStream},
    kaori_error,
};

use super::{
    ast_node::AstNode,
    decl::{Decl, Field, Parameter},
    ty::Ty,
};

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
            declarations.push(declaration);
        }

        Ok(declarations)
    }

    fn parse_declaration(&mut self) -> Result<Decl, KaoriError> {
        let declaration = match self.token_stream.token_kind() {
            TokenKind::Function => self.parse_function_declaration(),
            _ => Err(kaori_error!(
                self.token_stream.span(),
                "invalid declaration at global scope"
            )),
        }?;

        Ok(declaration)
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

    /* Types */
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
