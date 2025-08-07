#![allow(clippy::new_without_default)]

use crate::{
    backend::vm::value::Value,
    error::kaori_error::KaoriError,
    frontend::syntax::{
        ast_node::ASTNode,
        declaration::{Decl, DeclKind},
        expression::{Expr, ExprKind},
        operator::{BinaryOp, UnaryOp},
        statement::{Stmt, StmtKind},
    },
    utils::visitor::Visitor,
};

use super::{constant_pool::ConstantPool, instruction::Instruction};

pub struct BytecodeGenerator<'a> {
    instructions: &'a mut Vec<Instruction>,
    constant_pool: &'a mut ConstantPool,
}

impl<'a> BytecodeGenerator<'a> {
    pub fn new(
        instructions: &'a mut Vec<Instruction>,
        constant_pool: &'a mut ConstantPool,
    ) -> Self {
        Self {
            instructions,
            constant_pool,
        }
    }

    pub fn generate(&mut self, nodes: &mut [ASTNode]) -> Result<(), KaoriError> {
        self.emit(Instruction::EnterScope);

        for i in 0..nodes.len() {
            if let Some(ASTNode::Declaration(decl)) = nodes.get(i)
                && let DeclKind::Function { .. } = &decl.kind
            {
                //self.environment.declare(type_annotation.clone());
            }
        }

        for i in 0..nodes.len() {
            if let Some(node) = nodes.get_mut(i) {
                self.visit_ast_node(node)?;
            }
        }

        self.emit(Instruction::ExitScope);

        Ok(())
    }

    pub fn emit(&mut self, instruction: Instruction) -> usize {
        let index = self.instructions.len();

        self.instructions.push(instruction);

        index
    }

    pub fn emit_constant(&mut self, constant: Value) {
        let index = self.constant_pool.add_constant(constant);

        self.emit(Instruction::LoadConst(index as u16));
    }

    pub fn instruction_ptr(&self) -> u16 {
        self.instructions.len() as u16
    }
}

impl<'a> Visitor<()> for BytecodeGenerator<'a> {
    fn visit_ast_node(&mut self, node: &mut ASTNode) -> Result<(), KaoriError> {
        match node {
            ASTNode::Declaration(declaration) => self.visit_declaration(declaration),
            ASTNode::Statement(statement) => self.visit_statement(statement),
        }
    }

    fn visit_declaration(&mut self, declaration: &mut Decl) -> Result<(), KaoriError> {
        match &mut declaration.kind {
            DeclKind::Variable { right, .. } => {
                self.visit_expression(right)?;
                self.emit(Instruction::Declare);
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

                self.emit(Instruction::Print);
            }
            StmtKind::Block(declarations) => {
                self.emit(Instruction::EnterScope);

                for declaration in declarations {
                    self.visit_ast_node(declaration)?;
                }

                self.emit(Instruction::ExitScope);
            }
            StmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.visit_expression(condition)?;

                let jump_if_false = self.emit(Instruction::Nothing);

                self.visit_statement(then_branch)?;

                let jump_end = self.emit(Instruction::Nothing);

                self.instructions[jump_if_false] =
                    Instruction::JumpIfFalse(self.instruction_ptr() as i16 - jump_if_false as i16);

                if let Some(branch) = else_branch {
                    self.visit_statement(branch)?;
                }

                self.instructions[jump_end] =
                    Instruction::Jump(self.instruction_ptr() as i16 - jump_end as i16);
            }
            StmtKind::WhileLoop { condition, block } => {
                let start = self.instruction_ptr();

                self.visit_expression(condition)?;

                let jump_if_false = self.emit(Instruction::Nothing);

                self.visit_statement(block)?;

                self.emit(Instruction::Jump(
                    start as i16 - self.instruction_ptr() as i16,
                ));

                self.instructions[jump_if_false] =
                    Instruction::JumpIfFalse(self.instruction_ptr() as i16 - jump_if_false as i16);
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
                    self.emit(Instruction::StoreGlobal(resolution.offset as u16));
                } else {
                    self.emit(Instruction::StoreLocal(resolution.offset as u16));
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
                    BinaryOp::Plus => self.emit(Instruction::Plus),
                    BinaryOp::Minus => self.emit(Instruction::Minus),
                    BinaryOp::Multiply => self.emit(Instruction::Multiply),
                    BinaryOp::Divide => self.emit(Instruction::Divide),
                    BinaryOp::Modulo => self.emit(Instruction::Modulo),

                    BinaryOp::And => self.emit(Instruction::And),
                    BinaryOp::Or => self.emit(Instruction::Or),

                    BinaryOp::Equal => self.emit(Instruction::Equal),
                    BinaryOp::NotEqual => self.emit(Instruction::NotEqual),

                    BinaryOp::Greater => self.emit(Instruction::Greater),
                    BinaryOp::GreaterEqual => self.emit(Instruction::GreaterEqual),
                    BinaryOp::Less => self.emit(Instruction::Less),
                    BinaryOp::LessEqual => self.emit(Instruction::LessEqual),
                };
            }
            ExprKind::Unary { right, operator } => {
                self.visit_expression(right)?;

                match operator {
                    UnaryOp::Negate => self.emit(Instruction::Negate),
                    UnaryOp::Not => self.emit(Instruction::Not),
                };
            }
            ExprKind::Identifier { resolution, .. } => {
                if resolution.global {
                    self.emit(Instruction::LoadGlobal(resolution.offset as u16));
                } else {
                    self.emit(Instruction::LoadLocal(resolution.offset as u16));
                }
            }
            ExprKind::NumberLiteral(value) => self.emit_constant(Value::number(*value)),
            ExprKind::BooleanLiteral(value) => self.emit_constant(Value::boolean(*value)),
            ExprKind::StringLiteral(value) => self.emit_constant(Value::Str(value.to_owned())),
            _ => (),
        };

        Ok(())
    }
}
