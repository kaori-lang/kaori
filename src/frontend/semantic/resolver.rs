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

use super::{environment::Environment, symbol::Symbol, table::Table};

pub struct Resolver<'a> {
    environment: Environment,
    active_loops: u8,
    table: &'a mut Table,
}

impl<'a> Resolver<'a> {
    pub fn new(table: &'a mut Table) -> Self {
        Self {
            environment: Environment::default(),
            active_loops: 0,
            table,
        }
    }

    pub fn resolve(&mut self, declarations: &[HirDecl]) -> Result<(), KaoriError> {
        for declaration in declarations.iter() {
            match &declaration.kind {
                HirDeclKind::Function { name, ty, .. } => {
                    if self.environment.search_current_scope(name).is_some() {
                        return Err(kaori_error!(
                            declaration.span,
                            "{} is already declared",
                            name
                        ));
                    }

                    self.environment
                        .declare_global(declaration.id, name.to_owned(), ty.to_owned());
                }
                HirDeclKind::Struct { name, ty, .. } => {
                    if self.environment.search_current_scope(name).is_some() {
                        return Err(kaori_error!(
                            declaration.span,
                            "{} is already declared",
                            name
                        ));
                    }
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
                    .declare_local(name.to_owned(), ty.to_owned());

                self.table.create_offset(declaration.id, offset);
            }
            HirDeclKind::Parameter { name, ty } => {
                if self.environment.search_current_scope(&name).is_some() {
                    return Err(kaori_error!(
                        declaration.span,
                        "function {} can't have parameters with the same name",
                        name,
                    ));
                };

                self.environment
                    .declare_local(name.to_owned(), ty.to_owned());
            }
            HirDeclKind::Field { name, ty } => {
                if self.environment.search_current_scope(&name).is_some() {
                    return Err(kaori_error!(
                        declaration.span,
                        "struct {} can't have fields with the same name",
                        name,
                    ));
                };

                self.environment
                    .declare_local(name.to_owned(), ty.to_owned());
            }
            HirDeclKind::Function {
                parameters, body, ..
            } => {
                self.environment.enter_scope();

                for parameter in parameters {
                    self.resolve_declaration(parameter)?;
                }

                self.resolve_nodes(body)?;

                self.environment.exit_scope();
            }
            HirDeclKind::Struct { name, fields, ty } => todo!(),
        };

        Ok(())
    }

    fn resolve_statement(&mut self, statement: &HirStmt) -> Result<(), KaoriError> {
        match &statement.kind {
            HirStmtKind::Expression(expression) => self.resolve_expression(expression)?,
            HirStmtKind::Print(expression) => self.resolve_expression(expression)?,
            HirStmtKind::Block(nodes) => {
                self.environment.enter_scope();
                self.resolve_nodes(nodes);
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

    fn resolve_expression(&self, expression: &HirExpr) -> Result<(), KaoriError> {
        match &expression.kind {
            HirExprKind::Assign(left, right) => {
                self.resolve_expression(right)?;
                self.resolve_expression(left)?;
            }
            HirExprKind::Add(left, right) => {
                self.resolve_expression(left);
                self.resolve_expression(right);
            }
            HirExprKind::Sub(left, right) => {
                self.resolve_expression(left);
                self.resolve_expression(right);
            }
            HirExprKind::Mul(left, right) => {
                self.resolve_expression(left);
                self.resolve_expression(right);
            }
            HirExprKind::Div(left, right) => {
                self.resolve_expression(left);
                self.resolve_expression(right);
            }
            HirExprKind::Mod(left, right) => {
                self.resolve_expression(left);
                self.resolve_expression(right);
            }
            HirExprKind::Equal(left, right) => {
                self.resolve_expression(left);
                self.resolve_expression(right);
            }
            HirExprKind::NotEqual(left, right) => {
                self.resolve_expression(left);
                self.resolve_expression(right);
            }
            HirExprKind::Less(left, right) => {
                self.resolve_expression(left);
                self.resolve_expression(right);
            }
            HirExprKind::LessEqual(left, right) => {
                self.resolve_expression(left);
                self.resolve_expression(right);
            }
            HirExprKind::Greater(left, right) => {
                self.resolve_expression(left);
                self.resolve_expression(right);
            }
            HirExprKind::GreaterEqual(left, right) => {
                self.resolve_expression(left);
                self.resolve_expression(right);
            }
            HirExprKind::And(left, right) => {
                self.resolve_expression(left);
                self.resolve_expression(right);
            }
            HirExprKind::Or(left, right) => {
                self.resolve_expression(left);
                self.resolve_expression(right);
            }
            HirExprKind::Negate(right) => {
                self.resolve_expression(right);
            }
            HirExprKind::Not(right) => {
                self.resolve_expression(right);
            }

            HirExprKind::FunctionCall { callee, arguments } => {
                self.resolve_expression(callee)?;

                for argument in arguments {
                    self.resolve_expression(argument)?;
                }
            }
            HirExprKind::Identifier(name) => match self.environment.search(name) {
                Some(Symbol::Local { offset, ty, .. }) => {
                    self.table.create_local_resolution(expression.id, *offset);
                }
                Some(Symbol::Global { id, ty, .. }) => {
                    self.table.create_global_resolution(expression.id, id);
                }
                _ => return Err(kaori_error!(expression.span, "{} is not declared", name)),
            },
            _ => (),
        };

        Ok(())
    }

    /*     pub fn resolve_type(&self, ty: &Ty) -> Result<(), KaoriError> {
        match &ty.kind {
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
                let Some(Symbol::Global { ty, .. }) = self.environment.search(name) else {
                    return Err(kaori_error!(
                        ty.span,
                        "expected a valid type, but found {}",
                        name
                    ));
                };

                ty.to_owned()
            }
        };
    } */
}
