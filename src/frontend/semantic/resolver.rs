#![allow(clippy::new_without_default)]
use crate::{
    error::kaori_error::KaoriError,
    frontend::syntax::{
        ast_node::AstNode,
        declaration::{Decl, DeclKind},
        expression::{Expr, ExprKind},
        statement::{Stmt, StmtKind},
    },
    kaori_error,
    utils::visitor::{AstNodeVisitor, DeclVisitor, ExprVisitor, StmtVisitor},
};

use super::{
    environment::Environment,
    resolved_ast_node::ResolvedAstNode,
    resolved_decl::{ResolvedDecl, ResolvedParameter},
    resolved_expr::ResolvedExpr,
    resolved_stmt::ResolvedStmt,
};

pub struct Resolver {
    environment: Environment<String>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            environment: Environment::default(),
        }
    }

    fn search_current_scope(&mut self, name: &str) {
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
            }
        }
    }

    fn search(&mut self, name: &str) {
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
            }
        }
    }

    pub fn resolve(&self, nodes: &[AstNode]) -> Result<Vec<ResolvedAstNode>, KaoriError> {
        self.visit_nodes(nodes)
    }
}

impl AstNodeVisitor<ResolvedAstNode> for Resolver {
    fn visit_nodes(&self, nodes: &[AstNode]) -> Result<Vec<ResolvedAstNode>, KaoriError> {
        for node in nodes.iter().as_slice() {
            if let AstNode::Declaration(declaration) = node
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

        let resolved_ast_nodes = Vec::new();

        for node in nodes {
            let node = self.visit_ast_node(node)?;

            resolved_ast_nodes.push(node);
        }

        Ok(resolved_ast_nodes)
    }

    fn visit_ast_node(&self, node: &AstNode) -> Result<ResolvedAstNode, KaoriError> {
        let resolved_ast_node = match node {
            AstNode::Declaration(declaration) => {
                let declaration = self.visit_declaration(declaration)?;

                ResolvedAstNode::Declaration(declaration)
            }
            AstNode::Statement(statement) => {
                let statement = self.visit_statement(statement)?;

                ResolvedAstNode::Statement(statement)
            }
        };

        Ok(resolved_ast_node)
    }
}

impl DeclVisitor<ResolvedDecl> for Resolver {
    fn visit_declaration(&self, declaration: &Decl) -> Result<ResolvedDecl, KaoriError> {
        let resolved_decl = match &declaration.kind {
            DeclKind::Variable {
                name,
                right,
                type_annotation,
            } => {
                let right = self.visit_expression(right)?;

                if self.search_current_scope(name).is_some() {
                    return Err(kaori_error!(
                        declaration.span,
                        "{} is already declared",
                        name
                    ));
                };

                self.environment.declare(name.to_owned());

                ResolvedDecl::variable(right, type_annotation.to_owned(), declaration.span)
            }
            DeclKind::Function {
                parameters,
                body,
                name,
                type_annotation,
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

                let body = self.visit_nodes(body)?;

                self.environment.exit_scope();

                ResolvedDecl::function(
                    parameters,
                    body,
                    type_annotation.to_owned(),
                    declaration.span,
                )
            }
        };

        Ok(resolved_decl)
    }
}

impl StmtVisitor<ResolvedStmt> for Resolver {
    fn visit_statement(&self, statement: &Stmt) -> Result<ResolvedStmt, KaoriError> {
        let resolved_stmt = match &statement.kind {
            StmtKind::Expression(expression) => {
                let expr = self.visit_expression(expression)?;

                ResolvedStmt::expression(expr, statement.span)
            }
            StmtKind::Print(expression) => {
                let expr = self.visit_expression(expression)?;

                ResolvedStmt::print(expr, statement.span)
            }
            StmtKind::Block(nodes) => {
                let nodes = self.visit_nodes(nodes)?;

                ResolvedStmt::block(nodes, statement.span)
            }
            StmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition = self.visit_expression(condition)?;
                let then_branch = self.visit_statement(then_branch)?;
                let else_branch = if let Some(branch) = else_branch {
                    Some(self.visit_statement(branch)?)
                } else {
                    None
                };

                ResolvedStmt::if_(condition, then_branch, else_branch, statement.span)
            }
            StmtKind::WhileLoop {
                condition, block, ..
            } => {
                let condition = self.visit_expression(condition)?;
                let block = self.visit_statement(block)?;

                ResolvedStmt::while_loop(condition, block, statement.span)
            }
            StmtKind::Break => ResolvedStmt::break_(statement.span),
            StmtKind::Continue => ResolvedStmt::continue_(statement.span),
        };

        Ok(resolved_stmt)
    }
}

impl ExprVisitor<ResolvedExpr> for Resolver {
    fn visit_expression(&self, expression: &Expr) -> Result<ResolvedExpr, KaoriError> {
        let resolved_expr = match &expression.kind {
            ExprKind::Assign { identifier, right } => {
                let right = self.visit_expression(right)?;
                let identifier = self.visit_expression(identifier)?;

                ResolvedExpr::assign(identifier, right, expression.span)
            }
            ExprKind::Binary {
                left,
                right,
                operator,
            } => {
                let left = self.visit_expression(left)?;
                let right = self.visit_expression(right)?;

                ResolvedExpr::binary(operator.to_owned(), left, right, expression.span)
            }
            ExprKind::Unary { right, .. } => self.visit_expression(right)?,
            ExprKind::Identifier { name } => ResolvedExpr::variable(0, expression.span),
            ExprKind::FunctionCall { callee, arguments } => {
                let callee = self.visit_expression(callee)?;
                let mut resolved_args = Vec::new();

                for argument in arguments {
                    let argument = self.visit_expression(argument)?;
                    resolved_args.push(argument);
                }

                ResolvedExpr::function_call(callee, resolved_args, expression.span)
            }
            ExprKind::NumberLiteral(value) => {
                ResolvedExpr::number_literal(value.to_owned(), expression.span)
            }
            ExprKind::BooleanLiteral(value) => {
                ResolvedExpr::boolean_literal(value.to_owned(), expression.span)
            }
            ExprKind::StringLiteral(value) => {
                ResolvedExpr::string_literal(value.to_owned(), expression.span)
            }
        };

        Ok(resolved_expr)
    }
}
