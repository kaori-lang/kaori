use crate::{
    compilation_error,
    compiler::{
        lexer::span::Span,
        syntax::{
            ast_node::ASTNode, declaration_ast::DeclarationAST, expression_ast::ExpressionAST,
            statement_ast::StatementAST, type_ast::TypeAST,
        },
    },
    error::compilation_error::{self, CompilationError},
};

use super::{environment::Environment, visitor::Visitor};

pub struct Resolver {
    declarations: Vec<ASTNode>,
    environment: Environment<String>,
    span: Span,
}

pub struct Resolution {
    offset: usize,
    global: bool,
}

impl Resolver {
    fn new(declarations: Vec<ASTNode>) -> Self {
        Self {
            declarations,
            environment: Environment::new(),
            span: Span {
                line: 1,
                start: 0,
                size: 0,
            },
        }
    }

    fn search_current_scope(&mut self, identifier: &str) -> Option<Resolution> {
        let start = self.environment.stack.len() - 1;
        let end = self.environment.scopes_pointer.last().unwrap();

        for i in start..=*end {
            if identifier == self.environment.stack[i] {
                let global =
                    self.environment.frame_pointer == 0 || i < self.environment.frame_pointer;
                let mut offset = i;

                if !global {
                    offset += self.environment.frame_pointer;
                }

                return Some(Resolution { offset, global });
            }
        }

        return None;
    }

    fn search(&mut self, identifier: &str) -> Option<Resolution> {
        let start = self.environment.stack.len() - 1;
        let end = 0;

        for i in start..=end {
            if identifier == self.environment.stack[i] {
                let global =
                    self.environment.frame_pointer == 0 || i < self.environment.frame_pointer;
                let mut offset = i;

                if !global {
                    offset += self.environment.frame_pointer;
                }

                return Some(Resolution { offset, global });
            }
        }

        return None;
    }
}

impl Visitor<()> for Resolver {
    fn visit_declaration(&mut self, declaration: DeclarationAST) -> Result<(), CompilationError> {
        match declaration {
            DeclarationAST::Variable {
                identifier,
                right,
                span,
                ..
            } => {
                self.visit_expression(*right)?;

                if let Some(_) = self.search_current_scope(&identifier) {
                    return Err(compilation_error!(
                        span,
                        "{} is already declared",
                        identifier
                    ));
                };

                self.environment.declare(identifier);

                Ok(())
            }
        }
    }

    fn visit_statement(&self, statement: StatementAST) -> Result<(), CompilationError> {
        match statement {
            StatementAST::Expression { expression, span } => {
                self.visit_expression(*expression)?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn visit_expression(&self, expression: ExpressionAST) -> Result<(), CompilationError> {
        Ok(())
    }
}
