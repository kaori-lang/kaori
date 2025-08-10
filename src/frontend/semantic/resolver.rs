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
                    offset = start - self.environment.frame_pointer;
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
                    offset = start - self.environment.frame_pointer;
                }

                return Some(Resolution { offset, global });
            }
        }

        None
    }

    pub fn resolve(&mut self, nodes: &mut [ASTNode]) -> Result<(), KaoriError> {
        self.visit_nodes(nodes)
    }
}

impl Visitor<()> for Resolver {
    fn visit_nodes(&mut self, nodes: &mut [ASTNode]) -> Result<(), KaoriError> {
        for node in nodes.iter().as_slice() {
            if let ASTNode::Declaration(declaration) = node
                && let DeclKind::Function { name, .. } = &declaration.kind
            {
                if self.search_current_scope(name).is_some() {
                    return Err(kaori_error!(
                        declaration.span,
                        "{} is already declared",
                        name
                    ));
                }

                self.environment.declare(name.to_owned());
            }
        }

        for node in nodes {
            self.visit_ast_node(node)?;
        }

        Ok(())
    }

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

                self.environment.declare(name.to_owned());
            }
            DeclKind::Function {
                parameters,
                body,
                name,
                ..
            } => {
                self.environment.enter_scope();

                for parameter in parameters {
                    if self.search_current_scope(&parameter.name).is_some() {
                        return Err(kaori_error!(
                            parameter.span,
                            "function {} can't have parameters with the same name",
                            name,
                        ));
                    };

                    self.environment.declare(parameter.name.to_owned());
                }

                self.visit_nodes(body)?;

                self.environment.exit_scope();
            }
        }

        Ok(())
    }

    fn visit_statement(&mut self, statement: &mut Stmt) -> Result<(), KaoriError> {
        match &mut statement.kind {
            StmtKind::Expression(expression) => self.visit_expression(expression)?,
            StmtKind::Print(expression) => self.visit_expression(expression)?,
            StmtKind::Block(nodes) => {
                self.environment.enter_scope();

                self.visit_nodes(nodes)?;

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
