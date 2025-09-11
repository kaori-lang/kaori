#![allow(clippy::new_without_default)]
#![allow(clippy::only_used_in_recursion)]

use std::collections::HashMap;

use crate::{error::kaori_error::KaoriError, kaori_error};

use super::{
    hir_decl::{HirDecl, HirDeclKind},
    hir_expr::{HirExpr, HirExprKind},
    hir_id::HirId,
    hir_node::HirNode,
    hir_stmt::{HirStmt, HirStmtKind},
    hir_ty::{HirTy, HirTyKind},
    type_def::TypeDef,
};

pub struct TypeChecker {
    function_return_ty: Option<TypeDef>,
    type_definitions: HashMap<HirId, TypeDef>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            function_return_ty: None,
            type_definitions: HashMap::new(),
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
                HirDeclKind::Struct { fields } => {}
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
    fn check_nodes(&mut self, nodes: &[HirNode]) -> Result<(), KaoriError> {
        for node in nodes {
            self.check_ast_node(node)?;
        }

        Ok(())
    }

    fn check_ast_node(&mut self, node: &HirNode) -> Result<(), KaoriError> {
        match node {
            HirNode::Declaration(declaration) => self.check_declaration(declaration),
            HirNode::Statement(statement) => self.check_statement(statement),
        }?;

        Ok(())
    }

    fn check_declaration(&mut self, declaration: &HirDecl) -> Result<(), KaoriError> {
        match &declaration.kind {
            HirDeclKind::Variable { right, ty, .. } => {
                let right = self.check_expression(right)?;
                let ty = self.get_type_def(ty);

                if right != ty {
                    return Err(kaori_error!(
                        declaration.span,
                        "expected {:#?} type on variable declaration, but found: {:#?}",
                        ty,
                        right
                    ));
                }
            }
            HirDeclKind::Parameter { .. } => {}
            HirDeclKind::Field { .. } => {}
            HirDeclKind::Function {
                body, return_ty, ..
            } => {
                self.function_return_ty = return_ty.as_ref().map(|ty| self.get_type_def(ty));

                let mut return_count = 0;

                for node in body {
                    if let HirNode::Statement(statement) = node
                        && let HirStmtKind::Break = statement.kind
                    {
                        return_count += 1;
                    }
                }

                if let Some(..) = return_ty
                    && return_count == 0
                {
                    return Err(kaori_error!(
                        declaration.span,
                        "expected a return type of {:#?}, but found no return statement",
                        return_ty
                    ));
                }

                for node in body {
                    self.check_ast_node(node)?;
                }
            }
            HirDeclKind::Struct { fields } => {}
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
            HirStmtKind::Loop {
                init,
                condition,
                block,
            } => {
                if let Some(init) = init {
                    self.check_declaration(init)?;
                }
                self.check_expression(condition)?;

                self.check_statement(block)?;
            }
            HirStmtKind::Break => {}
            HirStmtKind::Continue => {}
            HirStmtKind::Return(expr) => {
                if let Some(expr) = expr {
                    let ty = self.check_expression(expr)?;
                }
            }
        };

        Ok(())
    }

    fn check_expression(&mut self, expression: &HirExpr) -> Result<TypeDef, KaoriError> {
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
            HirExprKind::FunctionRef(id) => self.type_definitions.get(id).unwrap().to_owned(),
            HirExprKind::VariableRef(id) => self.type_definitions.get(id).unwrap().to_owned(),
            HirExprKind::StringLiteral(..) => TypeDef::String,
            HirExprKind::BooleanLiteral(..) => TypeDef::Boolean,
            HirExprKind::NumberLiteral(..) => TypeDef::Number,
        };

        Ok(ty)
    }

    pub fn get_type_def(&mut self, ty: &HirTy) -> TypeDef {
        match &ty.kind {
            HirTyKind::Function {
                parameters,
                return_ty,
            } => {
                let parameters = parameters
                    .iter()
                    .map(|param| self.get_type_def(param))
                    .collect();

                let return_ty = match return_ty {
                    Some(ty) => self.get_type_def(ty),
                    None => TypeDef::Void,
                };

                TypeDef::function(parameters, return_ty)
            }
            HirTyKind::TypeRef(id) => self.type_definitions.get(id).unwrap().to_owned(),
            HirTyKind::Bool => TypeDef::Boolean,
            HirTyKind::Number => TypeDef::Number,
        }
    }
}
