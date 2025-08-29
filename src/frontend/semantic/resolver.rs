#![allow(clippy::new_without_default)]
use crate::{
    error::kaori_error::KaoriError,
    frontend::{
        scanner::span::Span,
        syntax::{
            ast_node::AstNode,
            decl::{Decl, DeclKind},
            expr::{Expr, ExprKind},
            stmt::{Stmt, StmtKind},
            ty::{Ty, TyKind},
        },
    },
    kaori_error,
};

use super::{
    environment::Environment,
    resolved_ast_node::ResolvedAstNode,
    resolved_decl::{ResolvedDecl, ResolvedParameter},
    resolved_expr::ResolvedExpr,
    resolved_stmt::ResolvedStmt,
    resolved_ty::ResolvedTy,
    symbol::Symbol,
};

pub struct Resolver {
    environment: Environment,
    active_loops: u8,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            environment: Environment::default(),
            active_loops: 0,
        }
    }

    pub fn resolve(&mut self, declarations: &mut [Decl]) -> Result<Vec<ResolvedDecl>, KaoriError> {
        self.resolve_main_function(declarations)?;

        for declaration in declarations.iter() {
            match &declaration.kind {
                DeclKind::Function { id, name, ty, .. } => {
                    if self.environment.search_current_scope(name).is_some() {
                        return Err(kaori_error!(
                            declaration.span,
                            "{} is already declared",
                            name
                        ));
                    }

                    let ty = self.resolve_type(ty)?;

                    self.environment.declare_variable(name.to_owned(), ty);
                }
                DeclKind::Struct { id, name, ty, .. } => {
                    if self.environment.search(name).is_some() {
                        return Err(kaori_error!(
                            declaration.span,
                            "{} is already declared",
                            name
                        ));
                    }

                    let ty = self.resolve_type(ty)?;

                    self.environment.declare_variable(name.to_owned(), ty);
                }
                _ => (),
            }
        }

        let resolved_declarations = declarations
            .iter()
            .map(|declaration| self.resolve_declaration(declaration))
            .collect::<Result<Vec<ResolvedDecl>, KaoriError>>()?;

        Ok(resolved_declarations)
    }

    fn resolve_main_function(&mut self, declarations: &mut [Decl]) -> Result<(), KaoriError> {
        for (index, declaration) in declarations.iter().enumerate() {
            if let DeclKind::Function { name, .. } = &declaration.kind
                && name == "main"
            {
                declarations.swap(0, index);
                return Ok(());
            }
        }

        Err(kaori_error!(
            Span::default(),
            "main function is not declared"
        ))
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

                let ty = self.resolve_type(ty)?;

                let offset = self
                    .environment
                    .declare_variable(name.to_owned(), ty.to_owned());

                ResolvedDecl::variable(offset, right, ty, declaration.span)
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

                    let ty = self.resolve_type(&parameter.ty)?;
                    let name = parameter.name.to_owned();

                    self.environment.declare_global_variable(name, ty);
                }

                let body = self.resolve_nodes(body)?;

                self.environment.exit_scope();

                let mut resolved_parameters = Vec::new();

                for parameter in parameters {
                    let ty = self.resolve_type(&parameter.ty)?;
                    let span = parameter.span;
                    let parameter = ResolvedParameter { ty, span };

                    resolved_parameters.push(parameter);
                }

                let ty = self.resolve_type(ty)?;

                ResolvedDecl::function(resolved_parameters, body, ty, declaration.span)
            }
            DeclKind::Struct {
                id,
                name,
                fields,
                ty,
            } => todo!(),
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
                self.environment.enter_scope();
                let nodes = self.resolve_nodes(nodes)?;

                self.environment.exit_scope();
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

                self.active_loops += 1;
                let block = self.resolve_statement(block)?;
                self.active_loops -= 1;

                ResolvedStmt::while_loop(condition, block, statement.span)
            }
            StmtKind::Break => {
                if self.active_loops == 0 {
                    return Err(kaori_error!(
                        statement.span,
                        "break statement can't appear outside of loops"
                    ));
                }

                ResolvedStmt::break_(statement.span)
            }
            StmtKind::Continue => {
                if self.active_loops == 0 {
                    return Err(kaori_error!(
                        statement.span,
                        "continue statement can't appear outside of loops"
                    ));
                }

                ResolvedStmt::continue_(statement.span)
            }
            StmtKind::Return(expr) => {
                let resolved_expr = match expr {
                    Some(expr) => Some(self.resolve_expression(expr)?),
                    None => None,
                };

                ResolvedStmt::return_(resolved_expr, statement.span)
            }
        };

        Ok(resolved_stmt)
    }

    fn resolve_expression(&self, expression: &Expr) -> Result<ResolvedExpr, KaoriError> {
        let resolved_expr = match &expression.kind {
            ExprKind::Assign { left, right } => {
                let right = self.resolve_expression(right)?;
                let left = self.resolve_expression(left)?;

                ResolvedExpr::assign(left, right, expression.span)
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

                let frame_size = self.environment.offset;

                ResolvedExpr::function_call(callee, resolved_args, frame_size, expression.span)
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
                Some(Symbol { offset, name, ty }) => {
                    ResolvedExpr::variable_ref(*offset, ty.to_owned(), expression.span)
                }
                _ => return Err(kaori_error!(expression.span, "{} is not declared", name)),
            },
        };

        Ok(resolved_expr)
    }

    pub fn resolve_type(&self, ty: &Ty) -> Result<ResolvedTy, KaoriError> {
        let resolved_ty = match &ty.kind {
            TyKind::Boolean => ResolvedTy::boolean(ty.span),
            TyKind::Number => ResolvedTy::number(ty.span),
            TyKind::Void => ResolvedTy::void(ty.span),
            TyKind::String => ResolvedTy::string(ty.span),
            TyKind::Function {
                parameters,
                return_ty,
            } => {
                let parameters = parameters
                    .iter()
                    .map(|parameter| self.resolve_type(parameter))
                    .collect::<Result<Vec<ResolvedTy>, KaoriError>>()?;

                let return_ty = self.resolve_type(return_ty)?;

                ResolvedTy::function(parameters, return_ty, ty.span)
            }
            TyKind::Struct { fields } => {
                let fields = fields
                    .iter()
                    .map(|field| self.resolve_type(field))
                    .collect::<Result<Vec<ResolvedTy>, KaoriError>>()?;

                ResolvedTy::struct_(fields, ty.span)
            }
            TyKind::Custom { name } => {
                todo!()
                /* let Some(Symbol::Struct { ty, .. }) = self.environment.search(name) else {
                    return Err(kaori_error!(
                        ty.span,
                        "expected a valid type, but found {}",
                        name
                    ));
                };

                ty.to_owned() */
            }
        };

        Ok(resolved_ty)
    }
}
