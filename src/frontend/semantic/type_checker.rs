#![allow(clippy::new_without_default)]
#![allow(clippy::only_used_in_recursion)]

use crate::{
    error::kaori_error::KaoriError,
    frontend::{
        hir::{
            hir_ast_node::HirAstNode,
            hir_decl::{HirDecl, HirDeclKind},
            hir_expr::{HirExpr, HirExprKind},
            hir_stmt::{HirStmt, HirStmtKind},
            node_id::NodeId,
        },
        syntax::{
            operator::{BinaryOp, UnaryOp},
            ty::{Ty, TyKind},
        },
    },
    kaori_error,
};

use super::resolution_table::ResolutionTable;

pub struct TypeChecker<'a> {
    function_return_ty: Option<Ty>,
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
        for declaration in declarations.iter() {
            match &declaration.kind {
                HirDeclKind::Function {
                    parameters,
                    return_ty,
                    ..
                } => {}
                HirDeclKind::Struct { name, .. } => {}
                _ => (),
            }
        }

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
            HirDeclKind::Struct { name, fields } => todo!(),
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

    fn check_expression(&mut self, expression: &HirExpr) -> Result<(), KaoriError> {
        match &expression.kind {
            HirExprKind::Assign(left, right) => {
                let right = self.check_expression(right)?;
                let left = self.check_expression(left)?;
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
                self.check_expression(left)?;
                self.check_expression(right)?;
            }
            HirExprKind::Negate(right) | HirExprKind::Not(right) => {
                self.check_expression(right)?;
            }
            HirExprKind::FunctionCall { callee, arguments } => {
                self.check_expression(callee)?;

                for argument in arguments {
                    self.check_expression(argument)?;
                }
            }
            HirExprKind::Identifier(..) => {}
            HirExprKind::StringLiteral(..) => {}
            HirExprKind::BooleanLiteral(..) => {}
            HirExprKind::NumberLiteral(..) => {}
        };

        Ok(())
    }

    pub fn check_type(&mut self, ty: &Ty) -> Result<(), KaoriError> {
        match &ty.kind {
            TyKind::Function {
                parameters,
                return_ty,
            } => {
                for parameter in parameters {
                    self.check_type(parameter)?;
                }

                self.check_type(return_ty)?;
            }
            TyKind::Struct { fields } => {
                for field in fields {
                    self.check_type(field)?;
                }
            }
            TyKind::Custom { .. } => {}
            TyKind::Boolean => {}
            TyKind::Number => {}
            TyKind::String => {}
            TyKind::Void => {}
        };

        Ok(())
    }
}
