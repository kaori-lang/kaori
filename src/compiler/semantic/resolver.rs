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
    fn visit_ast_node(&mut self, ast_node: ASTNode) -> Result<(), CompilationError> {
        match ast_node {
            ASTNode::Declaration(declaration) => self.visit_declaration(declaration),
            ASTNode::Statement(statement) => self.visit_statement(statement),
        }
    }

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

    fn visit_statement(&mut self, statement: StatementAST) -> Result<(), CompilationError> {
        match statement {
            StatementAST::Expression(expression) => {
                self.visit_expression(*expression)?;
                Ok(())
            }
            StatementAST::Print { expression, .. } => {
                self.visit_expression(*expression)?;
                Ok(())
            }
            StatementAST::Block { declarations, .. } => {
                self.environment.enter_scope();

                for declaration in declarations {
                    self.visit_ast_node(declaration)?;
                }

                self.environment.exit_scope();

                Ok(())
            }
            StatementAST::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                self.visit_expression(*condition)?;
                self.visit_statement(*then_branch)?;

                if let Some(branch) = else_branch {
                    self.visit_statement(*branch)?;
                }

                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn visit_expression(&mut self, expression: ExpressionAST) -> Result<(), CompilationError> {
        match expression {
            ExpressionAST::Assign {
                identifier,
                right,
                span,
            } => {
                self.visit_expression(*right)?;

                let Some(_) = self.search(&identifier) else {
                    return Err(compilation_error!(span, "{} is not declared", identifier));
                };

                Ok(())
            }
            ExpressionAST::Binary { left, right, .. } => {
                self.visit_expression(*left)?;
                self.visit_expression(*right)?;

                Ok(())
            }
            ExpressionAST::Unary { right, .. } => {
                self.visit_expression(*right)?;

                Ok(())
            }
            ExpressionAST::Identifier(identifier, span) => {
                let Some(_) = self.search(&identifier) else {
                    return Err(compilation_error!(span, "{} is not declared", identifier));
                };

                Ok(())
            }
            _ => Ok(()),
        }
    }
}
