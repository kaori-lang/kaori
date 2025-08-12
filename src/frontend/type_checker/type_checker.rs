#![allow(clippy::new_without_default)]

use crate::{
    error::kaori_error::KaoriError,
    frontend::{
        resolver::{
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

pub struct TypeChecker {}

impl TypeChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check(&mut self, nodes: &[ResolvedAstNode]) -> Result<(), KaoriError> {
        self.visit_nodes(nodes)
    }

    fn visit_nodes(&mut self, nodes: &[ResolvedAstNode]) -> Result<(), KaoriError> {
        for node in nodes {
            self.visit_ast_node(node)?;
        }

        Ok(())
    }

    fn visit_ast_node(&mut self, node: &ResolvedAstNode) -> Result<(), KaoriError> {
        match node {
            ResolvedAstNode::Declaration(declaration) => self.visit_declaration(declaration),
            ResolvedAstNode::Statement(statement) => self.visit_statement(statement),
        }
    }

    fn visit_declaration(&mut self, declaration: &ResolvedDecl) -> Result<(), KaoriError> {
        match &declaration.kind {
            ResolvedDeclKind::Variable { right, ty, .. } => {
                let right_type = self.visit_expression(right)?;

                if !right_type.eq(ty) {
                    return Err(kaori_error!(
                        right.span,
                        "expected value of type {:?}, but found {:?}",
                        ty,
                        right_type
                    ));
                }
            }
            ResolvedDeclKind::Function { body, .. } => {
                self.visit_nodes(body)?;
            }
        }

        Ok(())
    }

    fn visit_statement(&mut self, statement: &ResolvedStmt) -> Result<(), KaoriError> {
        match &statement.kind {
            ResolvedStmtKind::Expression(expression) => {
                self.visit_expression(expression)?;
            }
            ResolvedStmtKind::Print(expression) => {
                self.visit_expression(expression)?;
            }
            ResolvedStmtKind::Block(nodes) => {
                for node in nodes {
                    self.visit_ast_node(node)?;
                }
            }
            ResolvedStmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition_type = self.visit_expression(condition)?;

                if !condition_type.eq(&Ty::Boolean) {
                    return Err(kaori_error!(
                        condition.span,
                        "expected {:?} for if statement condition, but found {:?}",
                        Ty::Boolean,
                        condition_type
                    ));
                }

                self.visit_statement(then_branch)?;

                if let Some(branch) = else_branch {
                    self.visit_statement(branch)?;
                }
            }
            ResolvedStmtKind::WhileLoop { condition, block } => {
                let condition_type = self.visit_expression(condition)?;

                if !condition_type.eq(&Ty::Boolean) {
                    return Err(kaori_error!(
                        condition.span,
                        "expected {:?} for while loop statement condition, but found {:?}",
                        Ty::Boolean,
                        condition_type
                    ));
                }

                self.visit_statement(block)?;
            }
            _ => (),
        };

        Ok(())
    }
    fn visit_expression(&mut self, expression: &ResolvedExpr) -> Result<Ty, KaoriError> {
        let type_ = match &expression.kind {
            ResolvedExprKind::Assign { identifier, right } => {
                let right = self.visit_expression(right)?;
                let identifier = self.visit_expression(identifier)?;

                if !right.eq(&identifier) {
                    return Err(kaori_error!(
                        expression.span,
                        "expected {:?} for assign, but found {:?}",
                        identifier,
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
                let left = self.visit_expression(left)?;
                let right = self.visit_expression(right)?;

                match (&left, &operator, &right) {
                    (Ty::Number, BinaryOp::Plus, Ty::Number) => Ty::Number,
                    (Ty::Number, BinaryOp::Minus, Ty::Number) => Ty::Number,
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
                let right = self.visit_expression(right)?;

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
            ResolvedExprKind::VariableRef { ty, .. } => ty.to_owned(),
            ResolvedExprKind::FunctionRef { ty, .. } => ty.to_owned(),
            ResolvedExprKind::FunctionCall { callee, arguments } => {
                let Ty::Function {
                    parameters,
                    return_type,
                } = self.visit_expression(callee)?
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
                    let argument_type = self.visit_expression(argument)?;
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
            ResolvedExprKind::NumberLiteral(..) => Ty::Number,
            ResolvedExprKind::BooleanLiteral(..) => Ty::Boolean,
            ResolvedExprKind::StringLiteral(..) => Ty::String,
        };

        Ok(type_)
    }
}
