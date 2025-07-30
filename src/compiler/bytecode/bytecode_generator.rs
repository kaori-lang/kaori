#![allow(clippy::new_without_default)]

use crate::{
    compiler::{
        semantic::visitor::Visitor,
        syntax::{
            ast_node::ASTNode,
            declaration::{self, Decl, DeclKind},
            expression::{Expr, ExprKind},
            operator::{BinaryOp, UnaryOp},
            statement::{Stmt, StmtKind},
        },
    },
    error::kaori_error::KaoriError,
};

use super::instruction::Instruction;

pub struct BytecodeGenerator {
    bytecode: Vec<Instruction>,
}

impl BytecodeGenerator {
    pub fn new() -> Self {
        Self {
            bytecode: Vec::new(),
        }
    }

    pub fn generate(&mut self, ast: &mut Vec<ASTNode>) -> Result<&Vec<Instruction>, KaoriError> {
        self.bytecode.push(Instruction::EnterScope);

        for i in 0..ast.len() {
            if let Some(ASTNode::Declaration(decl)) = ast.get(i)
                && let DeclKind::Function { .. } = &decl.kind
            {
                //self.environment.declare(type_annotation.clone());
            }
        }

        for i in 0..ast.len() {
            if let Some(node) = ast.get_mut(i) {
                self.visit_ast_node(node)?;
            }
        }

        self.bytecode.push(Instruction::ExitScope);

        Ok(&self.bytecode)
    }
}

impl Visitor<()> for BytecodeGenerator {
    fn visit_ast_node(&mut self, ast_node: &mut ASTNode) -> Result<(), KaoriError> {
        match ast_node {
            ASTNode::Declaration(declaration) => self.visit_declaration(declaration),
            ASTNode::Statement(statement) => self.visit_statement(statement),
        }
    }

    fn visit_declaration(&mut self, declaration: &mut Decl) -> Result<(), KaoriError> {
        match &mut declaration.kind {
            DeclKind::Variable { right, .. } => {
                self.visit_expression(right)?;
                self.bytecode.push(Instruction::Declare);
            }
            DeclKind::Function {
                parameters, block, ..
            } => {
                for parameter in parameters {
                    self.visit_declaration(parameter)?;
                }

                let StmtKind::Block(declarations) = &mut block.kind else {
                    unreachable!()
                };

                for declaration in declarations {
                    self.visit_ast_node(declaration)?;
                }
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

                self.bytecode.push(Instruction::Print);
            }
            StmtKind::Block(declarations) => {
                self.bytecode.push(Instruction::EnterScope);

                for declaration in declarations {
                    self.visit_ast_node(declaration)?;
                }

                self.bytecode.push(Instruction::ExitScope);
            }
            StmtKind::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {}
            StmtKind::WhileLoop {
                condition, block, ..
            } => {}
            _ => (),
        };

        Ok(())
    }
    fn visit_expression(&mut self, expression: &mut Expr) -> Result<(), KaoriError> {
        match &mut expression.kind {
            ExprKind::Assign { identifier, right } => {
                self.visit_expression(right)?;

                let ExprKind::Identifier { resolution, .. } = &identifier.kind else {
                    unreachable!();
                };

                if resolution.global {
                    let instruction = Instruction::StoreGlobal(resolution.offset);
                    self.bytecode.push(instruction);
                } else {
                    let instruction = Instruction::StoreLocal(resolution.offset);
                    self.bytecode.push(instruction);
                }
            }
            ExprKind::Binary {
                left,
                right,
                operator,
            } => {
                self.visit_expression(left)?;
                self.visit_expression(right)?;

                match operator {
                    BinaryOp::Plus => self.bytecode.push(Instruction::Plus),
                    BinaryOp::Minus => self.bytecode.push(Instruction::Minus),
                    BinaryOp::Multiply => self.bytecode.push(Instruction::Multiply),
                    BinaryOp::Divide => self.bytecode.push(Instruction::Divide),
                    BinaryOp::Modulo => self.bytecode.push(Instruction::Modulo),

                    BinaryOp::And => self.bytecode.push(Instruction::And),
                    BinaryOp::Or => self.bytecode.push(Instruction::Or),

                    BinaryOp::Equal => self.bytecode.push(Instruction::Equal),
                    BinaryOp::NotEqual => self.bytecode.push(Instruction::NotEqual),

                    BinaryOp::Greater => self.bytecode.push(Instruction::Greater),
                    BinaryOp::GreaterEqual => self.bytecode.push(Instruction::GreaterEqual),
                    BinaryOp::Less => self.bytecode.push(Instruction::Less),
                    BinaryOp::LessEqual => self.bytecode.push(Instruction::LessEqual),
                }
            }
            ExprKind::Unary { right, operator } => {
                self.visit_expression(right)?;

                match &operator {
                    UnaryOp::Negate => self.bytecode.push(Instruction::Negate),
                    UnaryOp::Not => self.bytecode.push(Instruction::Not),
                }
            }
            ExprKind::Identifier { resolution, .. } => {
                if resolution.global {
                    let instruction = Instruction::LoadGlobal(resolution.offset);

                    self.bytecode.push(instruction);
                } else {
                    let instruction = Instruction::LoadLocal(resolution.offset);

                    self.bytecode.push(instruction);
                }
            }
            ExprKind::NumberLiteral(value) => {
                let instruction = Instruction::number_const(*value);
                self.bytecode.push(instruction);
            }
            ExprKind::BooleanLiteral(value) => {
                let instruction = Instruction::bool_const(*value);
                self.bytecode.push(instruction);
            }
            ExprKind::StringLiteral(value) => {
                let instruction = Instruction::str_const(value);
                self.bytecode.push(instruction);
            }
        };

        Ok(())
    }
}
