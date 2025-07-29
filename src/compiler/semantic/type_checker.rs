use crate::{
    compilation_error,
    compiler::syntax::{
        ast_node::ASTNode,
        declaration::{Decl, DeclKind},
        expression::{Expr, ExprKind},
        operator::{BinaryOp, UnaryOp},
        r#type::Type,
        statement::{Stmt, StmtKind},
    },
    error::compilation_error::{self, CompilationError},
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
}

impl Visitor<Type> for TypeChecker {
    fn run(&mut self, ast: &mut Vec<ASTNode>) -> Result<(), CompilationError> {
        self.environment.enter_function();

        for node in ast {
            self.visit_ast_node(node)?;
        }

        self.environment.exit_function();

        Ok(())
    }

    fn visit_ast_node(&mut self, ast_node: &mut ASTNode) -> Result<(), CompilationError> {
        match ast_node {
            ASTNode::Declaration(declaration) => self.visit_declaration(declaration),
            ASTNode::Statement(statement) => self.visit_statement(statement),
        }
    }

    fn visit_declaration(&mut self, declaration: &mut Decl) -> Result<(), CompilationError> {
        match &mut declaration.kind {
            DeclKind::Variable { name, right, ty } => {
                let right = self.visit_expression(right)?;

                if right != *ty {
                    return Err(compilation_error!(
                        declaration.span,
                        "expected value of type {:?} in variable declaration",
                        ty,
                    ));
                }

                self.environment.declare(right);
            }
        }

        Ok(())
    }

    fn visit_statement(&mut self, statement: &mut Stmt) -> Result<(), CompilationError> {
        match &mut statement.kind {
            StmtKind::Expression(expression) => {
                self.visit_expression(expression.as_mut())?;
                ()
            }
            StmtKind::Print(expression) => {
                self.visit_expression(expression.as_mut())?;
                ()
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
                ..
            } => {
                self.visit_expression(condition)?;
                self.visit_statement(then_branch)?;

                if let Some(branch) = else_branch {
                    self.visit_statement(branch)?;
                }
            }
            StmtKind::WhileLoop {
                condition, block, ..
            } => {
                self.visit_expression(condition)?;
                self.visit_statement(block)?;
            }
            _ => (),
        };

        Ok(())
    }
    fn visit_expression(&mut self, expression: &mut Expr) -> Result<Type, CompilationError> {
        let expr_type = match &mut expression.kind {
            ExprKind::Assign { identifier, right } => {
                let right = self.visit_expression(right)?;
                let identifier = self.visit_expression(identifier)?;

                if right != identifier {
                    return Err(compilation_error!(
                        expression.span,
                        "can't assign type {:?} to type {:?}",
                        right,
                        identifier
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
                    (Type::Number, BinaryOp::Remainder, Type::Number) => Type::Number,

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
                        return Err(compilation_error!(
                            expression.span,
                            "invalid {:?} operation between {:?} and {:?}",
                            operator,
                            left,
                            right
                        ))
                    }
                }
            }
            ExprKind::Unary { right, operator } => {
                let right = self.visit_expression(right)?;

                match (&operator, &right) {
                    (UnaryOp::Negate, Type::Number) => Type::Number,
                    (UnaryOp::Not, Type::Boolean) => Type::Boolean,
                    _ => {
                        return Err(compilation_error!(
                            expression.span,
                            "invalid {:?} operation for right {:?}",
                            operator,
                            right
                        ))
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
