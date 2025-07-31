#![allow(clippy::new_without_default)]

use crate::{
    compiler::{
        semantic::visitor::Visitor,
        syntax::{
            ast_node::ASTNode,
            declaration::{Decl, DeclKind},
            expression::{Expr, ExprKind},
            operator::{BinaryOp, UnaryOp},
            statement::{Stmt, StmtKind},
        },
    },
    error::kaori_error::KaoriError,
};

use super::{bytecode::Bytecode, instruction::Instruction};

pub struct BytecodeGenerator {
    bytecode: Bytecode,
}

impl BytecodeGenerator {
    pub fn new() -> Self {
        Self {
            bytecode: Bytecode::new(),
        }
    }

    pub fn generate(&mut self, ast: &mut [ASTNode]) -> Result<&Bytecode, KaoriError> {
        self.bytecode.emit(Instruction::EnterScope);

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

        self.bytecode.emit(Instruction::ExitScope);

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
                self.bytecode.emit(Instruction::Declare);
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

                self.bytecode.emit(Instruction::Print);
            }
            StmtKind::Block(declarations) => {
                self.bytecode.emit(Instruction::EnterScope);

                for declaration in declarations {
                    self.visit_ast_node(declaration)?;
                }

                self.bytecode.emit(Instruction::ExitScope);
            }
            StmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.visit_expression(condition)?;

                let jump_if_false_placeholder = self.bytecode.create_placeholder();

                self.visit_statement(then_branch)?;

                let jump_end_placeholder = self.bytecode.create_placeholder();

                self.bytecode.update_placeholder(
                    jump_if_false_placeholder,
                    Instruction::JumpIfFalse(self.bytecode.index()),
                );

                if let Some(branch) = else_branch {
                    self.visit_statement(branch)?;
                }

                self.bytecode.update_placeholder(
                    jump_end_placeholder,
                    Instruction::Jump(self.bytecode.index()),
                );
            }
            StmtKind::WhileLoop { condition, block } => {
                let start = self.bytecode.index();

                self.visit_expression(condition)?;

                let jump_if_false_placeholder = self.bytecode.create_placeholder();

                self.visit_statement(block)?;

                self.bytecode.emit(Instruction::Jump(start));

                self.bytecode.update_placeholder(
                    jump_if_false_placeholder,
                    Instruction::JumpIfFalse(self.bytecode.index()),
                );
            }
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
                    self.bytecode.emit(instruction);
                } else {
                    let instruction = Instruction::StoreLocal(resolution.offset);
                    self.bytecode.emit(instruction);
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
                    BinaryOp::Plus => self.bytecode.emit(Instruction::Plus),
                    BinaryOp::Minus => self.bytecode.emit(Instruction::Minus),
                    BinaryOp::Multiply => self.bytecode.emit(Instruction::Multiply),
                    BinaryOp::Divide => self.bytecode.emit(Instruction::Divide),
                    BinaryOp::Modulo => self.bytecode.emit(Instruction::Modulo),

                    BinaryOp::And => self.bytecode.emit(Instruction::And),
                    BinaryOp::Or => self.bytecode.emit(Instruction::Or),

                    BinaryOp::Equal => self.bytecode.emit(Instruction::Equal),
                    BinaryOp::NotEqual => self.bytecode.emit(Instruction::NotEqual),

                    BinaryOp::Greater => self.bytecode.emit(Instruction::Greater),
                    BinaryOp::GreaterEqual => self.bytecode.emit(Instruction::GreaterEqual),
                    BinaryOp::Less => self.bytecode.emit(Instruction::Less),
                    BinaryOp::LessEqual => self.bytecode.emit(Instruction::LessEqual),
                }
            }
            ExprKind::Unary { right, operator } => {
                self.visit_expression(right)?;

                match operator {
                    UnaryOp::Negate => self.bytecode.emit(Instruction::Negate),
                    UnaryOp::Not => self.bytecode.emit(Instruction::Not),
                }
            }
            ExprKind::Identifier { resolution, .. } => {
                if resolution.global {
                    let instruction = Instruction::LoadGlobal(resolution.offset);

                    self.bytecode.emit(instruction);
                } else {
                    let instruction = Instruction::LoadLocal(resolution.offset);

                    self.bytecode.emit(instruction);
                }
            }
            ExprKind::NumberLiteral(value) => {
                let instruction = Instruction::PushNumber(*value);
                self.bytecode.emit(instruction);
            }
            ExprKind::BooleanLiteral(value) => {
                let instruction = Instruction::PushBool(*value);
                self.bytecode.emit(instruction);
            }
            ExprKind::StringLiteral(value) => {
                let instruction = Instruction::PushStr(value.clone());
                self.bytecode.emit(instruction);
            }
        };

        Ok(())
    }
}
