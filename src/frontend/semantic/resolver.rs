#![allow(clippy::new_without_default)]
use crate::{
    error::kaori_error::KaoriError,
    frontend::{
        hir::{
            hir_ast_node::HirAstNode,
            hir_decl::{HirDecl, HirDeclKind},
            hir_expr::{HirExpr, HirExprKind},
            hir_stmt::{HirStmt, HirStmtKind},
        },
        syntax::ty::{Ty, TyKind},
    },
    kaori_error,
};

use super::{environment::Environment, resolution_table::ResolutionTable};

pub struct Resolver<'a> {
    environment: Environment,
    active_loops: u8,
    resolution_table: &'a mut ResolutionTable,
}

impl<'a> Resolver<'a> {
    pub fn new(resolution_table: &'a mut ResolutionTable) -> Self {
        Self {
            environment: Environment::default(),
            active_loops: 0,
            resolution_table,
        }
    }

    pub fn resolve(&mut self, declarations: &[HirDecl]) -> Result<(), KaoriError> {
        for declaration in declarations.iter() {
            match &declaration.kind {
                HirDeclKind::Function { name, .. } => {
                    if self.environment.search_current_scope(name).is_some() {
                        return Err(kaori_error!(
                            declaration.span,
                            "{} is already declared",
                            name
                        ));
                    }

                    self.environment
                        .declare_function(declaration.id, name.to_owned());
                }
                HirDeclKind::Struct { name, .. } => {
                    if self.environment.search_current_scope(name).is_some() {
                        return Err(kaori_error!(
                            declaration.span,
                            "{} is already declared",
                            name
                        ));
                    }

                    self.environment
                        .declare_struct(declaration.id, name.to_owned());
                }
                _ => (),
            }
        }

        for declaration in declarations {
            self.resolve_declaration(declaration)?;
        }

        Ok(())
    }

    /*  fn resolve_main_function(&mut self, declarations: &[Decl]) -> Result<(), KaoriError> {
           for (index, declaration) in declarations.iter().enumerate() {
               if let HirDeclKind::Function { name, .. } = &declaration.kind
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
    */
    fn resolve_nodes(&mut self, nodes: &[HirAstNode]) -> Result<(), KaoriError> {
        for node in nodes {
            self.resolve_ast_node(node)?;
        }

        Ok(())
    }

    fn resolve_ast_node(&mut self, node: &HirAstNode) -> Result<(), KaoriError> {
        match node {
            HirAstNode::Declaration(declaration) => self.resolve_declaration(declaration),
            HirAstNode::Statement(statement) => self.resolve_statement(statement),
        }?;

        Ok(())
    }

    fn resolve_declaration(&mut self, declaration: &HirDecl) -> Result<(), KaoriError> {
        match &declaration.kind {
            HirDeclKind::Variable { name, right, ty } => {
                self.resolve_expression(right)?;

                if self.environment.search_current_scope(name).is_some() {
                    return Err(kaori_error!(
                        declaration.span,
                        "{} is already declared",
                        name
                    ));
                };

                let offset = self
                    .environment
                    .declare_variable(declaration.id, name.to_owned());

                self.resolution_table
                    .insert_variable_offset(declaration.id, offset);

                self.resolve_type(ty)?;
            }
            HirDeclKind::Parameter { name, ty } => {
                if self.environment.search_current_scope(name).is_some() {
                    return Err(kaori_error!(
                        declaration.span,
                        "function can't have parameters with the same name: {}",
                        name,
                    ));
                };

                let offset = self
                    .environment
                    .declare_variable(declaration.id, name.to_owned());

                self.resolution_table
                    .insert_variable_offset(declaration.id, offset);

                self.resolve_type(ty)?;
            }
            HirDeclKind::Field { name, ty } => {
                if self.environment.search_current_scope(name).is_some() {
                    return Err(kaori_error!(
                        declaration.span,
                        "struct can't have fields with the same name: {}",
                        name,
                    ));
                };

                let offset = self
                    .environment
                    .declare_variable(declaration.id, name.to_owned());

                self.resolution_table
                    .insert_variable_offset(declaration.id, offset);

                self.resolve_type(ty)?;
            }
            HirDeclKind::Function {
                parameters,
                body,
                return_ty,
                ..
            } => {
                self.environment.enter_scope();

                for parameter in parameters {
                    self.resolve_declaration(parameter)?;
                }

                self.resolve_nodes(body)?;

                self.environment.exit_scope();

                if let Some(ty) = return_ty {
                    self.resolve_type(ty)?;
                }
            }
            HirDeclKind::Struct { fields, .. } => {
                self.environment.enter_scope();

                for field in fields {
                    self.resolve_declaration(field)?;
                }

                self.environment.exit_scope();
            }
        };

        Ok(())
    }

    fn resolve_statement(&mut self, statement: &HirStmt) -> Result<(), KaoriError> {
        match &statement.kind {
            HirStmtKind::Expression(expression) => self.resolve_expression(expression)?,
            HirStmtKind::Print(expression) => self.resolve_expression(expression)?,
            HirStmtKind::Block(nodes) => {
                self.environment.enter_scope();
                self.resolve_nodes(nodes)?;
                self.environment.exit_scope();
            }
            HirStmtKind::Branch {
                condition,
                then_branch,
                else_branch,
            } => {
                self.resolve_expression(condition)?;
                self.resolve_statement(then_branch)?;

                if let Some(branch) = else_branch {
                    self.resolve_statement(branch)?;
                }
            }
            HirStmtKind::WhileLoop { condition, block } => {
                self.resolve_expression(condition)?;

                self.active_loops += 1;
                self.resolve_statement(block)?;
                self.active_loops -= 1;
            }
            HirStmtKind::Break => {
                if self.active_loops == 0 {
                    return Err(kaori_error!(
                        statement.span,
                        "break statement can't appear outside of loops"
                    ));
                }
            }
            HirStmtKind::Continue => {
                if self.active_loops == 0 {
                    return Err(kaori_error!(
                        statement.span,
                        "continue statement can't appear outside of loops"
                    ));
                }
            }
            HirStmtKind::Return(expr) => {
                if let Some(expr) = expr {
                    self.resolve_expression(expr)?;
                }
            }
        };

        Ok(())
    }

    fn resolve_expression(&mut self, expression: &HirExpr) -> Result<(), KaoriError> {
        match &expression.kind {
            HirExprKind::Assign(left, right) => {
                self.resolve_expression(right)?;
                self.resolve_expression(left)?;
            }
            HirExprKind::Binary { left, right, .. } => {
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            }
            HirExprKind::Unary { right, .. } => {
                self.resolve_expression(right)?;
            }
            HirExprKind::FunctionCall { callee, arguments } => {
                self.resolve_expression(callee)?;

                for argument in arguments {
                    self.resolve_expression(argument)?;
                }
            }
            HirExprKind::Identifier(name) => {
                if let Some(symbol) = self.environment.search(name) {
                    self.resolution_table
                        .insert_name_resolution(expression.id, symbol.as_resolution());
                } else {
                    return Err(kaori_error!(expression.span, "{} is not declared", name));
                }
            }
            HirExprKind::StringLiteral(..) => {}
            HirExprKind::BooleanLiteral(..) => {}
            HirExprKind::NumberLiteral(..) => {}
        };

        Ok(())
    }

    pub fn resolve_type(&mut self, ty: &Ty) -> Result<(), KaoriError> {
        match &ty.kind {
            TyKind::Function {
                parameters,
                return_ty,
            } => {
                for parameter in parameters {
                    self.resolve_type(parameter)?;
                }

                if let Some(ty) = return_ty {
                    self.resolve_type(ty)?;
                }
            }
            TyKind::Identifier(name) => {
                let Some(symbol) = self.environment.search(name) else {
                    return Err(kaori_error!(ty.span, "{} type is not declared", name));
                };

                self.resolution_table
                    .insert_name_resolution(ty.id, symbol.as_resolution());
            }
        };

        Ok(())
    }
}
