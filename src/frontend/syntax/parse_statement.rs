use crate::{error::kaori_error::KaoriError, frontend::scanner::token_kind::TokenKind};

use super::{ast_node::AstNode, parser::Parser, stmt::Stmt};

impl Parser {
    pub fn parse_return_statement(&mut self) -> Result<Stmt, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::Return)?;

        if self.token_stream.token_kind() == TokenKind::Semicolon {
            let expression = None;
            self.token_stream.consume(TokenKind::Semicolon)?;
            return Ok(Stmt::return_(expression, span));
        }

        let expression = Some(self.parse_expression()?);

        self.token_stream.consume(TokenKind::Semicolon)?;

        Ok(Stmt::return_(expression, span))
    }

    pub fn parse_continue_statement(&mut self) -> Result<Stmt, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::Continue)?;
        self.token_stream.consume(TokenKind::Semicolon)?;

        Ok(Stmt::continue_(span))
    }

    pub fn parse_break_statement(&mut self) -> Result<Stmt, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::Break)?;
        self.token_stream.consume(TokenKind::Semicolon)?;

        Ok(Stmt::break_(span))
    }

    pub fn parse_expression_statement(&mut self) -> Result<Stmt, KaoriError> {
        let span = self.token_stream.span();
        let expression = self.parse_expression()?;

        Ok(Stmt::expression(expression, span))
    }

    pub fn parse_print_statement(&mut self) -> Result<Stmt, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::Print)?;
        self.token_stream.consume(TokenKind::LeftParen)?;
        let expression = self.parse_expression()?;
        self.token_stream.consume(TokenKind::RightParen)?;
        self.token_stream.consume(TokenKind::Semicolon)?;

        Ok(Stmt::print(expression, span))
    }

    pub fn parse_block_statement(&mut self) -> Result<Stmt, KaoriError> {
        let span = self.token_stream.span();

        let mut nodes: Vec<AstNode> = Vec::new();

        self.token_stream.consume(TokenKind::LeftBrace)?;

        while !self.token_stream.at_end() && self.token_stream.token_kind() != TokenKind::RightBrace
        {
            let node = self.parse_ast_node()?;
            nodes.push(node);
        }

        self.token_stream.consume(TokenKind::RightBrace)?;

        Ok(Stmt::block(nodes, span))
    }

    pub fn parse_if_statement(&mut self) -> Result<Stmt, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::If)?;

        let condition = self.parse_expression()?;

        let then_branch = self.parse_block_statement()?;

        if self.token_stream.token_kind() != TokenKind::Else {
            return Ok(Stmt::if_(condition, then_branch, None, span));
        }

        self.token_stream.advance();

        if self.token_stream.token_kind() == TokenKind::If {
            let else_branch = Some(self.parse_if_statement()?);

            return Ok(Stmt::if_(condition, then_branch, else_branch, span));
        }

        let else_branch = Some(self.parse_block_statement()?);

        Ok(Stmt::if_(condition, then_branch, else_branch, span))
    }

    pub fn parse_while_loop_statement(&mut self) -> Result<Stmt, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::While)?;

        let condition = self.parse_expression()?;
        let block = self.parse_block_statement()?;

        Ok(Stmt::while_loop(condition, block, span))
    }

    pub fn parse_for_loop_statement(&mut self) -> Result<Stmt, KaoriError> {
        let span = self.token_stream.span();

        self.token_stream.consume(TokenKind::For)?;

        let declaration = self.parse_variable_declaration()?;

        self.token_stream.consume(TokenKind::Semicolon)?;

        let condition = self.parse_expression()?;

        self.token_stream.consume(TokenKind::Semicolon)?;

        let increment = self.parse_expression_statement()?;

        let block = self.parse_block_statement()?;

        Ok(Stmt::for_loop(
            declaration,
            condition,
            increment,
            block,
            span,
        ))
    }
}
