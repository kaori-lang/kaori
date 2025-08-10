#![allow(clippy::new_without_default)]

use crate::{
    error::kaori_error::KaoriError,
    frontend::syntax::{
        ast_node::ASTNode,
        declaration::{Decl, DeclKind},
        expression::{Expr, ExprKind},
        operator::{BinaryOp, UnaryOp},
        statement::{Stmt, StmtKind},
        r#type::Type,
    },
    kaori_error,
    utils::visitor::Visitor,
};

use super::environment::Environment;

pub struct TypeChecker {
    environment: Environment<Type>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            environment: Environment::default(),
        }
    }

    pub fn check(&mut self, nodes: &mut [ASTNode]) -> Result<(), KaoriError> {
        self.visit_nodes(nodes)
    }
}

impl Visitor<Type> for TypeChecker {
    fn visit_nodes(&mut self, nodes: &mut [ASTNode]) -> Result<(), KaoriError> {
        for node in nodes.iter().as_slice() {
            if let ASTNode::Declaration(declaration) = node
                && let DeclKind::Function {
                    type_annotation, ..
                } = &declaration.kind
            {
                self.environment.declare(type_annotation.to_owned());
            }
        }

        for node in nodes {
            self.visit_ast_node(node)?;
        }

        Ok(())
    }

    fn visit_ast_node(&mut self, node: &mut ASTNode) -> Result<(), KaoriError> {
        match node {
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

                if !right_type.eq(type_annotation) {
                    return Err(kaori_error!(
                        right.span,
                        "expected value of type {:?}, but found {:?}",
                        type_annotation,
                        right_type
                    ));
                }

                self.environment.declare(right_type);
            }
            DeclKind::Function {
                parameters, body, ..
            } => {
                self.environment.enter_scope();

                for parameter in parameters {
                    self.environment
                        .declare(parameter.type_annotation.to_owned());
                }

                self.visit_nodes(body)?;

                self.environment.exit_scope();
            }
        }

        Ok(())
    }

    fn visit_statement(&mut self, statement: &mut Stmt) -> Result<(), KaoriError> {
        match &mut statement.kind {
            StmtKind::Expression(expression) => {
                self.visit_expression(expression)?;
            }
            StmtKind::Print(expression) => {
                self.visit_expression(expression)?;
            }
            StmtKind::Block(nodes) => {
                self.environment.enter_scope();

                for node in nodes {
                    self.visit_ast_node(node)?;
                }

                self.environment.exit_scope();
            }
            StmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition_type = self.visit_expression(condition)?;

                if !condition_type.eq(&Type::Boolean) {
                    return Err(kaori_error!(
                        condition.span,
                        "expected {:?} for if statement condition, but found {:?}",
                        Type::Boolean,
                        condition_type
                    ));
                }

                self.visit_statement(then_branch)?;

                if let Some(branch) = else_branch {
                    self.visit_statement(branch)?;
                }
            }
            StmtKind::WhileLoop { condition, block } => {
                let condition_type = self.visit_expression(condition)?;

                if !condition_type.eq(&Type::Boolean) {
                    return Err(kaori_error!(
                        condition.span,
                        "expected {:?} for while loop statement condition, but found {:?}",
                        Type::Boolean,
                        condition_type
                    ));
                }

                self.visit_statement(block)?;
            }
            _ => (),
        };

        Ok(())
    }
    fn visit_expression(&mut self, expression: &mut Expr) -> Result<Type, KaoriError> {
        let type_ = match &mut expression.kind {
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
                            "invalid {:?} operation for {:?}",
                            operator,
                            right
                        ));
                    }
                }
            }
            ExprKind::Identifier { resolution, .. } => self.environment.get(*resolution).to_owned(),
            ExprKind::FunctionCall { callee, arguments } => {
                let Type::Function {
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

                for (argument, parameter_type) in arguments.iter_mut().zip(parameters) {
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
            ExprKind::NumberLiteral(..) => Type::Number,
            ExprKind::BooleanLiteral(..) => Type::Boolean,
            ExprKind::StringLiteral(..) => Type::String,
        };

        Ok(type_)
    }
}
