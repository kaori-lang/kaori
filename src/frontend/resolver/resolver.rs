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
};

use super::{
    environment::Environment, resolved_ast_node::ResolvedAstNode, resolved_decl::ResolvedDecl,
    resolved_expr::ResolvedExpr, resolved_stmt::ResolvedStmt,
};

pub struct Resolver {
    environment: Environment,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            environment: Environment::default(),
        }
    }

    fn search_variable(&mut self, name: &str) -> Option<usize> {
        let mut i = self.environment.runtime_symbols.len();
        let j: usize = 0;

        while i > j {
            i -= 1;

            if name == self.environment.runtime_symbols[i] {
                return Some(i);
            }
        }

        None
    }

    fn is_declared(&self, name: &String) -> bool {
        self.environment.is_function_declared(name) || self.environment.is_variable_declared(name)
    }

    pub fn resolve(&mut self, nodes: &[AstNode]) -> Result<Vec<ResolvedAstNode>, KaoriError> {
        self.resolve_nodes(nodes)
    }

    fn resolve_nodes(&mut self, nodes: &[AstNode]) -> Result<Vec<ResolvedAstNode>, KaoriError> {
        for node in nodes.iter().as_slice() {
            if let AstNode::Declaration(declaration) = node
                && let DeclKind::Function { name, .. } = &declaration.kind
            {
                if self.is_declared(name) {
                    return Err(kaori_error!(
                        declaration.span,
                        "{} is already declared",
                        name
                    ));
                }

                self.environment.declare_function(name.to_owned());
            }
        }

        let mut resolved_ast_nodes = Vec::new();

        for node in nodes {
            let node = self.resolve_ast_node(node)?;

            resolved_ast_nodes.push(node);
        }

        Ok(resolved_ast_nodes)
    }

    fn resolve_ast_node(&mut self, node: &AstNode) -> Result<ResolvedAstNode, KaoriError> {
        let resolved_ast_node = match node {
            AstNode::Declaration(declaration) => {
                let declaration = self.resolve_declaration(declaration)?;

                ResolvedAstNode::Declaration(declaration)
            }
            AstNode::Statement(statement) => {
                let statement = self.resolve_statement(statement)?;

                ResolvedAstNode::Statement(statement)
            }
        };

        Ok(resolved_ast_node)
    }

    fn resolve_declaration(&mut self, declaration: &Decl) -> Result<ResolvedDecl, KaoriError> {
        let resolved_decl = match &declaration.kind {
            DeclKind::Variable {
                name,
                right,
                type_annotation,
            } => {
                let right = self.resolve_expression(right)?;

                if self.is_declared(name) {
                    return Err(kaori_error!(
                        declaration.span,
                        "{} is already declared",
                        name
                    ));
                };

                self.environment.declare_variable(name.to_owned());

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
                    if self.is_declared(&parameter.name) {
                        return Err(kaori_error!(
                            parameter.span,
                            "function {} can't have parameters with the same name",
                            name,
                        ));
                    };

                    self.declare(parameter.name.to_owned());
                }

                let body = self.resolve_nodes(body)?;

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

    fn resolve_statement(&mut self, statement: &Stmt) -> Result<ResolvedStmt, KaoriError> {
        let resolved_stmt = match &statement.kind {
            StmtKind::Expression(expression) => {
                let expr = self.resolve_expression(expression)?;

                ResolvedStmt::expression(expr, statement.span)
            }
            StmtKind::Print(expression) => {
                let expr = self.resolve_expression(expression)?;

                ResolvedStmt::print(expr, statement.span)
            }
            StmtKind::Block(nodes) => {
                let nodes = self.resolve_nodes(nodes)?;

                ResolvedStmt::block(nodes, statement.span)
            }
            StmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition = self.resolve_expression(condition)?;
                let then_branch = self.resolve_statement(then_branch)?;
                let else_branch = if let Some(branch) = else_branch {
                    Some(self.resolve_statement(branch)?)
                } else {
                    None
                };

                ResolvedStmt::if_(condition, then_branch, else_branch, statement.span)
            }
            StmtKind::WhileLoop {
                condition, block, ..
            } => {
                let condition = self.resolve_expression(condition)?;
                let block = self.resolve_statement(block)?;

                ResolvedStmt::while_loop(condition, block, statement.span)
            }
            StmtKind::Break => ResolvedStmt::break_(statement.span),
            StmtKind::Continue => ResolvedStmt::continue_(statement.span),
        };

        Ok(resolved_stmt)
    }

    fn resolve_expression(&self, expression: &Expr) -> Result<ResolvedExpr, KaoriError> {
        let resolved_expr = match &expression.kind {
            ExprKind::Assign { identifier, right } => {
                let right = self.resolve_expression(right)?;
                let identifier = self.resolve_expression(identifier)?;

                ResolvedExpr::assign(identifier, right, expression.span)
            }
            ExprKind::Binary {
                left,
                right,
                operator,
            } => {
                let left = self.resolve_expression(left)?;
                let right = self.resolve_expression(right)?;

                ResolvedExpr::binary(operator.to_owned(), left, right, expression.span)
            }
            ExprKind::Unary { right, .. } => self.resolve_expression(right)?,
            ExprKind::Identifier { name } => ResolvedExpr::variable(0, expression.span),
            ExprKind::FunctionCall { callee, arguments } => {
                let callee = self.resolve_expression(callee)?;
                let mut resolved_args = Vec::new();

                for argument in arguments {
                    let argument = self.resolve_expression(argument)?;
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
