use crate::{
    compilation_error,
    compiler::syntax::{
        ast_node::ASTNode, declaration_ast::DeclarationAST, expression_ast::ExpressionAST,
        statement_ast::StatementAST,
    },
    error::compilation_error::CompilationError,
};

use super::{environment::Environment, resolution::Resolution, visitor::Visitor};

pub struct Resolver {
    environment: Environment<String>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    fn search_current_scope(&mut self, identifier: &str) -> Option<Resolution> {
        let mut start = self.environment.declarations.len();
        let end = *self.environment.scopes_pointer.last().unwrap();

        while start > end {
            start -= 1;

            if identifier == self.environment.declarations[start] {
                let global =
                    self.environment.frame_pointer == 0 || start < self.environment.frame_pointer;
                let mut offset = start;

                if !global {
                    offset += self.environment.frame_pointer;
                }

                return Some(Resolution { offset, global });
            }
        }

        return None;
    }

    fn search(&mut self, identifier: &str) -> Option<Resolution> {
        let mut start = self.environment.declarations.len();
        let end: usize = 0;

        while start > end {
            start -= 1;

            if identifier == self.environment.declarations[start] {
                let global =
                    self.environment.frame_pointer == 0 || start < self.environment.frame_pointer;
                let mut offset = start;

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
    fn run(&mut self, ast: &mut Vec<ASTNode>) -> Result<(), CompilationError> {
        self.environment.enter_function();

        for node in ast {
            self.visit_ast_node(node)?;
        }

        self.environment.exit_function();

        Ok(())
    }

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
