#![allow(clippy::new_without_default)]

use crate::{
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

use super::{
    bytecode::Bytecode,
    const_value::ConstValue,
    instruction::{Instruction, Opcode},
};

pub struct BytecodeGenerator {
    instructions: Vec<Instruction>,
    constant_pool: Vec<ConstValue>,
}

impl BytecodeGenerator {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constant_pool: Vec::new(),
        }
    }

    pub fn generate(&mut self, nodes: &mut [ASTNode]) -> Result<Bytecode, KaoriError> {
        self.emit(Instruction::nullary(Opcode::EnterScope));

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

        self.emit(Instruction::nullary(Opcode::ExitScope));

        let bytecode = Bytecode::new(self.instructions.clone(), &self.constant_pool);

        Ok(bytecode)
    }

    pub fn emit(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn emit_constant(&mut self, other: ConstValue) {
        let mut index = 0;

        while index < self.constant_pool.len() {
            let current = &self.constant_pool[index];

            if current == &other {
                break;
            }

            index += 1;
        }

        if index == self.constant_pool.len() {
            self.constant_pool.push(other);
        }

        self.emit(Instruction::unary(Opcode::LoadConst, index));
    }

    pub fn create_placeholder(&mut self) -> usize {
        let index = self.instructions.len();

        self.instructions
            .push(Instruction::nullary(Opcode::Nothing));

        index
    }

    pub fn update_placeholder(&mut self, index: usize, instruction: Instruction) {
        self.instructions[index] = instruction;
    }
}

impl Visitor<()> for BytecodeGenerator {
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
                self.emit(Instruction::nullary(Opcode::Declare));
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

                self.emit(Instruction::nullary(Opcode::Print));
            }
            StmtKind::Block(declarations) => {
                self.emit(Instruction::nullary(Opcode::EnterScope));

                for declaration in declarations {
                    self.visit_ast_node(declaration)?;
                }

                self.emit(Instruction::nullary(Opcode::ExitScope));
            }
            StmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.visit_expression(condition)?;

                let jump_if_false_placeholder = self.create_placeholder();

                self.visit_statement(then_branch)?;

                let jump_end_placeholder = self.create_placeholder();

                self.update_placeholder(
                    jump_if_false_placeholder,
                    Instruction::unary(Opcode::JumpIfFalse, self.instructions.len()),
                );

                if let Some(branch) = else_branch {
                    self.visit_statement(branch)?;
                }

                self.update_placeholder(
                    jump_end_placeholder,
                    Instruction::unary(Opcode::Jump, self.instructions.len()),
                );
            }
            StmtKind::WhileLoop { condition, block } => {
                let start = self.instructions.len();

                self.visit_expression(condition)?;

                let jump_if_false_placeholder = self.create_placeholder();

                self.visit_statement(block)?;

                self.emit(Instruction::unary(Opcode::Jump, start));

                self.update_placeholder(
                    jump_if_false_placeholder,
                    Instruction::unary(Opcode::JumpIfFalse, self.instructions.len()),
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
                    self.emit(Instruction::unary(Opcode::StoreGlobal, resolution.offset));
                } else {
                    self.emit(Instruction::unary(Opcode::StoreLocal, resolution.offset));
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
                    BinaryOp::Plus => self.emit(Instruction::nullary(Opcode::Plus)),
                    BinaryOp::Minus => self.emit(Instruction::nullary(Opcode::Minus)),
                    BinaryOp::Multiply => self.emit(Instruction::nullary(Opcode::Multiply)),
                    BinaryOp::Divide => self.emit(Instruction::nullary(Opcode::Divide)),
                    BinaryOp::Modulo => self.emit(Instruction::nullary(Opcode::Modulo)),

                    BinaryOp::And => self.emit(Instruction::nullary(Opcode::And)),
                    BinaryOp::Or => self.emit(Instruction::nullary(Opcode::Or)),

                    BinaryOp::Equal => self.emit(Instruction::nullary(Opcode::Equal)),
                    BinaryOp::NotEqual => self.emit(Instruction::nullary(Opcode::NotEqual)),

                    BinaryOp::Greater => self.emit(Instruction::nullary(Opcode::Greater)),
                    BinaryOp::GreaterEqual => self.emit(Instruction::nullary(Opcode::GreaterEqual)),
                    BinaryOp::Less => self.emit(Instruction::nullary(Opcode::Less)),
                    BinaryOp::LessEqual => self.emit(Instruction::nullary(Opcode::LessEqual)),
                }
            }
            ExprKind::Unary { right, operator } => {
                self.visit_expression(right)?;

                match operator {
                    UnaryOp::Negate => self.emit(Instruction::nullary(Opcode::Negate)),
                    UnaryOp::Not => self.emit(Instruction::nullary(Opcode::Not)),
                }
            }
            ExprKind::Identifier { resolution, .. } => {
                if resolution.global {
                    self.emit(Instruction::unary(Opcode::LoadGlobal, resolution.offset));
                } else {
                    self.emit(Instruction::unary(Opcode::LoadLocal, resolution.offset));
                }
            }
            ExprKind::NumberLiteral(value) => self.emit_constant(ConstValue::Number(*value)),
            ExprKind::BooleanLiteral(value) => self.emit_constant(ConstValue::Bool(*value)),
            /* ExprKind::StringLiteral(value) => {
                self.emit_constant(Value::Str(value.to_string()))
            } */
            _ => (),
        };

        Ok(())
    }
}
