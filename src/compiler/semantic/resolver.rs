use crate::{compiler::syntax::{
    ast_node::ASTNode, declaration_ast::DeclarationAST, expression_ast::ExpressionAST,
    statement_ast::StatementAST, type_ast::TypeAST,
}, error::compilation_error::CompilationError};

use super::{environment::Environment, visitor::Visitor};

pub struct Resolver {
    declarations: Vec<ASTNode>,
    environment: Environment<String>,
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
        }
    }

    fn search_current_scope(&mut self, identifier: String) -> Option<Resolution> {
        let start = self.environment.stack.len() - 1;
        let end = self.environment.scopes_pointer.last().unwrap();

        for i in start..=*end {
            if identifier == self.environment.stack[i] {
                let global =
                    self.environment.frame_pointer == 0 || i < self.environment.frame_pointer;
                let mut offset = i;

                if (!global) {
                    offset += self.environment.frame_pointer;
                }

                return Some(Resolution { offset, global });
            }
        }

        return None;
    }

    fn search(&mut self, identifier: String) -> Option<Resolution> {
        let start = self.environment.stack.len() - 1;
        let end = 0;

        for i in start..=end {
            if identifier == self.environment.stack[i] {
                let global =
                    self.environment.frame_pointer == 0 || i < self.environment.frame_pointer;
                let mut offset = i;

                if (!global) {
                    offset += self.environment.frame_pointer;
                }

                return Some(Resolution { offset, global });
            }
        }

        return None;
    }
}

impl Visitor<()> for Resolver {
    fn visit_declaration(&self, declaration: DeclarationAST) {
        match declaration {
            DeclarationAST::Variable {
                identifier,
                right,
                ty,
                span,
                offset,
            } => (),
        }
    }

    fn visit_statement(&self, statement: StatementAST) {
        match statement {
            StatementAST::Expression { expression, span } => {
                self.visit_expression(*expression);
                ()
            }
            _ => (),
        }
    }

    fn visit_expression(&self, expression: ExpressionAST) -> Result<(), CompilationError> {
        match expression {
            ExpressionAST::Identifier(identifier, span) => {
                let Some(value) = self.search_current_scope(identifier) else {
                    return Err()
                }

                

            }
            ExpressionAST::Assign { identifier, right } => TypeAST::String,
            ExpressionAST::Binary {
                operator,
                left,
                right,
            } => {
                let left = self.visit_expression(*left);
                let right = self.visit_expression(*right);

                ()
            }
            _ => (),
        }
    }
}
