#![allow(clippy::new_without_default)]
use crate::{
    compiler::syntax::{
        ast_node::ASTNode,
        declaration::{Decl, DeclKind},
        expression::{Expr, ExprKind},
        operator::{BinaryOp, UnaryOp},
        statement::{Stmt, StmtKind},
        r#type::Type,
    },
    error::kaori_error::KaoriError,
    kaori_error,
};

use super::{environment::Environment, visitor::Visitor};

pub struct TypeChecker {
    environment: Environment<Type>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn check(&mut self, ast: &mut [ASTNode]) -> Result<(), KaoriError> {
        self.environment.enter_function();

        for i in 0..ast.len() {
            if let Some(ASTNode::Declaration(decl)) = ast.get(i)
                && let DeclKind::Function {
                    type_annotation, ..
                } = &decl.kind
            {
                self.environment.declare(type_annotation.clone());
            }
        }

        for i in 0..ast.len() {
            if let Some(node) = ast.get_mut(i) {
                self.visit_ast_node(node)?;
            }
        }

        self.environment.exit_function();

        Ok(())
    }
}

impl Visitor<Type> for TypeChecker {
    fn visit_ast_node(&mut self, ast_node: &mut ASTNode) -> Result<(), KaoriError> {
        match ast_node {
            ASTNode::Declaration(declaration) => self.visit_declaration(declaration),
            ASTNode::Statement(statement) => self.visit_statement(statement),
        }
    }

    fn visit_declaration(&mut self, declaration: &mut Decl) -> Result<(), KaoriError> {
        match &mut declaration.kind {
            DeclKind::Variable {
                right,
                type_annotation,
                ..
            } => {
                let right_type = self.visit_expression(right)?;

                if right_type != *type_annotation {
                    return Err(kaori_error!(
                        right.span,
                        "expected value of type {:?} in variable declaration",
                        type_annotation,
                    ));
                }

                self.environment.declare(right_type);
            }
            DeclKind::Function {
                parameters, block, ..
            } => {
                self.environment.enter_function();

                for parameter in parameters {
                    self.visit_declaration(parameter)?;
                }

                let StmtKind::Block(declarations) = &mut block.kind else {
                    unreachable!()
                };

                for declaration in declarations {
                    self.visit_ast_node(declaration)?;
                }

                self.environment.exit_function();
            }
        }

        Ok(())
    }

    fn visit_statement(&mut self, statement: &mut Stmt) -> Result<(), KaoriError> {
        match &mut statement.kind {
            StmtKind::Expression(expression) => {
                self.visit_expression(expression.as_mut())?;
            }
            StmtKind::Print(expression) => {
                self.visit_expression(expression.as_mut())?;
            }
            StmtKind::Block(declarations) => {
                self.environment.enter_scope();

                for declaration in declarations {
                    self.visit_ast_node(declaration)?;
                }

                self.environment.exit_scope();
            }
            StmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let expr = self.visit_expression(condition)?;

                if !expr.eq(&Type::Boolean) {
                    return Err(kaori_error!(
                        condition.span,
                        "expected {:?} for if statement condition, but found {:?}",
                        Type::Boolean,
                        expr
                    ));
                }

                self.visit_statement(then_branch)?;

                if let Some(branch) = else_branch {
                    self.visit_statement(branch)?;
                }
            }
            StmtKind::WhileLoop { condition, block } => {
                let expr = self.visit_expression(condition)?;

                if !expr.eq(&Type::Boolean) {
                    return Err(kaori_error!(
                        condition.span,
                        "expected {:?} for while loop statement condition, but found {:?}",
                        Type::Boolean,
                        expr
                    ));
                }

                self.visit_statement(block)?;
            }
            _ => (),
        };

        Ok(())
    }
    fn visit_expression(&mut self, expression: &mut Expr) -> Result<Type, KaoriError> {
        let expr_type = match &mut expression.kind {
            ExprKind::Assign { identifier, right } => {
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
            ExprKind::Binary {
                left,
                right,
                operator,
            } => {
                let left = self.visit_expression(left)?;
                let right = self.visit_expression(right)?;

                match (&left, &operator, &right) {
                    (Type::Number, BinaryOp::Plus, Type::Number) => Type::Number,
                    (Type::Number, BinaryOp::Minus, Type::Number) => Type::Number,
                    (Type::Number, BinaryOp::Multiply, Type::Number) => Type::Number,
                    (Type::Number, BinaryOp::Divide, Type::Number) => Type::Number,
                    (Type::Number, BinaryOp::Modulo, Type::Number) => Type::Number,

                    (Type::Boolean, BinaryOp::And, Type::Boolean) => Type::Boolean,
                    (Type::Boolean, BinaryOp::Or, Type::Boolean) => Type::Boolean,

                    (Type::Number, BinaryOp::Equal, Type::Number) => Type::Boolean,
                    (Type::Boolean, BinaryOp::Equal, Type::Boolean) => Type::Boolean,

                    (Type::Number, BinaryOp::NotEqual, Type::Number) => Type::Boolean,
                    (Type::Boolean, BinaryOp::NotEqual, Type::Boolean) => Type::Boolean,

                    (Type::Number, BinaryOp::Greater, Type::Number) => Type::Boolean,
                    (Type::Number, BinaryOp::GreaterEqual, Type::Number) => Type::Boolean,
                    (Type::Number, BinaryOp::Less, Type::Number) => Type::Boolean,
                    (Type::Number, BinaryOp::LessEqual, Type::Number) => Type::Boolean,
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
            ExprKind::Unary { right, operator } => {
                let right = self.visit_expression(right)?;

                match (&operator, &right) {
                    (UnaryOp::Negate, Type::Number) => Type::Number,
                    (UnaryOp::Not, Type::Boolean) => Type::Boolean,
                    _ => {
                        return Err(kaori_error!(
                            expression.span,
                            "invalid {:?} operation for right {:?}",
                            operator,
                            right
                        ));
                    }
                }
            }
            ExprKind::Identifier { resolution, .. } => self.environment.get(*resolution).clone(),
            ExprKind::NumberLiteral(..) => Type::Number,
            ExprKind::BooleanLiteral(..) => Type::Boolean,
            ExprKind::StringLiteral(..) => Type::String,
        };

        Ok(expr_type)
    }
}
