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
            Instruction::JumpIfTrueR { offset: o, .. }
            | Instruction::JumpIfTrueK { offset: o, .. }
            | Instruction::JumpIfFalseR { offset: o, .. }
            | Instruction::JumpIfFalseK { offset: o, .. } => {
                *o = offset;
            }

            _ => panic!("tried to patch a non-jump instruction at index {index}"),
        }
    }

    fn patch_arguments(&mut self) {
        for index in self.pending_arguments.iter().copied() {
            match &mut self.instructions[index] {
                Instruction::AddRR { dest, .. }
                | Instruction::AddRK { dest, .. }
                | Instruction::AddKR { dest, .. }
                | Instruction::SubtractRR { dest, .. }
                | Instruction::SubtractRK { dest, .. }
                | Instruction::SubtractKR { dest, .. }
                | Instruction::MultiplyRR { dest, .. }
                | Instruction::MultiplyRK { dest, .. }
                | Instruction::MultiplyKR { dest, .. }
                | Instruction::DivideRR { dest, .. }
                | Instruction::DivideRK { dest, .. }
                | Instruction::DivideKR { dest, .. }
                | Instruction::ModuloRR { dest, .. }
                | Instruction::ModuloRK { dest, .. }
                | Instruction::ModuloKR { dest, .. }
                | Instruction::PowerRR { dest, .. }
                | Instruction::PowerRK { dest, .. }
                | Instruction::PowerKR { dest, .. }
                | Instruction::EqualRR { dest, .. }
                | Instruction::EqualRK { dest, .. }
                | Instruction::EqualKR { dest, .. }
                | Instruction::NotEqualRR { dest, .. }
                | Instruction::NotEqualRK { dest, .. }
                | Instruction::NotEqualKR { dest, .. }
                | Instruction::GreaterRR { dest, .. }
                | Instruction::GreaterRK { dest, .. }
                | Instruction::GreaterKR { dest, .. }
                | Instruction::GreaterEqualRR { dest, .. }
                | Instruction::GreaterEqualRK { dest, .. }
                | Instruction::GreaterEqualKR { dest, .. }
                | Instruction::NotK { dest, .. }
                | Instruction::NotR { dest, .. }
                | Instruction::NegateK { dest, .. }
                | Instruction::NegateR { dest, .. }
                | Instruction::MoveR { dest, .. }
                | Instruction::MoveK { dest, .. }
                | Instruction::CreateDict { dest }
                | Instruction::GetFieldR { dest, .. }
                | Instruction::GetFieldK { dest, .. }
                | Instruction::CallK { dest, .. }
                | Instruction::CallR { dest, .. } => {
                    *dest += self.next_register;
                }
                Instruction::SetFieldRR { .. }
                | Instruction::SetFieldRK { .. }
                | Instruction::SetFieldKR { .. }
                | Instruction::SetFieldKK { .. }
                | Instruction::ReturnK { .. }
                | Instruction::ReturnR { .. }
                | Instruction::Jump { .. }
                | Instruction::JumpIfTrueK { .. }
                | Instruction::JumpIfTrueR { .. }
                | Instruction::JumpIfFalseK { .. }
                | Instruction::JumpIfFalseR { .. }
                | Instruction::PrintK { .. }
                | Instruction::PrintR { .. } => {}
            }
        }
    }

    fn mutate_dest(instr: &mut Instruction, register: u8) {
        match instr {
            Instruction::AddRR { dest, .. }
            | Instruction::AddRK { dest, .. }
            | Instruction::AddKR { dest, .. }
            | Instruction::SubtractRR { dest, .. }
            | Instruction::SubtractRK { dest, .. }
            | Instruction::SubtractKR { dest, .. }
            | Instruction::MultiplyRR { dest, .. }
            | Instruction::MultiplyRK { dest, .. }
            | Instruction::MultiplyKR { dest, .. }
            | Instruction::DivideRR { dest, .. }
            | Instruction::DivideRK { dest, .. }
            | Instruction::DivideKR { dest, .. }
            | Instruction::ModuloRR { dest, .. }
            | Instruction::ModuloRK { dest, .. }
            | Instruction::ModuloKR { dest, .. }
            | Instruction::PowerRR { dest, .. }
            | Instruction::PowerRK { dest, .. }
            | Instruction::PowerKR { dest, .. }
            | Instruction::EqualRR { dest, .. }
            | Instruction::EqualRK { dest, .. }
            | Instruction::EqualKR { dest, .. }
            | Instruction::NotEqualRR { dest, .. }
            | Instruction::NotEqualRK { dest, .. }
            | Instruction::NotEqualKR { dest, .. }
            | Instruction::GreaterRR { dest, .. }
            | Instruction::GreaterRK { dest, .. }
            | Instruction::GreaterKR { dest, .. }
            | Instruction::GreaterEqualRR { dest, .. }
            | Instruction::GreaterEqualRK { dest, .. }
            | Instruction::GreaterEqualKR { dest, .. }
            | Instruction::NotK { dest, .. }
            | Instruction::NotR { dest, .. }
            | Instruction::NegateK { dest, .. }
            | Instruction::NegateR { dest, .. }
            | Instruction::MoveR { dest, .. }
            | Instruction::MoveK { dest, .. }
            | Instruction::CreateDict { dest }
            | Instruction::GetFieldR { dest, .. }
            | Instruction::GetFieldK { dest, .. }
            | Instruction::CallK { dest, .. }
            | Instruction::CallR { dest, .. } => {
                *dest = register;
            }
            Instruction::SetFieldRR { .. }
            | Instruction::SetFieldRK { .. }
            | Instruction::SetFieldKR { .. }
            | Instruction::SetFieldKK { .. }
            | Instruction::ReturnK { .. }
            | Instruction::ReturnR { .. }
            | Instruction::Jump { .. }
            | Instruction::JumpIfTrueK { .. }
            | Instruction::JumpIfTrueR { .. }
            | Instruction::JumpIfFalseK { .. }
            | Instruction::JumpIfFalseR { .. }
            | Instruction::PrintK { .. }
            | Instruction::PrintR { .. } => {}
        }
    }

    fn emit_move(&mut self, expression: &Expr, dest: Operand) {
        let instructions_size = self.instructions.len();
        let src = self.visit_expression(expression);

        if self.instructions.len() == instructions_size {
            let instruction = match src {
                Operand::Constant(src) => Instruction::MoveK {
                    dest: dest.unwrap_register(),
                    src,
                },
                Operand::Register(src) => Instruction::MoveR {
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
                    let src = self.constants.push_nil();
                    self.emit_instruction(Instruction::ReturnK {
                        src: src.unwrap_constant(),
                    });
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

                let instruction = match src {
                    Operand::Register(src) => Instruction::PrintR { src },
                    Operand::Constant(src) => Instruction::PrintK { src },
                };

                self.emit_instruction(instruction);
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
                let src = self.visit_expression(condition);

                let jump_to_else = match src {
                    Operand::Register(src) => {
                        self.emit_instruction(Instruction::JumpIfFalseR { src, offset: 0 })
                    }
                    Operand::Constant(src) => {
                        self.emit_instruction(Instruction::JumpIfFalseK { src, offset: 0 })
                    }
                };

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

                let jump_to_end = match cond {
                    Operand::Register(src) => {
                        self.emit_instruction(Instruction::JumpIfFalseR { src, offset: 0 })
                    }
                    Operand::Constant(src) => {
                        self.emit_instruction(Instruction::JumpIfFalseK { src, offset: 0 })
                    }
                };

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
                    self.constants.push_nil()
                };

                let instruction = match src {
                    Operand::Register(src) => Instruction::ReturnR { src },
                    Operand::Constant(src) => Instruction::ReturnK { src },
                };

                self.emit_instruction(instruction);
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

                let dest = match left {
                    Operand::Register(_) => left,
                    Operand::Constant(src) => {
                        let dest = self.allocate_register();
                        self.emit_instruction(Instruction::MoveK {
                            dest: dest.unwrap_register(),
                            src,
                        });
                        dest
                    }
                };

                let jump_to_end = match dest {
                    Operand::Register(src) => {
                        self.emit_instruction(Instruction::JumpIfFalseR { src, offset: 0 })
                    }
                    Operand::Constant(src) => {
                        self.emit_instruction(Instruction::JumpIfFalseK { src, offset: 0 })
                    }
                };

                self.emit_move(right, dest);

                self.patch_jump(jump_to_end);

                dest
            }

            ExprKind::LogicalOr { left, right } => {
                let left = self.visit_expression(left);

                let dest = match left {
                    Operand::Register(_) => left,
                    Operand::Constant(src) => {
                        let dest = self.allocate_register();
                        self.emit_instruction(Instruction::MoveK {
                            dest: dest.unwrap_register(),
                            src,
                        });
                        dest
                    }
                };

                let jump_to_end = match dest {
                    Operand::Register(src) => {
                        self.emit_instruction(Instruction::JumpIfTrueR { src, offset: 0 })
                    }
                    Operand::Constant(src) => {
                        self.emit_instruction(Instruction::JumpIfTrueK { src, offset: 0 })
                    }
                };

                self.emit_move(right, dest);

                self.patch_jump(jump_to_end);

                dest
            }

            ExprKind::LogicalNot { expr } => {
                let src = self.visit_expression(expr);
                let dest = self.allocate_register();

                match src {
                    Operand::Register(src) => {
                        self.emit_instruction(Instruction::NotR {
                            dest: dest.unwrap_register(),
                            src,
                        });
                    }
                    Operand::Constant(src) => {
                        self.emit_instruction(Instruction::NotK {
                            dest: dest.unwrap_register(),
                            src,
                        });
                    }
                }

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

                let instruction = match (src1, src2) {
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
                        BinaryOpKind::Less => Instruction::GreaterRR {
                            dest,
                            src1: src2,
                            src2: src1,
                        },
                        BinaryOpKind::LessEqual => Instruction::GreaterEqualRR {
                            dest,
                            src1: src2,
                            src2: src1,
                        },
                    },
                    (Operand::Register(src1), Operand::Constant(src2)) => match operator.kind {
                        BinaryOpKind::Add => Instruction::AddRK { dest, src1, src2 },
                        BinaryOpKind::Subtract => Instruction::SubtractRK { dest, src1, src2 },
                        BinaryOpKind::Multiply => Instruction::MultiplyRK { dest, src1, src2 },
                        BinaryOpKind::Divide => Instruction::DivideRK { dest, src1, src2 },
                        BinaryOpKind::Modulo => Instruction::ModuloRK { dest, src1, src2 },
                        BinaryOpKind::Power => Instruction::PowerRK { dest, src1, src2 },

                        BinaryOpKind::Equal => Instruction::EqualRK { dest, src1, src2 },
                        BinaryOpKind::NotEqual => Instruction::NotEqualRK { dest, src1, src2 },

                        BinaryOpKind::Greater => Instruction::GreaterRK { dest, src1, src2 },
                        BinaryOpKind::GreaterEqual => {
                            Instruction::GreaterEqualRK { dest, src1, src2 }
                        }
                        BinaryOpKind::Less => Instruction::GreaterKR {
                            dest,
                            src1: src2,
                            src2: src1,
                        },
                        BinaryOpKind::LessEqual => Instruction::GreaterEqualKR {
                            dest,
                            src1: src2,
                            src2: src1,
                        },
                    },
                    (Operand::Constant(src1), Operand::Register(src2)) => match operator.kind {
                        BinaryOpKind::Add => Instruction::AddKR { dest, src1, src2 },
                        BinaryOpKind::Subtract => Instruction::SubtractKR { dest, src1, src2 },
                        BinaryOpKind::Multiply => Instruction::MultiplyKR { dest, src1, src2 },
                        BinaryOpKind::Divide => Instruction::DivideKR { dest, src1, src2 },
                        BinaryOpKind::Modulo => Instruction::ModuloKR { dest, src1, src2 },
                        BinaryOpKind::Power => Instruction::PowerKR { dest, src1, src2 },
                        BinaryOpKind::Equal => Instruction::EqualKR { dest, src1, src2 },
                        BinaryOpKind::NotEqual => Instruction::NotEqualKR { dest, src1, src2 },
                        BinaryOpKind::Greater => Instruction::GreaterKR { dest, src1, src2 },
                        BinaryOpKind::GreaterEqual => {
                            Instruction::GreaterEqualKR { dest, src1, src2 }
                        }
                        BinaryOpKind::Less => Instruction::GreaterRK {
                            dest,
                            src1: src2,
                            src2: src1,
                        },
                        BinaryOpKind::LessEqual => Instruction::GreaterEqualRK {
                            dest,
                            src1: src2,
                            src2: src1,
                        },
                    },

                    (Operand::Constant(_), Operand::Constant(_)) => {
                        unreachable!("No constant fold done yet!")
                    }
                };

                self.emit_instruction(instruction);

                Operand::Register(dest)
            }
            ExprKind::Unary { right, operator } => {
                let src = self.visit_expression(right);
                let dest = self.allocate_register().unwrap_register();

                let instruction = match (operator.kind, src) {
                    (UnaryOpKind::Negate, Operand::Register(src)) => {
                        Instruction::NegateR { dest, src }
                    }

                    (UnaryOpKind::Negate, Operand::Constant(src)) => {
                        Instruction::NegateK { dest, src }
                    }
                };

                self.emit_instruction(instruction);

                Operand::Register(dest)
            }

            ExprKind::FunctionCall { callee, arguments } => {
                let dest = self.allocate_register().unwrap_register();

                let callee_src = self.visit_expression(callee);

                for (index, argument) in arguments.iter().enumerate() {
                    let dest = Operand::Register(index as u8);
                    self.emit_move(argument, dest);

                    self.pending_arguments.push(self.instructions.len() - 1);
                }

                let instruction = match callee_src {
                    Operand::Register(src) => Instruction::CallR { dest, src },
                    Operand::Constant(src) => Instruction::CallK { dest, src },
                };

                self.emit_instruction(instruction);

                Operand::Register(dest)
            }
            ExprKind::MemberAccess { object, property } => {
                let dest = self.allocate_register().unwrap_register();

                let object = self.visit_expression(object).unwrap_register();
                let key = self.visit_expression(property);

                let instruction = match key {
                    Operand::Register(key) => Instruction::GetFieldR { dest, object, key },

                    Operand::Constant(key) => Instruction::GetFieldK { dest, object, key },
                };

                self.emit_instruction(instruction);

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

                self.constants.push_function_index(index)
            }
            ExprKind::String(value) => self.constants.push_string(value.to_owned()),
            ExprKind::Boolean(value) => self.constants.push_boolean(*value),
            ExprKind::Number(value) => self.constants.push_number(*value),
            ExprKind::DictLiteral { fields } => {
                let dest = self.allocate_register();

                self.emit_instruction(Instruction::CreateDict {
                    dest: dest.unwrap_register(),
                });

                for (key, expr) in fields {
                    let key = self.visit_expression(key);
                    let value = self.visit_expression(expr);

                    let instruction = match (key, value) {
                        (Operand::Register(key), Operand::Register(value)) => {
                            Instruction::SetFieldRR {
                                object: dest.unwrap_register(),
                                key,
                                value,
                            }
                        }

                        (Operand::Register(key), Operand::Constant(value)) => {
                            Instruction::SetFieldRK {
                                object: dest.unwrap_register(),
                                key,
                                value,
                            }
                        }

                        (Operand::Constant(key), Operand::Register(value)) => {
                            Instruction::SetFieldKR {
                                object: dest.unwrap_register(),
                                key,
                                value,
                            }
                        }

                        (Operand::Constant(key), Operand::Constant(value)) => {
                            Instruction::SetFieldKK {
                                object: dest.unwrap_register(),
                                key,
                                value,
                            }
                        }
                    };

                    self.emit_instruction(instruction);
                }

                dest
            }
        }
    }
}
