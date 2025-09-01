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

use super::{environment::Environment, resolution_table::ResolutionTable, symbol::Symbol};

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

                self.resolution_table
                    .create_local_resolution(declaration.id, offset);
            }
            HirDeclKind::Parameter { name, ty } => {
                if self.environment.search_current_scope(name).is_some() {
                    return Err(kaori_error!(
                        declaration.span,
                        "function can't have parameters with the same name: {}",
                        name,
                    ));
                };

                self.environment
                    .declare_local(name.to_owned(), ty.to_owned());
            }
            HirDeclKind::Field { name, ty } => {
                if self.environment.search_current_scope(name).is_some() {
                    return Err(kaori_error!(
                        declaration.span,
                        "struct can't have fields with the same name: {}",
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
            HirExprKind::Add(left, right)
            | HirExprKind::Sub(left, right)
            | HirExprKind::Mul(left, right)
            | HirExprKind::Div(left, right)
            | HirExprKind::Mod(left, right)
            | HirExprKind::Equal(left, right)
            | HirExprKind::NotEqual(left, right)
            | HirExprKind::Less(left, right)
            | HirExprKind::LessEqual(left, right)
            | HirExprKind::Greater(left, right)
            | HirExprKind::GreaterEqual(left, right)
            | HirExprKind::And(left, right)
            | HirExprKind::Or(left, right) => {
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            }
            HirExprKind::Negate(right) | HirExprKind::Not(right) => {
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
                    match symbol {
                        Symbol::Local { offset, ty, .. } => {
                            self.resolution_table
                                .create_local_resolution(expression.id, *offset);

                            self.resolution_table
                                .create_type_resolution(expression.id, ty.to_owned());
                        }
                        Symbol::Global { id, ty, .. } => {
                            self.resolution_table
                                .create_global_resolution(expression.id, *id);

                            self.resolution_table
                                .create_type_resolution(expression.id, ty.to_owned());
                        }
                    }
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

    pub fn resolve_type(&self, ty: &Ty) -> Result<Ty, KaoriError> {
        match &ty.kind {
            TyKind::Boolean => Ty::boolean(ty.span),
            TyKind::Number => Ty::number(ty.span),
            TyKind::Void => Ty::void(ty.span),
            TyKind::String => Ty::string(ty.span),
            TyKind::Function {
                parameters,
                return_ty,
            } => {
                let parameters = parameters
                    .iter()
                    .map(|parameter| self.resolve_type(parameter))
                    .collect::<Result<Vec<Ty>, KaoriError>>()?;

                let return_ty = self.resolve_type(return_ty)?;

                Ty::function(parameters, return_ty)
            }
            TyKind::Struct { fields } => {
                let fields = fields
                    .iter()
                    .map(|field| self.resolve_type(field))
                    .collect::<Result<Vec<Ty>, KaoriError>>()?;

                Ty::struct_(fields, ty.span)
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
    }
}
