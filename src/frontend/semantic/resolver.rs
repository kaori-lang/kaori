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
    resolved_expr::ResolvedExpr, resolved_stmt::ResolvedStmt, symbol::Symbol,
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

    pub fn resolve(&mut self, declarations: &[Decl]) -> Result<Vec<ResolvedDecl>, KaoriError> {
        for declaration in declarations.iter().as_slice() {
            if let DeclKind::Function { name, ty, .. } = &declaration.kind {
                if self.environment.search_current_scope(name).is_some() {
                    return Err(kaori_error!(
                        declaration.span,
                        "{} is already declared",
                        name
                    ));
                }

                self.environment
                    .declare_function(name.to_owned(), ty.to_owned());
            }
        }

        let resolved_declarations = declarations
            .iter()
            .map(|declaration| self.resolve_declaration(declaration))
            .collect::<Result<Vec<ResolvedDecl>, KaoriError>>()?;

        Ok(resolved_declarations)
    }

    fn resolve_nodes(&mut self, nodes: &[AstNode]) -> Result<Vec<ResolvedAstNode>, KaoriError> {
        let nodes = nodes
            .iter()
            .map(|node| self.resolve_ast_node(node))
            .collect::<Result<Vec<ResolvedAstNode>, KaoriError>>()?;

        Ok(nodes)
    }

    fn resolve_ast_node(&mut self, node: &AstNode) -> Result<ResolvedAstNode, KaoriError> {
        let resolved_node = match node {
            AstNode::Declaration(declaration) => {
                let declaration = self.resolve_declaration(declaration)?;

                ResolvedAstNode::Declaration(declaration)
            }
            AstNode::Statement(statement) => {
                let statement = self.resolve_statement(statement)?;

                ResolvedAstNode::Statement(statement)
            }
        };

        Ok(resolved_node)
    }

    fn resolve_declaration(&mut self, declaration: &Decl) -> Result<ResolvedDecl, KaoriError> {
        let resolved_decl = match &declaration.kind {
            DeclKind::Variable { name, right, ty } => {
                let right = self.resolve_expression(right)?;

                if self.environment.search_current_scope(name).is_some() {
                    return Err(kaori_error!(
                        declaration.span,
                        "{} is already declared",
                        name
                    ));
                };

                self.environment
                    .declare_variable(name.to_owned(), ty.to_owned());

                ResolvedDecl::variable(right, ty.to_owned(), declaration.span)
            }
            DeclKind::Function {
                parameters,
                body,
                name,
                ty,
            } => {
                self.environment.enter_scope();

                for parameter in parameters {
                    if self
                        .environment
                        .search_current_scope(&parameter.name)
                        .is_some()
                    {
                        return Err(kaori_error!(
                            parameter.span,
                            "function {} can't have parameters with the same name",
                            name,
                        ));
                    };

                    self.environment
                        .declare_variable(parameter.name.to_owned(), parameter.ty.to_owned());
                }

                let body = self.resolve_nodes(body)?;

                self.environment.exit_scope();

                let id = if let Some(Symbol::Function { id, .. }) = self.environment.search(name) {
                    *id
                } else {
                    0
                };

                ResolvedDecl::function(id, parameters, body, ty.to_owned(), declaration.span)
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
            StmtKind::WhileLoop { condition, block } => {
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
            ExprKind::Unary { right, operator } => {
                let right = self.resolve_expression(right)?;

                ResolvedExpr::unary(operator.to_owned(), right, expression.span)
            }
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
            ExprKind::Identifier { name } => match self.environment.search(name) {
                Some(Symbol::Variable { offset, ty, .. }) => {
                    ResolvedExpr::variable_ref(*offset, ty.to_owned(), expression.span)
                }
                Some(Symbol::Function { id, ty, .. }) => {
                    ResolvedExpr::function_ref(*id, ty.to_owned(), expression.span)
                }
                _ => return Err(kaori_error!(expression.span, "{} is not declared", name)),
            },
        };

        Ok(resolved_expr)
    }
}
