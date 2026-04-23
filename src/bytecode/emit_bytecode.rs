use std::collections::HashMap;

use crate::{
    ast::ops::{BinaryOp, UnaryOp},
    bytecode::immediate::Imm,
    error::kaori_error::KaoriError,
    hir::{
        expr::{Expr, ExprKind},
        node_id::NodeId,
    },
};

use super::{constants::Constants, function::Function, instruction::Instruction, operand::Operand};

pub fn emit_bytecode(expressions: &[Expr]) -> Result<Vec<Function>, KaoriError> {
    let mut node_to_function: HashMap<NodeId, usize> = HashMap::new();

    for (i, expression) in expressions.iter().enumerate() {
        match &expression.kind {
            ExprKind::Function { .. } => {
                node_to_function.insert(expression.id, i);
            }
            _ => unreachable!("Only Function expressions are allowed at the top level"),
        }
    }

    let mut functions = Vec::new();

    for expression in expressions {
        match &expression.kind {
            ExprKind::Function { .. } => {
                let mut ctx = FunctionContext::new(&node_to_function);
                ctx.visit_function(expression)?;
                functions.push(Function::new(
                    ctx.instructions,
                    ctx.next_register,
                    ctx.constants.0,
                ));
            }
            _ => unreachable!("Only Function expressions are allowed at the top level"),
        }
    }

    Ok(functions)
}

pub struct FunctionContext<'a> {
    next_register: u8,
    registers: HashMap<NodeId, Operand>,
    constants: Constants,
    instructions: Vec<Instruction>,
    node_to_function: &'a HashMap<NodeId, usize>,
    pending_arguments: Vec<usize>,
}

impl<'a> FunctionContext<'a> {
    pub fn new(node_to_function: &'a HashMap<NodeId, usize>) -> Self {
        Self {
            registers: HashMap::new(),
            next_register: 0,
            constants: Constants::default(),
            instructions: Vec::new(),
            node_to_function,
            pending_arguments: Vec::new(),
        }
    }

    pub fn allocate_register(&mut self) -> Operand {
        let register = self.next_register;
        self.next_register += 1;
        Operand::Register(register)
    }

    fn materialize(&mut self, src: Operand) -> Operand {
        match src {
            Operand::Register(_) => src,
            Operand::Constant(src) => {
                let dest = self.allocate_register();
                self.emit_instruction(Instruction::LoadK {
                    dest: dest.unwrap_register(),
                    src,
                });
                dest
            }
            Operand::Immediate(src) => {
                let dest = self.allocate_register();
                self.emit_instruction(Instruction::LoadImm {
                    dest: dest.unwrap_register(),
                    src,
                });
                dest
            }
        }
    }

    fn unit() -> Operand {
        Operand::Immediate(Imm::try_to_encode(0.0).unwrap())
    }

    fn block_returns(&self, exprs: &[Expr]) -> bool {
        exprs.iter().any(|e| self.expression_returns(e))
    }

    fn expression_returns(&self, expr: &Expr) -> bool {
        match &expr.kind {
            ExprKind::Return(..) => true,
            ExprKind::Block(exprs) | ExprKind::UncheckedBlock(exprs) => self.block_returns(exprs),
            ExprKind::Branch {
                then_branch,
                else_branch: Some(else_branch),
                ..
            } => self.expression_returns(then_branch) && self.expression_returns(else_branch),

            ExprKind::Branch {
                else_branch: None, ..
            } => false,
            _ => false,
        }
    }

    fn emit_instruction(&mut self, instruction: Instruction) -> usize {
        let index = self.instructions.len();
        self.instructions.push(instruction);

        index
    }

    fn patch_jump(&mut self, index: usize, offset: i32) {
        match &mut self.instructions[index] {
            Instruction::Jump { offset: o }
            | Instruction::JumpIfTrue { offset: o, .. }
            | Instruction::JumpIfFalse { offset: o, .. }
            | Instruction::JumpIfLess { offset: o, .. }
            | Instruction::JumpIfLessI { offset: o, .. }
            | Instruction::JumpIfLessEqual { offset: o, .. }
            | Instruction::JumpIfLessEqualI { offset: o, .. }
            | Instruction::JumpIfGreater { offset: o, .. }
            | Instruction::JumpIfGreaterI { offset: o, .. }
            | Instruction::JumpIfGreaterEqual { offset: o, .. }
            | Instruction::JumpIfGreaterEqualI { offset: o, .. }
            | Instruction::JumpIfEqual { offset: o, .. }
            | Instruction::JumpIfEqualI { offset: o, .. }
            | Instruction::JumpIfNotEqual { offset: o, .. }
            | Instruction::JumpIfNotEqualI { offset: o, .. } => *o = offset,
            _ => panic!("tried to patch a non-jump instruction at index {index}"),
        }
    }

    fn make_jump_if_true(&mut self, src: u8) -> Instruction {
        match self.instructions.last().copied() {
            Some(Instruction::Less { src1, src2, .. }) => {
                self.instructions.pop();
                Instruction::JumpIfLess {
                    src1,
                    src2,
                    offset: 0,
                }
            }
            Some(Instruction::LessI { src1, src2, .. }) => {
                self.instructions.pop();
                Instruction::JumpIfLessI {
                    src1,
                    src2,
                    offset: 0,
                }
            }
            Some(Instruction::LessEqual { src1, src2, .. }) => {
                self.instructions.pop();
                Instruction::JumpIfLessEqual {
                    src1,
                    src2,
                    offset: 0,
                }
            }
            Some(Instruction::LessEqualI { src1, src2, .. }) => {
                self.instructions.pop();
                Instruction::JumpIfLessEqualI {
                    src1,
                    src2,
                    offset: 0,
                }
            }
            Some(Instruction::Greater { src1, src2, .. }) => {
                self.instructions.pop();
                Instruction::JumpIfGreater {
                    src1,
                    src2,
                    offset: 0,
                }
            }
            Some(Instruction::GreaterI { src1, src2, .. }) => {
                self.instructions.pop();
                Instruction::JumpIfGreaterI {
                    src1,
                    src2,
                    offset: 0,
                }
            }
            Some(Instruction::GreaterEqual { src1, src2, .. }) => {
                self.instructions.pop();
                Instruction::JumpIfGreaterEqual {
                    src1,
                    src2,
                    offset: 0,
                }
            }
            Some(Instruction::GreaterEqualI { src1, src2, .. }) => {
                self.instructions.pop();
                Instruction::JumpIfGreaterEqualI {
                    src1,
                    src2,
                    offset: 0,
                }
            }
            Some(Instruction::Equal { src1, src2, .. }) => {
                self.instructions.pop();
                Instruction::JumpIfEqual {
                    src1,
                    src2,
                    offset: 0,
                }
            }
            Some(Instruction::EqualI { src1, src2, .. }) => {
                self.instructions.pop();
                Instruction::JumpIfEqualI {
                    src1,
                    src2,
                    offset: 0,
                }
            }
            Some(Instruction::NotEqual { src1, src2, .. }) => {
                self.instructions.pop();
                Instruction::JumpIfNotEqual {
                    src1,
                    src2,
                    offset: 0,
                }
            }
            Some(Instruction::NotEqualI { src1, src2, .. }) => {
                self.instructions.pop();
                Instruction::JumpIfNotEqualI {
                    src1,
                    src2,
                    offset: 0,
                }
            }
            _ => Instruction::JumpIfTrue { src, offset: 0 },
        }
    }

    fn make_jump_if_false(&mut self, src: u8) -> Instruction {
        match self.instructions.last().copied() {
            Some(Instruction::Less { src1, src2, .. }) => {
                self.instructions.pop();
                Instruction::JumpIfGreaterEqual {
                    src1,
                    src2,
                    offset: 0,
                }
            }
            Some(Instruction::LessI { src1, src2, .. }) => {
                self.instructions.pop();
                Instruction::JumpIfGreaterEqualI {
                    src1,
                    src2,
                    offset: 0,
                }
            }
            Some(Instruction::LessEqual { src1, src2, .. }) => {
                self.instructions.pop();
                Instruction::JumpIfGreater {
                    src1,
                    src2,
                    offset: 0,
                }
            }
            Some(Instruction::LessEqualI { src1, src2, .. }) => {
                self.instructions.pop();
                Instruction::JumpIfGreaterI {
                    src1,
                    src2,
                    offset: 0,
                }
            }
            Some(Instruction::Greater { src1, src2, .. }) => {
                self.instructions.pop();
                Instruction::JumpIfLessEqual {
                    src1,
                    src2,
                    offset: 0,
                }
            }
            Some(Instruction::GreaterI { src1, src2, .. }) => {
                self.instructions.pop();
                Instruction::JumpIfLessEqualI {
                    src1,
                    src2,
                    offset: 0,
                }
            }
            Some(Instruction::GreaterEqual { src1, src2, .. }) => {
                self.instructions.pop();
                Instruction::JumpIfLess {
                    src1,
                    src2,
                    offset: 0,
                }
            }
            Some(Instruction::GreaterEqualI { src1, src2, .. }) => {
                self.instructions.pop();
                Instruction::JumpIfLessI {
                    src1,
                    src2,
                    offset: 0,
                }
            }
            Some(Instruction::Equal { src1, src2, .. }) => {
                self.instructions.pop();
                Instruction::JumpIfNotEqual {
                    src1,
                    src2,
                    offset: 0,
                }
            }
            Some(Instruction::EqualI { src1, src2, .. }) => {
                self.instructions.pop();
                Instruction::JumpIfNotEqualI {
                    src1,
                    src2,
                    offset: 0,
                }
            }
            Some(Instruction::NotEqual { src1, src2, .. }) => {
                self.instructions.pop();
                Instruction::JumpIfEqual {
                    src1,
                    src2,
                    offset: 0,
                }
            }
            Some(Instruction::NotEqualI { src1, src2, .. }) => {
                self.instructions.pop();
                Instruction::JumpIfEqualI {
                    src1,
                    src2,
                    offset: 0,
                }
            }
            _ => Instruction::JumpIfFalse { src, offset: 0 },
        }
    }

    fn patch_arguments(&mut self) {
        for index in self.pending_arguments.iter().copied() {
            match &mut self.instructions[index] {
                Instruction::Add { dest, .. }
                | Instruction::AddI { dest, .. }
                | Instruction::Subtract { dest, .. }
                | Instruction::SubtractRI { dest, .. }
                | Instruction::SubtractIR { dest, .. }
                | Instruction::Multiply { dest, .. }
                | Instruction::MultiplyI { dest, .. }
                | Instruction::Divide { dest, .. }
                | Instruction::DivideRI { dest, .. }
                | Instruction::DivideIR { dest, .. }
                | Instruction::Modulo { dest, .. }
                | Instruction::ModuloRI { dest, .. }
                | Instruction::ModuloIR { dest, .. }
                | Instruction::Equal { dest, .. }
                | Instruction::EqualI { dest, .. }
                | Instruction::NotEqual { dest, .. }
                | Instruction::NotEqualI { dest, .. }
                | Instruction::Less { dest, .. }
                | Instruction::LessI { dest, .. }
                | Instruction::LessEqual { dest, .. }
                | Instruction::LessEqualI { dest, .. }
                | Instruction::Greater { dest, .. }
                | Instruction::GreaterI { dest, .. }
                | Instruction::GreaterEqual { dest, .. }
                | Instruction::GreaterEqualI { dest, .. }
                | Instruction::Not { dest, .. }
                | Instruction::Negate { dest, .. }
                | Instruction::Move { dest, .. }
                | Instruction::LoadK { dest, .. }
                | Instruction::LoadImm { dest, .. }
                | Instruction::CreateDict { dest }
                | Instruction::GetField { dest, .. }
                | Instruction::Call { dest, .. } => {
                    *dest += self.next_register;
                }
                _ => {}
            }
        }
    }

    fn mutate_dest(instr: &mut Instruction, register: u8) {
        match instr {
            Instruction::Add { dest, .. }
            | Instruction::AddI { dest, .. }
            | Instruction::Subtract { dest, .. }
            | Instruction::SubtractRI { dest, .. }
            | Instruction::SubtractIR { dest, .. }
            | Instruction::Multiply { dest, .. }
            | Instruction::MultiplyI { dest, .. }
            | Instruction::Divide { dest, .. }
            | Instruction::DivideRI { dest, .. }
            | Instruction::DivideIR { dest, .. }
            | Instruction::Modulo { dest, .. }
            | Instruction::ModuloRI { dest, .. }
            | Instruction::ModuloIR { dest, .. }
            | Instruction::Equal { dest, .. }
            | Instruction::EqualI { dest, .. }
            | Instruction::NotEqual { dest, .. }
            | Instruction::NotEqualI { dest, .. }
            | Instruction::Less { dest, .. }
            | Instruction::LessI { dest, .. }
            | Instruction::LessEqual { dest, .. }
            | Instruction::LessEqualI { dest, .. }
            | Instruction::Greater { dest, .. }
            | Instruction::GreaterI { dest, .. }
            | Instruction::GreaterEqual { dest, .. }
            | Instruction::GreaterEqualI { dest, .. }
            | Instruction::Not { dest, .. }
            | Instruction::Negate { dest, .. }
            | Instruction::Move { dest, .. }
            | Instruction::LoadK { dest, .. }
            | Instruction::LoadImm { dest, .. }
            | Instruction::CreateDict { dest }
            | Instruction::GetField { dest, .. }
            | Instruction::Call { dest, .. } => {
                *dest = register;
            }
            _ => {}
        }
    }

    fn emit_move(&mut self, expression: &Expr, dest: Operand) {
        let instructions_size = self.instructions.len();
        let src = self.visit_expression(expression);

        if self.instructions.len() == instructions_size {
            let instruction = match src {
                Operand::Constant(src) => Instruction::LoadK {
                    dest: dest.unwrap_register(),
                    src,
                },
                Operand::Register(src) => Instruction::Move {
                    dest: dest.unwrap_register(),
                    src,
                },
                Operand::Immediate(src) => Instruction::LoadImm {
                    dest: dest.unwrap_register(),
                    src,
                },
            };
            self.emit_instruction(instruction);
        } else {
            FunctionContext::mutate_dest(
                self.instructions.last_mut().unwrap(),
                dest.unwrap_register(),
            );
        }
    }

    fn visit_function(&mut self, expr: &Expr) -> Result<(), KaoriError> {
        let ExprKind::Function { parameters, body } = &expr.kind else {
            unreachable!("visit_function called on non-Function expr");
        };

        for (id, _span) in parameters {
            let dest = self.allocate_register();
            self.registers.insert(*id, dest);
        }

        for expr in body {
            self.visit_expression(expr);
        }

        if !self.block_returns(body) {
            let src = self.materialize(Self::unit());
            self.emit_instruction(Instruction::Return {
                src: src.unwrap_register(),
            });
        }

        self.patch_arguments();

        Ok(())
    }

    fn visit_expression(&mut self, expression: &Expr) -> Operand {
        match &expression.kind {
            ExprKind::DeclareAssign { id, right } => {
                let dest = self.allocate_register();
                self.emit_move(right, dest);
                self.registers.insert(*id, dest);

                dest
            }

            ExprKind::Assign { left, right } => {
                let dest = self.visit_expression(left);
                self.emit_move(right, dest);

                dest
            }
            ExprKind::LogicalAnd { left, right } => {
                let left = self.visit_expression(left);
                let dest = self.materialize(left);

                let jump_if_false = self.make_jump_if_false(dest.unwrap_register());
                let jump_if_false = self.emit_instruction(jump_if_false);

                self.emit_move(right, dest);

                self.patch_jump(
                    jump_if_false,
                    self.instructions.len() as i32 - jump_if_false as i32,
                );

                dest
            }
            ExprKind::LogicalOr { left, right } => {
                let left = self.visit_expression(left);
                let dest = self.materialize(left);

                let jump_if_true = self.make_jump_if_true(dest.unwrap_register());
                let jump_if_true = self.emit_instruction(jump_if_true);

                self.emit_move(right, dest);

                self.patch_jump(
                    jump_if_true,
                    self.instructions.len() as i32 - jump_if_true as i32,
                );

                dest
            }
            ExprKind::LogicalNot { expr } => {
                let src = self.visit_expression(expr);
                let src = self.materialize(src);
                let dest = self.allocate_register();

                self.emit_instruction(Instruction::Not {
                    dest: dest.unwrap_register(),
                    src: src.unwrap_register(),
                });

                dest
            }
            ExprKind::Binary {
                operator,
                left,
                right,
            } => {
                let src1 = self.visit_expression(left);
                let src2 = self.visit_expression(right);
                let dest = self.allocate_register().unwrap_register();

                use BinaryOp::*;

                let instruction = match (src1, src2) {
                    (Operand::Register(src1), Operand::Register(src2)) => match operator {
                        Add => Instruction::Add { dest, src1, src2 },
                        Subtract => Instruction::Subtract { dest, src1, src2 },
                        Multiply => Instruction::Multiply { dest, src1, src2 },
                        Divide => Instruction::Divide { dest, src1, src2 },
                        Modulo => Instruction::Modulo { dest, src1, src2 },
                        Equal => Instruction::Equal { dest, src1, src2 },
                        NotEqual => Instruction::NotEqual { dest, src1, src2 },
                        Less => Instruction::Less { dest, src1, src2 },
                        LessEqual => Instruction::LessEqual { dest, src1, src2 },
                        Greater => Instruction::Greater { dest, src1, src2 },
                        GreaterEqual => Instruction::GreaterEqual { dest, src1, src2 },
                    },
                    (Operand::Register(src1), Operand::Immediate(src2)) => match operator {
                        Add => Instruction::AddI { dest, src1, src2 },
                        Subtract => Instruction::SubtractRI { dest, src1, src2 },
                        Multiply => Instruction::MultiplyI { dest, src1, src2 },
                        Divide => Instruction::DivideRI { dest, src1, src2 },
                        Modulo => Instruction::ModuloRI { dest, src1, src2 },
                        Equal => Instruction::EqualI { dest, src1, src2 },
                        NotEqual => Instruction::NotEqualI { dest, src1, src2 },
                        Less => Instruction::LessI { dest, src1, src2 },
                        LessEqual => Instruction::LessEqualI { dest, src1, src2 },
                        Greater => Instruction::GreaterI { dest, src1, src2 },
                        GreaterEqual => Instruction::GreaterEqualI { dest, src1, src2 },
                    },
                    (Operand::Immediate(src1), Operand::Register(src2)) => match operator {
                        Add => Instruction::AddI {
                            dest,
                            src1: src2,
                            src2: src1,
                        },
                        Multiply => Instruction::MultiplyI {
                            dest,
                            src1: src2,
                            src2: src1,
                        },
                        Equal => Instruction::EqualI {
                            dest,
                            src1: src2,
                            src2: src1,
                        },
                        NotEqual => Instruction::NotEqualI {
                            dest,
                            src1: src2,
                            src2: src1,
                        },
                        Subtract => Instruction::SubtractIR { dest, src1, src2 },
                        Divide => Instruction::DivideIR { dest, src1, src2 },
                        Modulo => Instruction::ModuloIR { dest, src1, src2 },
                        Less => Instruction::GreaterI {
                            dest,
                            src1: src2,
                            src2: src1,
                        },
                        LessEqual => Instruction::GreaterEqualI {
                            dest,
                            src1: src2,
                            src2: src1,
                        },
                        Greater => Instruction::LessI {
                            dest,
                            src1: src2,
                            src2: src1,
                        },
                        GreaterEqual => Instruction::LessEqualI {
                            dest,
                            src1: src2,
                            src2: src1,
                        },
                    },
                    (Operand::Constant(src1), Operand::Register(src2)) => {
                        let src1 = self.materialize(Operand::Constant(src1)).unwrap_register();
                        match operator {
                            Add => Instruction::Add { dest, src1, src2 },
                            Subtract => Instruction::Subtract { dest, src1, src2 },
                            Multiply => Instruction::Multiply { dest, src1, src2 },
                            Divide => Instruction::Divide { dest, src1, src2 },
                            Modulo => Instruction::Modulo { dest, src1, src2 },
                            Equal => Instruction::Equal { dest, src1, src2 },
                            NotEqual => Instruction::NotEqual { dest, src1, src2 },
                            Less => Instruction::Less { dest, src1, src2 },
                            LessEqual => Instruction::LessEqual { dest, src1, src2 },
                            Greater => Instruction::Greater { dest, src1, src2 },
                            GreaterEqual => Instruction::GreaterEqual { dest, src1, src2 },
                        }
                    }
                    (Operand::Register(src1), Operand::Constant(src2)) => {
                        let src2 = self.materialize(Operand::Constant(src2)).unwrap_register();
                        match operator {
                            Add => Instruction::Add { dest, src1, src2 },
                            Subtract => Instruction::Subtract { dest, src1, src2 },
                            Multiply => Instruction::Multiply { dest, src1, src2 },
                            Divide => Instruction::Divide { dest, src1, src2 },
                            Modulo => Instruction::Modulo { dest, src1, src2 },
                            Equal => Instruction::Equal { dest, src1, src2 },
                            NotEqual => Instruction::NotEqual { dest, src1, src2 },
                            Less => Instruction::Less { dest, src1, src2 },
                            LessEqual => Instruction::LessEqual { dest, src1, src2 },
                            Greater => Instruction::Greater { dest, src1, src2 },
                            GreaterEqual => Instruction::GreaterEqual { dest, src1, src2 },
                        }
                    }
                    (Operand::Constant(_), Operand::Constant(_))
                    | (Operand::Immediate(_), Operand::Immediate(_))
                    | (Operand::Constant(_), Operand::Immediate(_))
                    | (Operand::Immediate(_), Operand::Constant(_)) => {
                        unreachable!("No constant fold done yet!")
                    }
                };

                self.emit_instruction(instruction);
                Operand::Register(dest)
            }
            ExprKind::Unary { operator, right } => {
                let src = self.visit_expression(right);
                let src = self.materialize(src);
                let dest = self.allocate_register();

                let instruction = match operator {
                    UnaryOp::Negate => Instruction::Negate {
                        dest: dest.unwrap_register(),
                        src: src.unwrap_register(),
                    },
                };

                self.emit_instruction(instruction);

                dest
            }
            ExprKind::FunctionCall { callee, arguments } => {
                let dest = self.allocate_register();

                let callee_src = self.visit_expression(callee);

                for (i, argument) in arguments.iter().enumerate() {
                    let arg_dest = Operand::Register(i as u8);
                    self.emit_move(argument, arg_dest);
                    self.pending_arguments.push(self.instructions.len() - 1);
                }

                let instruction = match callee_src {
                    Operand::Constant(src) => Instruction::CallK {
                        dest: dest.unwrap_register(),
                        src,
                    },
                    Operand::Register(src) => Instruction::Call {
                        dest: dest.unwrap_register(),
                        src,
                    },
                    Operand::Immediate(_) => {
                        let src = self.materialize(callee_src);

                        Instruction::Call {
                            dest: dest.unwrap_register(),
                            src: src.unwrap_register(),
                        }
                    }
                };

                self.emit_instruction(instruction);

                dest
            }
            ExprKind::MemberAccess { object, property } => {
                let dest = self.allocate_register();
                let object = self.visit_expression(object);
                let key = self.visit_expression(property);
                let key = self.materialize(key);

                self.emit_instruction(Instruction::GetField {
                    dest: dest.unwrap_register(),
                    object: object.unwrap_register(),
                    key: key.unwrap_register(),
                });

                dest
            }
            ExprKind::Block(exprs) => {
                let mut last = Self::unit();
                for expr in exprs {
                    last = self.visit_expression(expr);
                }
                last
            }
            ExprKind::UncheckedBlock(exprs) => {
                self.emit_instruction(Instruction::EnterUncheckedBlock);
                let mut last = Self::unit();
                for expr in exprs {
                    last = self.visit_expression(expr);
                }
                self.emit_instruction(Instruction::ExitUncheckedBlock);

                last
            }
            ExprKind::Branch {
                condition,
                then_branch,
                else_branch,
            } => {
                let src = self.visit_expression(condition);
                let src = self.materialize(src);

                let jump_if_false = self.make_jump_if_false(src.unwrap_register());
                let jump_if_false = self.emit_instruction(jump_if_false);

                self.visit_expression(then_branch);

                self.patch_jump(
                    jump_if_false,
                    self.instructions.len() as i32 - jump_if_false as i32,
                );

                if let Some(else_branch) = else_branch {
                    self.visit_expression(else_branch);
                }

                Self::unit()
            }
            ExprKind::Loop {
                init,
                condition,
                block,
                increment,
            } => {
                if let Some(init) = init {
                    self.visit_expression(init);
                }

                let src = self.visit_expression(condition);
                let src = self.materialize(src);

                let jump_if_false = self.make_jump_if_false(src.unwrap_register());
                let jump_if_false = self.emit_instruction(jump_if_false);

                let loop_body = self.instructions.len();

                self.visit_expression(block);

                if let Some(increment) = increment {
                    self.visit_expression(increment);
                }

                let src = self.visit_expression(condition);
                let src = self.materialize(src);

                let jump_if_true = self.make_jump_if_true(src.unwrap_register());
                let jump_if_true = self.emit_instruction(jump_if_true);

                self.patch_jump(jump_if_true, loop_body as i32 - jump_if_true as i32);

                self.patch_jump(
                    jump_if_false,
                    self.instructions.len() as i32 - jump_if_false as i32,
                );

                Self::unit()
            }

            ExprKind::Return(expr) => {
                let src = match expr {
                    Some(expr) => self.visit_expression(expr),
                    None => Self::unit(),
                };
                let src = self.materialize(src);
                self.emit_instruction(Instruction::Return {
                    src: src.unwrap_register(),
                });
                Self::unit()
            }
            ExprKind::Print(expr) => {
                let src = self.visit_expression(expr);
                let src = self.materialize(src);
                self.emit_instruction(Instruction::Print {
                    src: src.unwrap_register(),
                });

                Self::unit()
            }
            ExprKind::Break => {
                todo!()
            }

            ExprKind::Continue => {
                todo!()
            }

            ExprKind::VariableRef(id) => *self
                .registers
                .get(id)
                .unwrap_or_else(|| panic!("VariableRef points to unknown NodeId {id:?}")),

            ExprKind::FunctionRef(id) => {
                let index = *self
                    .node_to_function
                    .get(id)
                    .unwrap_or_else(|| panic!("FunctionRef points to unknown NodeId {id:?}"));
                self.constants.push_function_index(index)
            }
            ExprKind::String(value) => self.constants.push_string(value.clone()),
            ExprKind::Number(value) => {
                let value = *value;
                if let Some(imm) = Imm::try_to_encode(value) {
                    Operand::Immediate(imm)
                } else {
                    self.constants.push_number(value)
                }
            }
            ExprKind::DictLiteral { fields } => {
                let dest = self.allocate_register();

                self.emit_instruction(Instruction::CreateDict {
                    dest: dest.unwrap_register(),
                });

                for (key, value) in fields {
                    let key_op = self.visit_expression(key);
                    let key_op = self.materialize(key_op);
                    let val_op = self.visit_expression(value);
                    let val_op = self.materialize(val_op);

                    self.emit_instruction(Instruction::SetField {
                        object: dest.unwrap_register(),
                        key: key_op.unwrap_register(),
                        value: val_op.unwrap_register(),
                    });
                }

                dest
            }
            ExprKind::Function { .. } => {
                unreachable!("Nested Function expr must be referenced via FunctionRef")
            }
        }
    }
}
