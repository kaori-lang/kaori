#![allow(clippy::new_without_default)]
#![allow(clippy::only_used_in_recursion)]

use crate::{
    error::kaori_error::KaoriError,
    frontend::hir::{
        hir_ast_node::HirAstNode,
        hir_decl::{HirDecl, HirDeclKind},
        hir_expr::{HirExpr, HirExprKind},
        hir_stmt::{HirStmt, HirStmtKind},
        hir_ty::{HirTy, HirTyKind},
    },
};

use super::{
    checked_ty::CheckedTy,
    resolution_table::{Resolution, ResolutionTable},
};

pub struct TypeChecker<'a> {
    function_return_ty: Option<HirTy>,
    resolution_table: &'a mut ResolutionTable,
}

impl<'a> TypeChecker<'a> {
    pub fn new(resolution_table: &'a mut ResolutionTable) -> Self {
        Self {
            function_return_ty: None,
            resolution_table,
        }
    }

    pub fn check(&mut self, declarations: &[HirDecl]) -> Result<(), KaoriError> {
        /*   for declaration in declarations.iter() {
            match &declaration.kind {
                HirDeclKind::Function {
                    parameters,
                    return_ty,
                    ..
                } => {}
                HirDeclKind::Struct { name, .. } => {}
                _ => (),
            }
        } */

        for declaration in declarations {
            self.check_declaration(declaration)?;
        }

        Ok(())
    }

    /*  fn check_main_function(&mut self, declarations: &[Decl]) -> Result<(), KaoriError> {
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
    fn check_nodes(&mut self, nodes: &[HirAstNode]) -> Result<(), KaoriError> {
        for node in nodes {
            self.check_ast_node(node)?;
        }

        Ok(())
    }

    fn check_ast_node(&mut self, node: &HirAstNode) -> Result<(), KaoriError> {
        match node {
            HirAstNode::Declaration(declaration) => self.check_declaration(declaration),
            HirAstNode::Statement(statement) => self.check_statement(statement),
        }?;

        Ok(())
    }

    fn check_declaration(&mut self, declaration: &HirDecl) -> Result<(), KaoriError> {
        match &declaration.kind {
            HirDeclKind::Variable { name, right, ty } => {}
            HirDeclKind::Parameter { name, ty } => {}
            HirDeclKind::Field { name, ty } => {}
            HirDeclKind::Function {
                parameters,
                body,
                return_ty,
                ..
            } => {}
            HirDeclKind::Struct { name, fields } => {}
        };

        Ok(())
    }

    fn check_statement(&mut self, statement: &HirStmt) -> Result<(), KaoriError> {
        match &statement.kind {
            HirStmtKind::Expression(expression) => {
                self.check_expression(expression)?;
            }
            HirStmtKind::Print(expression) => {
                self.check_expression(expression)?;
            }
            HirStmtKind::Block(nodes) => {
                self.check_nodes(nodes)?;
            }
            HirStmtKind::Branch {
                condition,
                then_branch,
                else_branch,
            } => {
                self.check_expression(condition)?;
                self.check_statement(then_branch)?;

                if let Some(branch) = else_branch {
                    self.check_statement(branch)?;
                }
            }
            HirStmtKind::WhileLoop { condition, block } => {
                self.check_expression(condition)?;

                self.check_statement(block)?;
            }
            HirStmtKind::Break => {}
            HirStmtKind::Continue => {}
            HirStmtKind::Return(expr) => {
                if let Some(expr) = expr {
                    self.check_expression(expr)?;
                }
            }
        };

        Ok(())
    }

    fn check_expression(&mut self, expression: &HirExpr) -> Result<CheckedTy, KaoriError> {
        let ty = match &expression.kind {
            HirExprKind::Assign(left, right) => {
                let right = self.check_expression(right)?;
                let left = self.check_expression(left)?;

                left
            }
            HirExprKind::Binary {
                operator,
                left,
                right,
            } => {
                let left = self.check_expression(left)?;
                let right = self.check_expression(right)?;

                left
            }
            HirExprKind::Unary { right, operator } => {
                let right = self.check_expression(right)?;

                right
            }
            HirExprKind::FunctionCall { callee, arguments } => {
                let callee = self.check_expression(callee)?;

                for argument in arguments {
                    self.check_expression(argument)?;
                }

                callee
            }
            HirExprKind::Identifier(..) => {
                let resolution = self
                    .resolution_table
                    .get_name_resolution(&expression.id)
                    .unwrap();

                match resolution {
                    Resolution::Variable(id)
                    | Resolution::Function(id)
                    | Resolution::Struct(id)
                    | Resolution::Type(id) => self.resolution_table.get_type_resolution(id),
                }
                .unwrap()
                .clone()
            }
            HirExprKind::StringLiteral(..) => CheckedTy::String,
            HirExprKind::BooleanLiteral(..) => CheckedTy::Boolean,
            HirExprKind::NumberLiteral(..) => CheckedTy::Number,
        };

        Ok(ty)
    }

    pub fn check_type(&mut self, ty: &HirTy) -> CheckedTy {
        match &ty.kind {
            HirTyKind::Function {
                parameters,
                return_ty,
            } => {
                let parameters = parameters
                    .iter()
                    .map(|param| self.check_type(param))
                    .collect();

                let return_ty = match return_ty {
                    Some(ty) => self.check_type(ty),
                    None => CheckedTy::Void,
                };

                CheckedTy::function(parameters, return_ty)
            }
            HirTyKind::Identifier(name) => {
                if let Some(Resolution::Struct(id)) =
                    self.resolution_table.get_name_resolution(&ty.id)
                {
                    CheckedTy::Identifier(*id)
                } else {
                    unreachable!()
                }
            }
            HirTyKind::Bool => CheckedTy::Boolean,
            HirTyKind::Number => CheckedTy::Number,
        }
    }
}
