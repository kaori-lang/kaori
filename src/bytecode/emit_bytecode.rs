use std::collections::HashMap;

use crate::{
    ast::{binary_op::BinaryOpKind, unary_op::UnaryOpKind},
    error::kaori_error::KaoriError,
    hir::{
        decl::{Decl, DeclKind},
        expr::{Expr, ExprKind},
        node_id::NodeId,
        stmt::{Stmt, StmtKind},
    },
};

use super::{constants::Constants, function::Function, instruction::Instruction, operand::Operand};

pub fn emit_bytecode(declarations: &[Decl]) -> Result<Vec<Function>, KaoriError> {
    let mut node_to_function = HashMap::new();
    let mut counter: usize = 0;

    #[allow(clippy::explicit_counter_loop)]
    for declaration in declarations {
        match &declaration.kind {
            DeclKind::Function { .. } => {
                node_to_function.insert(declaration.id, counter);
            }
        }
        counter += 1;
    }

    let mut functions = Vec::new();

    for declaration in declarations {
        match &declaration.kind {
            DeclKind::Function { .. } => {
                let mut ctx = FunctionContext::new(&node_to_function);
                ctx.visit_declaration(declaration)?;
                let function = Function::new(ctx.instructions, ctx.next_register, ctx.constants.0);
                functions.push(function);
            }
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
    break_jumps: Vec<usize>,
    continue_jumps: Vec<usize>,
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
            break_jumps: Vec::new(),
            continue_jumps: Vec::new(),
            pending_arguments: Vec::new(),
        }
    }

    pub fn allocate_register(&mut self) -> Operand {
        let register = self.next_register;
        self.next_register += 1;
        Operand::Register(register)
    }

    fn force_register(&mut self, operand: Operand) -> Operand {
        match operand {
            Operand::Register(_) => operand,
            Operand::BoolImm(value) => {
                let dest = self.allocate_register();
                self.emit_instruction(Instruction::LoadBool {
                    dest: dest.unwrap_register(),
                    imm: value,
                });
                dest
            }
            Operand::NilImm => {
                let dest = self.allocate_register();
                self.emit_instruction(Instruction::LoadNil {
                    dest: dest.unwrap_register(),
                });
                dest
            }
            Operand::NumberImm(imm) => {
                let dest = self.allocate_register();
                self.emit_instruction(Instruction::LoadNumber {
                    dest: dest.unwrap_register(),
                    imm,
                });
                dest
            }
        }
    }

    fn block_returns(&self, statements: &[Stmt]) -> bool {
        for statement in statements {
            if self.statement_returns(statement) {
                return true;
            }
        }
        false
    }

    fn statement_returns(&self, statement: &Stmt) -> bool {
        match &statement.kind {
            StmtKind::Return(..) => true,
            StmtKind::Block(statements) => self.block_returns(statements),
            StmtKind::Branch {
                then_branch,
                else_branch,
                ..
            } => {
                if let Some(else_branch) = else_branch {
                    self.statement_returns(then_branch) && self.statement_returns(else_branch)
                } else {
                    false
                }
            }
            StmtKind::Break
            | StmtKind::Continue
            | StmtKind::Print(..)
            | StmtKind::Expression(..)
            | StmtKind::Loop { .. } => false,
        }
    }

    fn emit_instruction(&mut self, instruction: Instruction) -> usize {
        let index = self.instructions.len();
        self.instructions.push(instruction);
        index
    }

    fn patch_jump(&mut self, index: usize) {
        let offset = self.instructions.len() as i16 - index as i16;

        match &mut self.instructions[index] {
            Instruction::Jump { offset: o } => *o = offset,
            Instruction::JumpIfTrue { offset: o, .. }
            | Instruction::JumpIfFalse { offset: o, .. } => {
                *o = offset;
            }
            _ => panic!("tried to patch a non-jump instruction at index {index}"),
        }
    }

    fn patch_arguments(&mut self) {
        for index in self.pending_arguments.iter().copied() {
            match &mut self.instructions[index] {
                Instruction::Add { dest, .. }
                | Instruction::AddImm { dest, .. }
                | Instruction::Subtract { dest, .. }
                | Instruction::SubtractImm { dest, .. }
                | Instruction::Multiply { dest, .. }
                | Instruction::MultiplyImm { dest, .. }
                | Instruction::Divide { dest, .. }
                | Instruction::DivideImm { dest, .. }
                | Instruction::Modulo { dest, .. }
                | Instruction::ModuloImm { dest, .. }
                | Instruction::Power { dest, .. }
                | Instruction::PowerImm { dest, .. }
                | Instruction::Equal { dest, .. }
                | Instruction::EqualImm { dest, .. }
                | Instruction::NotEqual { dest, .. }
                | Instruction::NotEqualImm { dest, .. }
                | Instruction::Less { dest, .. }
                | Instruction::LessImm { dest, .. }
                | Instruction::LessEqual { dest, .. }
                | Instruction::LessEqualImm { dest, .. }
                | Instruction::Greater { dest, .. }
                | Instruction::GreaterImm { dest, .. }
                | Instruction::GreaterEqual { dest, .. }
                | Instruction::GreaterEqualImm { dest, .. }
                | Instruction::Not { dest, .. }
                | Instruction::Negate { dest, .. }
                | Instruction::Move { dest, .. }
                | Instruction::LoadConst { dest, .. }
                | Instruction::LoadBool { dest, .. }
                | Instruction::LoadNil { dest }
                | Instruction::LoadNumber { dest, .. }
                | Instruction::CreateDict { dest }
                | Instruction::GetField { dest, .. }
                | Instruction::Call { dest, .. } => {
                    *dest += self.next_register;
                }
                Instruction::SetField { .. }
                | Instruction::Return { .. }
                | Instruction::Jump { .. }
                | Instruction::JumpIfTrue { .. }
                | Instruction::JumpIfFalse { .. }
                | Instruction::Print { .. } => {}
            }
        }
    }

    fn mutate_dest(instr: &mut Instruction, register: u8) {
        match instr {
            Instruction::Add { dest, .. }
            | Instruction::AddImm { dest, .. }
            | Instruction::Subtract { dest, .. }
            | Instruction::SubtractImm { dest, .. }
            | Instruction::Multiply { dest, .. }
            | Instruction::MultiplyImm { dest, .. }
            | Instruction::Divide { dest, .. }
            | Instruction::DivideImm { dest, .. }
            | Instruction::Modulo { dest, .. }
            | Instruction::ModuloImm { dest, .. }
            | Instruction::Power { dest, .. }
            | Instruction::PowerImm { dest, .. }
            | Instruction::Equal { dest, .. }
            | Instruction::EqualImm { dest, .. }
            | Instruction::NotEqual { dest, .. }
            | Instruction::NotEqualImm { dest, .. }
            | Instruction::Less { dest, .. }
            | Instruction::LessImm { dest, .. }
            | Instruction::LessEqual { dest, .. }
            | Instruction::LessEqualImm { dest, .. }
            | Instruction::Greater { dest, .. }
            | Instruction::GreaterImm { dest, .. }
            | Instruction::GreaterEqual { dest, .. }
            | Instruction::GreaterEqualImm { dest, .. }
            | Instruction::Not { dest, .. }
            | Instruction::Negate { dest, .. }
            | Instruction::Move { dest, .. }
            | Instruction::LoadConst { dest, .. }
            | Instruction::LoadBool { dest, .. }
            | Instruction::LoadNil { dest }
            | Instruction::LoadNumber { dest, .. }
            | Instruction::CreateDict { dest }
            | Instruction::GetField { dest, .. }
            | Instruction::Call { dest, .. } => {
                *dest = register;
            }
            Instruction::SetField { .. }
            | Instruction::Return { .. }
            | Instruction::Jump { .. }
            | Instruction::JumpIfTrue { .. }
            | Instruction::JumpIfFalse { .. }
            | Instruction::Print { .. } => {}
        }
    }

    fn emit_move(&mut self, expression: &Expr, dest: Operand) {
        let instructions_size = self.instructions.len();
        let src = self.visit_expression(expression);

        if self.instructions.len() == instructions_size {
            let instruction = match src {
                Operand::Register(src) => Instruction::Move {
                    dest: dest.unwrap_register(),
                    src,
                },
                Operand::NumberImm(imm) => Instruction::LoadNumber {
                    dest: dest.unwrap_register(),
                    imm,
                },
                Operand::BoolImm(imm) => Instruction::LoadBool {
                    dest: dest.unwrap_register(),
                    imm,
                },
                Operand::NilImm => Instruction::LoadNil {
                    dest: dest.unwrap_register(),
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

    fn visit_declaration(&mut self, declaration: &Decl) -> Result<(), KaoriError> {
        match &declaration.kind {
            DeclKind::Function { body, parameters } => {
                for parameter in parameters {
                    self.visit_expression(parameter);
                }

                for statement in body {
                    self.visit_statement(statement)?;
                }

                if !self.block_returns(body) {
                    let src = self.force_register(Operand::NilImm).unwrap_register();

                    self.emit_instruction(Instruction::Return { src });
                }

                self.patch_arguments();
            }
        };

        Ok(())
    }

    fn visit_statement(&mut self, statement: &Stmt) -> Result<(), KaoriError> {
        match &statement.kind {
            StmtKind::Expression(expression) => {
                self.visit_expression(expression);
            }

            StmtKind::Print(expression) => {
                let src = self.visit_expression(expression);
                let src = self.force_register(src);
                self.emit_instruction(Instruction::Print {
                    src: src.unwrap_register(),
                });
            }

            StmtKind::Block(statements) => {
                for stmt in statements {
                    self.visit_statement(stmt)?;
                }
            }

            StmtKind::Branch {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond = self.visit_expression(condition);
                let cond = self.force_register(cond);

                let jump_to_else = self.emit_instruction(Instruction::JumpIfFalse {
                    src: cond.unwrap_register(),
                    offset: 0,
                });

                self.visit_statement(then_branch)?;
                self.patch_jump(jump_to_else);

                if let Some(else_branch) = else_branch {
                    self.visit_statement(else_branch)?;
                }
            }

            StmtKind::Loop {
                init,
                condition,
                block,
                increment,
            } => {
                if let Some(init) = init {
                    self.visit_expression(init);
                }

                let loop_start = self.instructions.len();

                let cond = self.visit_expression(condition);
                let cond = self.force_register(cond);

                let jump_to_end = self.emit_instruction(Instruction::JumpIfFalse {
                    src: cond.unwrap_register(),
                    offset: 0,
                });

                self.visit_statement(block)?;

                if let Some(increment) = increment {
                    self.visit_statement(increment)?;
                }

                let offset = loop_start as i16 - self.instructions.len() as i16;
                self.emit_instruction(Instruction::Jump { offset });
                self.patch_jump(jump_to_end);
            }

            StmtKind::Break => {
                todo!()
            }

            StmtKind::Continue => {
                todo!()
            }

            StmtKind::Return(expr) => {
                let src = if let Some(expr) = expr {
                    self.visit_expression(expr)
                } else {
                    Operand::NilImm
                };
                let src = self.force_register(src);
                self.emit_instruction(Instruction::Return {
                    src: src.unwrap_register(),
                });
            }
        };

        Ok(())
    }

    fn visit_expression(&mut self, expression: &Expr) -> Operand {
        match &expression.kind {
            ExprKind::Parameter(id) => {
                let reg = self.allocate_register();
                self.registers.insert(*id, reg);
                reg
            }

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
                let dest = self.force_register(left);

                let jump_to_end = self.emit_instruction(Instruction::JumpIfFalse {
                    src: dest.unwrap_register(),
                    offset: 0,
                });

                self.emit_move(right, dest);
                self.patch_jump(jump_to_end);
                dest
            }

            ExprKind::LogicalOr { left, right } => {
                let left = self.visit_expression(left);
                let dest = self.force_register(left);

                let jump_to_end = self.emit_instruction(Instruction::JumpIfTrue {
                    src: dest.unwrap_register(),
                    offset: 0,
                });

                self.emit_move(right, dest);
                self.patch_jump(jump_to_end);
                dest
            }

            ExprKind::LogicalNot { expr } => {
                let src = self.visit_expression(expr);
                let src = self.force_register(src);
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
                let lhs = self.visit_expression(left);
                let rhs = self.visit_expression(right);
                let dest = self.allocate_register().unwrap_register();

                let instruction = match (lhs, rhs) {
                    (Operand::Register(src1), Operand::Register(src2)) => match operator.kind {
                        BinaryOpKind::Add => Instruction::AddRR { dest, src1, src2 },
                        BinaryOpKind::Subtract => Instruction::SubtractRR { dest, src1, src2 },
                        BinaryOpKind::Multiply => Instruction::MultiplyRR { dest, src1, src2 },
                        BinaryOpKind::Divide => Instruction::DivideRR { dest, src1, src2 },
                        BinaryOpKind::Modulo => Instruction::ModuloRR { dest, src1, src2 },
                        BinaryOpKind::Power => Instruction::PowerRR { dest, src1, src2 },

                        BinaryOpKind::Equal => Instruction::EqualRR { dest, src1, src2 },
                        BinaryOpKind::NotEqual => Instruction::NotEqualRR { dest, src1, src2 },
                        BinaryOpKind::Greater => Instruction::GreaterRR { dest, src1, src2 },
                        BinaryOpKind::GreaterEqual => {
                            Instruction::GreaterEqualRR { dest, src1, src2 }
                        }
                        BinaryOpKind::Less => Instruction::LessRR { dest, src1, src2 },
                        BinaryOpKind::LessEqual => Instruction::LessEqualRR { dest, src1, src2 },
                    },

                    (Operand::Register(src1), Operand::NumberImm(imm)) => match operator.kind {
                        BinaryOpKind::Add => Instruction::AddRI { dest, src1, imm },
                        BinaryOpKind::Subtract => Instruction::SubtractRI { dest, src1, imm },
                        BinaryOpKind::Multiply => Instruction::MultiplyRI { dest, src1, imm },
                        BinaryOpKind::Divide => Instruction::DivideRI { dest, src1, imm },
                        BinaryOpKind::Modulo => Instruction::ModuloRI { dest, src1, imm },
                        BinaryOpKind::Power => Instruction::PowerRI { dest, src1, imm },

                        BinaryOpKind::Equal => Instruction::EqualRI { dest, src1, imm },
                        BinaryOpKind::NotEqual => Instruction::NotEqualRI { dest, src1, imm },
                        BinaryOpKind::Greater => Instruction::GreaterRI { dest, src1, imm },
                        BinaryOpKind::GreaterEqual => {
                            Instruction::GreaterEqualRI { dest, src1, imm }
                        }
                        BinaryOpKind::Less => Instruction::LessRI { dest, src1, imm },
                        BinaryOpKind::LessEqual => Instruction::LessEqualRI { dest, src1, imm },
                    },

                    (Operand::NumberImm(imm), Operand::Register(src2)) => match operator.kind {
                        BinaryOpKind::Add => Instruction::AddIR { dest, imm, src2 },
                        BinaryOpKind::Subtract => Instruction::SubtractIR { dest, imm, src2 },
                        BinaryOpKind::Multiply => Instruction::MultiplyIR { dest, imm, src2 },
                        BinaryOpKind::Divide => Instruction::DivideIR { dest, imm, src2 },
                        BinaryOpKind::Modulo => Instruction::ModuloIR { dest, imm, src2 },
                        BinaryOpKind::Power => Instruction::PowerIR { dest, imm, src2 },

                        BinaryOpKind::Equal => Instruction::EqualIR { dest, imm, src2 },
                        BinaryOpKind::NotEqual => Instruction::NotEqualIR { dest, imm, src2 },
                        BinaryOpKind::Greater => Instruction::GreaterIR { dest, imm, src2 },
                        BinaryOpKind::GreaterEqual => {
                            Instruction::GreaterEqualIR { dest, imm, src2 }
                        }
                        BinaryOpKind::Less => Instruction::LessIR { dest, imm, src2 },
                        BinaryOpKind::LessEqual => Instruction::LessEqualIR { dest, imm, src2 },
                    },

                    _ => panic!("Invalid operand types for binary operator"),
                };

                self.emit_instruction(instruction);
                Operand::Register(dest)
            }

            ExprKind::Unary { right, operator } => {
                let src = self.visit_expression(right);
                let src = self.force_register(src);
                let dest = self.allocate_register().unwrap_register();

                let instruction = match operator.kind {
                    UnaryOpKind::Negate => Instruction::Negate {
                        dest,
                        src: src.unwrap_register(),
                    },
                };

                self.emit_instruction(instruction);
                Operand::Register(dest)
            }

            ExprKind::FunctionCall { callee, arguments } => {
                let dest = self.allocate_register().unwrap_register();
                let callee_src = self.visit_expression(callee);
                let callee_src = self.force_register(callee_src);

                for argument in arguments.iter() {
                    let dest = Operand::Register(self.next_register);
                    self.emit_move(argument, dest);
                    self.pending_arguments.push(self.instructions.len() - 1);
                }

                self.emit_instruction(Instruction::Call {
                    dest,
                    src: callee_src.unwrap_register(),
                });

                Operand::Register(dest)
            }

            ExprKind::MemberAccess { object, property } => {
                let dest = self.allocate_register().unwrap_register();
                let object = self.visit_expression(object);
                let object = self.force_register(object).unwrap_register();
                let key = self.visit_expression(property);
                let key = self.force_register(key).unwrap_register();

                self.emit_instruction(Instruction::GetField { dest, object, key });
                Operand::Register(dest)
            }

            ExprKind::Variable(id) => *self
                .registers
                .get(id)
                .expect("Variable not found for NodeId"),

            ExprKind::Function(id) => {
                let index = *self
                    .node_to_function
                    .get(id)
                    .expect("FunctionRef points to a missing variable node");
                let src = self.constants.push_function_index(index);
                let dest = self.allocate_register();
                self.emit_instruction(Instruction::LoadConst {
                    dest: dest.unwrap_register(),
                    src,
                });
                dest
            }

            ExprKind::String(value) => {
                let src = self.constants.push_string(value.to_owned());
                let dest = self.allocate_register();
                self.emit_instruction(Instruction::LoadConst {
                    dest: dest.unwrap_register(),
                    src,
                });
                dest
            }

            ExprKind::Boolean(value) => Operand::from_bool(*value),
            ExprKind::Number(value) => {
                Operand::from_f64(*value).expect("Number literal out of immediate range")
            }

            ExprKind::DictLiteral { fields } => {
                let dest = self.allocate_register();

                self.emit_instruction(Instruction::CreateDict {
                    dest: dest.unwrap_register(),
                });

                for (key, expr) in fields {
                    let key = self.visit_expression(key);
                    let key = self.force_register(key).unwrap_register();
                    let value = self.visit_expression(expr);
                    let value = self.force_register(value).unwrap_register();

                    self.emit_instruction(Instruction::SetField {
                        object: dest.unwrap_register(),
                        key,
                        value,
                    });
                }

                dest
            }
        }
    }
}
