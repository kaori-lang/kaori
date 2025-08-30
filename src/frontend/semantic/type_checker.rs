#![allow(clippy::new_without_default)]
#![allow(clippy::only_used_in_recursion)]

use crate::{
    error::kaori_error::KaoriError,
    frontend::{
        semantic::{
            resolved_ast_node::ResolvedAstNode,
            resolved_decl::{ResolvedDecl, ResolvedDeclKind},
            resolved_expr::{ResolvedExpr, ResolvedExprKind},
            resolved_stmt::{ResolvedStmt, ResolvedStmtKind},
        },
        syntax::operator::{BinaryOp, UnaryOp},
    },
    kaori_error,
};

use super::resolved_ty::{ResolvedTy, ResolvedTyKind};

pub struct TypeChecker {
    function_return_ty: Option<ResolvedTy>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            function_return_ty: None,
        }
    }

    pub fn check(&mut self, declarations: &[ResolvedDecl]) -> Result<(), KaoriError> {
        for declaration in declarations {
            self.check_declaration(declaration)?;
        }

        Ok(())
    }

    fn check_nodes(&mut self, nodes: &[ResolvedAstNode]) -> Result<(), KaoriError> {
        for node in nodes {
            self.check_ast_node(node)?;
        }

        Ok(())
    }

    fn check_ast_node(&mut self, node: &ResolvedAstNode) -> Result<(), KaoriError> {
        match node {
            ResolvedAstNode::Declaration(declaration) => self.check_declaration(declaration),
            ResolvedAstNode::Statement(statement) => self.check_statement(statement),
        }
    }

    fn check_declaration(&mut self, declaration: &ResolvedDecl) -> Result<(), KaoriError> {
        match &declaration.kind {
            ResolvedDeclKind::Variable { right, ty, .. } => {
                let right_ty = self.check_expression(right)?;

                if !right_ty.eq(ty) {
                    return Err(kaori_error!(
                        right.span,
                        "expected {} for the right hand side, but found {}",
                        ty,
                        right_ty
                    ));
                }
            }
            ResolvedDeclKind::Function { body, ty, .. } => {
                if let ResolvedTyKind::Function { return_ty, .. } = &ty.kind {
                    self.function_return_ty = Some(*return_ty.to_owned());
                }

                self.check_nodes(body)?;

                self.function_return_ty = None;
            }
        }

        Ok(())
    }

    fn check_statement(&mut self, statement: &ResolvedStmt) -> Result<(), KaoriError> {
        match &statement.kind {
            ResolvedStmtKind::Expression(expression) => {
                self.check_expression(expression)?;
            }
            ResolvedStmtKind::Print(expression) => {
                self.check_expression(expression)?;
            }
            ResolvedStmtKind::Block(nodes) => {
                for node in nodes {
                    self.check_ast_node(node)?;
                }
            }
            ResolvedStmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition_ty = self.check_expression(condition)?;

                let ResolvedTyKind::Boolean = condition_ty.kind else {
                    return Err(kaori_error!(
                        condition.span,
                        "expected boolean for condition, but found {}",
                        condition_ty
                    ));
                };

                self.check_statement(then_branch)?;

                if let Some(branch) = else_branch {
                    self.check_statement(branch)?;
                }
            }
            ResolvedStmtKind::WhileLoop {
                condition, block, ..
            } => {
                let condition_ty = self.check_expression(condition)?;

                let ResolvedTyKind::Boolean = condition_ty.kind else {
                    return Err(kaori_error!(
                        condition.span,
                        "expected boolean for condition, but found {}",
                        condition_ty
                    ));
                };

                self.check_statement(block)?;
            }
            ResolvedStmtKind::Return(expr) => {
                let return_ty = match expr {
                    Some(expr) => self.check_expression(expr)?,
                    None => ResolvedTy::void(statement.span),
                };

                if let Some(function_return_ty) = &self.function_return_ty
                    && !return_ty.eq(function_return_ty)
                {
                    return Err(kaori_error!(
                        statement.span,
                        "expected {} for function return type, but found {}",
                        function_return_ty,
                        return_ty
                    ));
                }
            }
            ResolvedStmtKind::Break => {}
            ResolvedStmtKind::Continue => {}
        };

        Ok(())
    }

    fn check_expression(&self, expression: &ResolvedExpr) -> Result<ResolvedTy, KaoriError> {
        let type_ = match &expression.kind {
            ResolvedExprKind::Assign { left, right } => {
                let ResolvedExprKind::LocalRef { .. } = left.kind else {
                    return Err(kaori_error!(left.span, "expected a variable to assign to",));
                };

                let right_ty = self.check_expression(right)?;
                let left_ty = self.check_expression(left)?;

                if !right_ty.eq(&left_ty) {
                    return Err(kaori_error!(
                        right.span,
                        "expected {} for assign, but found {}",
                        left_ty,
                        right_ty
                    ));
                }

                right_ty
            }
            ResolvedExprKind::Binary {
                left,
                right,
                operator,
            } => {
                let left_ty = self.check_expression(left)?;
                let right_ty = self.check_expression(right)?;

                use BinaryOp::*;
                use ResolvedTyKind::{Boolean as Bool, Number as Num};

                match (&left_ty.kind, operator, &right_ty.kind) {
                    (Num, Add | Subtract | Multiply | Divide | Modulo, Num) => {
                        ResolvedTy::number(expression.span)
                    }

                    (Bool, And | Or, Bool) => ResolvedTy::boolean(expression.span),

                    (Num, Equal | NotEqual, Num) => ResolvedTy::boolean(expression.span),

                    (Bool, Equal | NotEqual, Bool) => ResolvedTy::boolean(expression.span),

                    (Num, Greater | GreaterEqual | Less | LessEqual, Num) => {
                        ResolvedTy::boolean(expression.span)
                    }
                    _ => {
                        return Err(kaori_error!(
                            expression.span,
                            "expected valid types for {:?} operation, but found: {} and {}",
                            operator,
                            left_ty,
                            right_ty
                        ));
                    }
                }
            }
            ResolvedExprKind::Unary { right, operator } => {
                let right_ty = self.check_expression(right)?;

                use ResolvedTyKind::{Boolean as Bool, Number as Num};
                use UnaryOp::*;

                match (operator, &right_ty.kind) {
                    (Negate, Num) => ResolvedTy::number(expression.span),
                    (Not, Bool) => ResolvedTy::boolean(expression.span),
                    _ => {
                        return Err(kaori_error!(
                            expression.span,
                            "expected valid type for {:?} operation, but found {}",
                            operator,
                            right_ty
                        ));
                    }
                }
            }
            ResolvedExprKind::FunctionCall {
                callee, arguments, ..
            } => {
                let callee_ty = self.check_expression(callee)?;

                let ResolvedTyKind::Function {
                    parameters,
                    return_ty,
                } = callee_ty.kind
                else {
                    return Err(kaori_error!(
                        callee.span,
                        "expected a callable function, but found"
                    ));
                };

                if parameters.len() != arguments.len() {
                    return Err(kaori_error!(
                        callee.span,
                        "expected {} arguments, but found {}",
                        parameters.len(),
                        arguments.len()
                    ));
                }

                for (argument, parameter_ty) in arguments.iter().zip(parameters) {
                    let argument_ty = self.check_expression(argument)?;

                    if !argument_ty.eq(&parameter_ty) {
                        return Err(kaori_error!(
                            argument.span,
                            "expected {}, but found argument of type {}",
                            parameter_ty,
                            argument_ty
                        ));
                    }
                }

                *return_ty
            }
            ResolvedExprKind::GlobalRef { ty, .. } => ty.to_owned(),
            ResolvedExprKind::LocalRef { ty, .. } => ty.to_owned(),
            ResolvedExprKind::NumberLiteral(..) => ResolvedTy::number(expression.span),
            ResolvedExprKind::BooleanLiteral(..) => ResolvedTy::boolean(expression.span),
            ResolvedExprKind::StringLiteral(..) => ResolvedTy::string(expression.span),
        };

        Ok(type_)
    }
}
