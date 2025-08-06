#![allow(clippy::new_without_default)]
use crate::{
    error::kaori_error::KaoriError,
    frontend::syntax::{
        ast_node::ASTNode,
        declaration::{Decl, DeclKind},
        expression::{Expr, ExprKind},
        statement::{Stmt, StmtKind},
    },
    kaori_error,
    utils::visitor::Visitor,
};

use super::{environment::Environment, resolution::Resolution};

pub struct Resolver {
    environment: Environment<String>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            environment: Environment::default(),
        }
    }

    fn search_current_scope(&mut self, name: &str) -> Option<Resolution> {
        let mut start = self.environment.declarations.len();
        let end = *self.environment.scopes_pointer.last().unwrap();

        while start > end {
            start -= 1;

            if name == self.environment.declarations[start] {
                let global =
                    self.environment.frame_pointer == 0 || start < self.environment.frame_pointer;
                let mut offset = start;

                if !global {
                    offset += self.environment.frame_pointer;
                }

                return Some(Resolution { offset, global });
            }
        }

        None
    }

    fn search(&mut self, name: &str) -> Option<Resolution> {
        let mut start = self.environment.declarations.len();
        let end: usize = 0;

        while start > end {
            start -= 1;

            if name == self.environment.declarations[start] {
                let global =
                    self.environment.frame_pointer == 0 || start < self.environment.frame_pointer;
                let mut offset = start;

                if !global {
                    offset += self.environment.frame_pointer;
                }

                return Some(Resolution { offset, global });
            }
        }

        None
    }

    pub fn resolve(&mut self, nodes: &mut [ASTNode]) -> Result<(), KaoriError> {
        self.environment.enter_function();

        for i in 0..nodes.len() {
            if let Some(ASTNode::Declaration(decl)) = nodes.get(i)
                && let DeclKind::Function { name, .. } = &decl.kind
            {
                if self.search(name).is_some() {
                    return Err(kaori_error!(decl.span, "{} is already declared", name));
                }

                self.environment.declare(name.clone());
            }
        }

        for i in 0..nodes.len() {
            if let Some(node) = nodes.get_mut(i) {
                self.visit_ast_node(node)?;
            }
        }

        self.environment.exit_function();

        Ok(())
    }
}

impl Visitor<()> for Resolver {
    fn visit_ast_node(&mut self, node: &mut ASTNode) -> Result<(), KaoriError> {
        match node {
            ASTNode::Declaration(declaration) => self.visit_declaration(declaration),
            ASTNode::Statement(statement) => self.visit_statement(statement),
        }
    }

    fn visit_declaration(&mut self, declaration: &mut Decl) -> Result<(), KaoriError> {
        match &mut declaration.kind {
            DeclKind::Variable { name, right, .. } => {
                self.visit_expression(right)?;

                if self.search_current_scope(name).is_some() {
                    return Err(kaori_error!(
                        declaration.span,
                        "{} is already declared",
                        name
                    ));
                };

                self.environment.declare(name.clone());
            }
            DeclKind::Function {
                parameters, block, ..
            } => {
                self.environment.enter_function();

                for parameter in parameters {
                    self.visit_declaration(parameter)?;
                }

                self.visit_statement(block)?;

                self.environment.exit_function();
            }
        }

        Ok(())
    }

    fn visit_statement(&mut self, statement: &mut Stmt) -> Result<(), KaoriError> {
        match &mut statement.kind {
            StmtKind::Expression(expression) => self.visit_expression(expression)?,
            StmtKind::Print(expression) => self.visit_expression(expression)?,
            StmtKind::Block(declarations) => {
                self.environment.enter_scope();

                for declaration in declarations {
                    self.visit_ast_node(declaration)?;
                }

                self.environment.exit_scope();
            }
            StmtKind::If {
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
            StmtKind::WhileLoop {
                condition, block, ..
            } => {
                self.visit_expression(condition)?;
                self.visit_statement(block)?;
            }
            _ => {}
        };

        Ok(())
    }

    fn visit_expression(&mut self, expression: &mut Expr) -> Result<(), KaoriError> {
        match &mut expression.kind {
            ExprKind::Assign { identifier, right } => {
                self.visit_expression(right)?;
                self.visit_expression(identifier)?;
            }
            ExprKind::Binary { left, right, .. } => {
                self.visit_expression(left)?;
                self.visit_expression(right)?;
            }
            ExprKind::Unary { right, .. } => self.visit_expression(right)?,
            ExprKind::Identifier { name, resolution } => {
                let Some(res) = self.search(name) else {
                    return Err(kaori_error!(expression.span, "{} is not declared", name));
                };

                *resolution = res;
            }
            ExprKind::FunctionCall { callee, arguments } => {
                self.visit_expression(callee)?;

                for argument in arguments {
                    self.visit_expression(argument)?;
                }
            }
            _ => (),
        };

        Ok(())
    }
}
