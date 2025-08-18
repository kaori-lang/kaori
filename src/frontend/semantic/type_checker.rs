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
        syntax::{
            operator::{BinaryOp, UnaryOp},
            ty::Ty,
        },
    },
    kaori_error,
};

pub struct TypeChecker {
    return_type: Option<Ty>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self { return_type: None }
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
                let right_type = self.check_expression(right)?;

                if !right_type.eq(ty) {
                    return Err(kaori_error!(
                        right.span,
                        "expected value of type {:?}, but found {:?}",
                        ty,
                        right_type
                    ));
                }
            }
            ResolvedDeclKind::Function { body, ty, .. } => {
                if let Ty::Function { return_type, .. } = &ty {
                    self.return_type = Some(*return_type.to_owned());
                }

                self.check_nodes(body)?;
                self.return_type = None;
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
                let condition_type = self.check_expression(condition)?;

                if !condition_type.eq(&Ty::Boolean) {
                    return Err(kaori_error!(
                        condition.span,
                        "expected {:?} for if statement condition, but found {:?}",
                        Ty::Boolean,
                        condition_type
                    ));
                }

                self.check_statement(then_branch)?;

                if let Some(branch) = else_branch {
                    self.check_statement(branch)?;
                }
            }
            ResolvedStmtKind::WhileLoop {
                condition, block, ..
            } => {
                let condition_type = self.check_expression(condition)?;

                if !condition_type.eq(&Ty::Boolean) {
                    return Err(kaori_error!(
                        condition.span,
                        "expected {:?} for while loop statement condition, but found {:?}",
                        Ty::Boolean,
                        condition_type
                    ));
                }

                self.check_statement(block)?;
            }
            ResolvedStmtKind::Return(expr) => {
                let return_type = match expr {
                    Some(expr) => self.check_expression(expr)?,
                    None => Ty::Nothing,
                };

                if let Some(current_return_type) = &self.return_type
                    && !return_type.eq(current_return_type)
                {
                    return Err(kaori_error!(
                        statement.span,
                        "expected {:?} for function return type, but found return with type of {:?}",
                        current_return_type,
                        return_type
                    ));
                }
            }
            _ => (),
        };

        Ok(())
    }

    fn check_expression(&self, expression: &ResolvedExpr) -> Result<Ty, KaoriError> {
        let type_ = match &expression.kind {
            ResolvedExprKind::Assign { left, right } => {
                let ResolvedExprKind::VariableRef { .. } = left.kind else {
                    return Err(kaori_error!(
                        expression.span,
                        "invalid assign to a non variable {:?}",
                        left.span,
                    ));
                };

                let right = self.check_expression(right)?;
                let left = self.check_expression(left)?;

                if !right.eq(&left) {
                    return Err(kaori_error!(
                        expression.span,
                        "expected {:?} for assign, but found {:?}",
                        left,
                        right
                    ));
                }

                right
            }
            ResolvedExprKind::Binary {
                left,
                right,
                operator,
            } => {
                let left = self.check_expression(left)?;
                let right = self.check_expression(right)?;

                match (&left, &operator, &right) {
                    (Ty::Number, BinaryOp::Add, Ty::Number) => Ty::Number,
                    (Ty::Number, BinaryOp::Subtract, Ty::Number) => Ty::Number,
                    (Ty::Number, BinaryOp::Multiply, Ty::Number) => Ty::Number,
                    (Ty::Number, BinaryOp::Divide, Ty::Number) => Ty::Number,
                    (Ty::Number, BinaryOp::Modulo, Ty::Number) => Ty::Number,

                    (Ty::Boolean, BinaryOp::And, Ty::Boolean) => Ty::Boolean,
                    (Ty::Boolean, BinaryOp::Or, Ty::Boolean) => Ty::Boolean,

                    (Ty::Number, BinaryOp::Equal, Ty::Number) => Ty::Boolean,
                    (Ty::Boolean, BinaryOp::Equal, Ty::Boolean) => Ty::Boolean,

                    (Ty::Number, BinaryOp::NotEqual, Ty::Number) => Ty::Boolean,
                    (Ty::Boolean, BinaryOp::NotEqual, Ty::Boolean) => Ty::Boolean,

                    (Ty::Number, BinaryOp::Greater, Ty::Number) => Ty::Boolean,
                    (Ty::Number, BinaryOp::GreaterEqual, Ty::Number) => Ty::Boolean,
                    (Ty::Number, BinaryOp::Less, Ty::Number) => Ty::Boolean,
                    (Ty::Number, BinaryOp::LessEqual, Ty::Number) => Ty::Boolean,
                    _ => {
                        return Err(kaori_error!(
                            expression.span,
                            "invalid {:?} operation between {:?} and {:?}",
                            operator,
                            left,
                            right
                        ));
                    }
                }
            }
            ResolvedExprKind::Unary { right, operator } => {
                let right = self.check_expression(right)?;

                match (&operator, &right) {
                    (UnaryOp::Negate, Ty::Number) => Ty::Number,
                    (UnaryOp::Not, Ty::Boolean) => Ty::Boolean,
                    _ => {
                        return Err(kaori_error!(
                            expression.span,
                            "invalid {:?} operation for {:?}",
                            operator,
                            right
                        ));
                    }
                }
            }
            ResolvedExprKind::FunctionCall {
                callee, arguments, ..
            } => {
                let Ty::Function {
                    parameters,
                    return_type,
                } = self.check_expression(callee)?
                else {
                    return Err(kaori_error!(
                        callee.span,
                        "invalid function call to a non callable"
                    ));
                };

                if parameters.len() != arguments.len() {
                    return Err(kaori_error!(
                        callee.span,
                        "invalid number of arguments, it must match number of parameters"
                    ));
                }

                for (argument, parameter_type) in arguments.iter().zip(parameters) {
                    let argument_type = self.check_expression(argument)?;
                    if !argument_type.eq(&parameter_type) {
                        return Err(kaori_error!(
                            argument.span,
                            "expected {:?}, but found argument of type {:?}",
                            parameter_type,
                            argument_type
                        ));
                    }
                }

                *return_type
            }
            ResolvedExprKind::VariableRef { ty, .. } => ty.to_owned(),
            ResolvedExprKind::FunctionRef { ty, .. } => ty.to_owned(),
            ResolvedExprKind::NumberLiteral(..) => Ty::Number,
            ResolvedExprKind::BooleanLiteral(..) => Ty::Boolean,
            ResolvedExprKind::StringLiteral(..) => Ty::String,
        };

        Ok(type_)
    }
}
