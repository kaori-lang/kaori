use crate::compiler::syntax::{
    ast_node::ASTNode, declaration_ast::DeclarationAST, expression_ast::ExpressionAST,
    statement_ast::StatementAST, type_ast::TypeAST,
};

use super::{environment::Environment, visitor::Visitor};

pub struct Resolver {
    declarations: Vec<ASTNode>,
    environment: Environment<String>,
}

pub struct Resolution {
    offset: usize,
    local: bool,
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
        let end = self.environment.scopes_start.last().unwrap();

        for i in start..=*end {
            if identifier == self.environment.stack[i] {
                let offset = return Some(i);
            }
        }

        return None;
    }

    fn search(&mut self, identifier: String) -> Option<usize> {
        let start = self.environment.stack.len() - 1;
        let end = 0;

        for i in start..=end {
            if identifier == self.environment.stack[i] {
                return Some(i);
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

    fn visit_expression(&self, expression: ExpressionAST) {
        match expression {
            ExpressionAST::Identifier(identifier) => {}
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
