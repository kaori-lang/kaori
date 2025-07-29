use crate::{
    compilation_error,
    compiler::{
        lexer::span::Span,
        syntax::{
            ast_node::ASTNode, declaration_ast::DeclarationAST, expression_ast::ExpressionAST,
            statement_ast::StatementAST,
        },
    },
    error::compilation_error::CompilationError,
};

use super::{
    environment::Environment,
    resolution::{self, Resolution},
    visitor::Visitor,
};

pub struct Resolver {
    declarations: Vec<ASTNode>,
    environment: Environment<String>,
    span: Span,
}

impl Resolver {
    pub fn new(declarations: Vec<ASTNode>) -> Self {
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
    fn visit_ast_node(&mut self, ast_node: &mut ASTNode) -> Result<(), CompilationError> {
        match ast_node {
            ASTNode::Declaration(declaration) => self.visit_declaration(declaration),
            ASTNode::Statement(statement) => self.visit_statement(statement),
        }
    }

    fn visit_declaration(
        &mut self,
        declaration: &mut DeclarationAST,
    ) -> Result<(), CompilationError> {
        match declaration {
            DeclarationAST::Variable {
                identifier,
                right,
                span,
                ..
            } => {
                self.visit_expression(right)?;

                if let Some(_) = self.search_current_scope(identifier) {
                    return Err(compilation_error!(
                        *span,
                        "{} is already declared",
                        identifier
                    ));
                };

                self.environment.declare((*identifier).clone());

                Ok(())
            }
        }
    }

    fn visit_statement(&mut self, statement: &mut StatementAST) -> Result<(), CompilationError> {
        match statement {
            StatementAST::Expression(expression) => self.visit_expression(expression.as_mut())?,

            StatementAST::Print { expression, .. } => self.visit_expression(expression.as_mut())?,
            StatementAST::Block { declarations, .. } => {
                self.environment.enter_scope();

                for declaration in declarations {
                    self.visit_ast_node(declaration)?;
                }

                self.environment.exit_scope();
            }
            StatementAST::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                self.visit_expression(condition)?;
                self.visit_statement(then_branch)?;

                if let Some(branch) = else_branch {
                    self.visit_statement(branch)?;
                }
            }
            StatementAST::WhileLoop {
                condition, block, ..
            } => {
                self.visit_expression(condition)?;
                self.visit_statement(block)?;
            }
            _ => (),
        };

        Ok(())
    }

    fn visit_expression(&mut self, expression: &mut ExpressionAST) -> Result<(), CompilationError> {
        match expression {
            ExpressionAST::Assign {
                identifier,
                right,
                span,
            } => {
                self.visit_expression(right)?;

                let Some(_) = self.search(&identifier) else {
                    return Err(compilation_error!(
                        span.clone(),
                        "{} is not declared",
                        identifier
                    ));
                };
            }
            ExpressionAST::Binary { left, right, .. } => {
                self.visit_expression(left)?;
                self.visit_expression(right)?;
            }
            ExpressionAST::Unary { right, .. } => self.visit_expression(right)?,
            ExpressionAST::Identifier {
                name,
                span,
                resolution,
            } => {
                let Some(res) = self.search(name) else {
                    return Err(compilation_error!(*span, "{} is not declared", name));
                };

                *resolution = Some(res);
            }
            _ => (),
        };

        Ok(())
    }
}
