use std::collections::HashMap;

use crate::{
    ast::{
        expr::{Expr, ExprKind},
        ops::{AssignOp, BinaryOp, UnaryOp},
    },
    bytecode::{
        constants::Constants, function::Function, immediate::Imm, instruction::Instruction,
        operand::Operand, register_allocator::RegisterAllocator,
    },
    error::kaori_error::KaoriError,
};

pub fn emit_bytecode_from_ast(expressions: &[Expr]) -> Result<Vec<Function>, KaoriError> {
    let mut global_functions: HashMap<String, usize> = HashMap::new();

    for (index, expression) in expressions.iter().enumerate() {
        match &expression.kind {
            ExprKind::Function { name, .. } => {
                global_functions.insert(name.to_owned(), index);
            }
            _ => unreachable!("Only Function expressions are allowed at the top level"),
        }
    }

    let mut functions = Vec::new();

    for expression in expressions {
        match &expression.kind {
            ExprKind::Function { .. } => {
                let mut ctx = FunctionContext::new(&global_functions);

                ctx.visit_function(expression)?;
                functions.push(Function::new(ctx.instructions, 0, ctx.constants.0));
            }
            _ => unreachable!("Only Function expressions are allowed at the top level"),
        }
    }

    Ok(functions)
}

pub struct FunctionContext<'a> {
    symbols: Vec<HashMap<String, u8>>,
    registers_allocator: RegisterAllocator,
    constants: Constants,
    instructions: Vec<Instruction>,
    global_functions: &'a HashMap<String, usize>,
}

impl<'a> FunctionContext<'a> {
    pub fn new(global_functions: &'a HashMap<String, usize>) -> Self {
        Self {
            symbols: Vec::new(),
            registers_allocator: RegisterAllocator::new(),
            constants: Constants::default(),
            instructions: Vec::new(),
            global_functions,
        }
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

    fn block_returns(&self, expressions: &[Expr]) -> bool {
        expressions
            .iter()
            .any(|expression| self.expression_returns(expression))
    }

    fn expression_returns(&self, expression: &Expr) -> bool {
        match &expression.kind {
            ExprKind::Return(..) => true,
            ExprKind::Block(expressions) | ExprKind::UncheckedBlock(expressions) => {
                self.block_returns(expressions)
            }
            ExprKind::If {
                then_branch,
                else_branch: Some(else_branch),
                ..
            } => self.expression_returns(then_branch) && self.expression_returns(else_branch),

            ExprKind::If {
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

    fn patch_jump(&mut self, index: usize, new_offset: i32) {
        match &mut self.instructions[index] {
            Instruction::Jump { offset }
            | Instruction::JumpIfTrue { offset, .. }
            | Instruction::JumpIfFalse { offset, .. } => *offset = new_offset,
            _ => panic!("tried to patch a non-jump instruction at index {index}"),
        }
    }

    fn patch_function_arguments(&mut self) {
        for instruction in self.instructions.iter_mut() {
            if let Instruction::MoveArg { dest, .. } = instruction {
                *dest += self.next_register;
            }
        }
    }

    fn visit_function(&mut self, expression: &Expr) -> Result<(), KaoriError> {
        let ExprKind::Function {
            parameters, body, ..
        } = &expression.kind
        else {
            unreachable!("visit_function called on non-Function expression");
        };

        for (name, _span) in parameters {
            let dest = self.allocate_register();
            self.registers.insert(name.clone(), dest);
        }

        for expression in body {
            self.visit_expression(expression);
        }

        if !self.block_returns(body) {
            let src = self.materialize(Self::unit());
            self.emit_instruction(Instruction::Return {
                src: src.unwrap_register(),
            });
        }

        self.patch_function_arguments();

        Ok(())
    }

    fn visit_expression(&mut self, expression: &Expr) -> Operand {
        match &expression.kind {
            ExprKind::DeclareAssign { left, right } => {
                let name = match &left.kind {
                    ExprKind::Identifier(name) => name.clone(),
                    _ => panic!("DeclareAssign left-hand side must be an Identifier"),
                };

                let dest = self.allocate_register();
                let src = self.visit_expression(right);
                let src = self.materialize(src);

                self.registers.insert(name, dest);

                self.emit_instruction(Instruction::Move {
                    dest: dest.unwrap_register(),
                    src: src.unwrap_register(),
                });

                dest
            }
            ExprKind::Assign {
                operator,
                left,
                right,
            } => {
                let dest = self.visit_expression(left);

                let src = match operator {
                    AssignOp::Assign => self.visit_expression(right),
                    AssignOp::AddAssign => self.visit_binary_op(BinaryOp::Add, left, right),
                    AssignOp::SubtractAssign => {
                        self.visit_binary_op(BinaryOp::Subtract, left, right)
                    }
                    AssignOp::MultiplyAssign => {
                        self.visit_binary_op(BinaryOp::Multiply, left, right)
                    }
                    AssignOp::DivideAssign => self.visit_binary_op(BinaryOp::Divide, left, right),
                    AssignOp::ModuloAssign => self.visit_binary_op(BinaryOp::Modulo, left, right),
                };
                let src = self.materialize(src);

                self.emit_instruction(Instruction::Move {
                    dest: dest.unwrap_register(),
                    src: src.unwrap_register(),
                });

                dest
            }
            ExprKind::LogicalAnd { left, right } => {
                let dest = self.allocate_register();

                let src = self.visit_expression(left);
                let src = self.materialize(src);

                self.emit_instruction(Instruction::Move {
                    dest: dest.unwrap_register(),
                    src: src.unwrap_register(),
                });

                let jump_if_false = self.emit_instruction(Instruction::JumpIfFalse {
                    src: dest.unwrap_register(),
                    offset: 0,
                });

                let src = self.visit_expression(right);
                let src = self.materialize(src);

                self.emit_instruction(Instruction::Move {
                    dest: dest.unwrap_register(),
                    src: src.unwrap_register(),
                });

                self.patch_jump(
                    jump_if_false,
                    self.instructions.len() as i32 - jump_if_false as i32,
                );

                dest
            }
            ExprKind::LogicalOr { left, right } => {
                let dest = self.allocate_register();

                let src = self.visit_expression(left);
                let src = self.materialize(src);

                self.emit_instruction(Instruction::Move {
                    dest: dest.unwrap_register(),
                    src: src.unwrap_register(),
                });

                let jump_if_true = self.emit_instruction(Instruction::JumpIfTrue {
                    src: dest.unwrap_register(),
                    offset: 0,
                });

                let src = self.visit_expression(right);
                let src = self.materialize(src);

                self.emit_instruction(Instruction::Move {
                    dest: dest.unwrap_register(),
                    src: src.unwrap_register(),
                });

                self.patch_jump(
                    jump_if_true,
                    self.instructions.len() as i32 - jump_if_true as i32,
                );

                dest
            }
            ExprKind::LogicalNot(expression) => {
                let dest = self.allocate_register();
                let src = self.visit_expression(expression);
                let src = self.materialize(src);

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
            } => self.visit_binary_op(*operator, left, right),
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

                for (index, argument) in arguments.iter().enumerate() {
                    let argument = self.visit_expression(argument);
                    let argument = self.materialize(argument);

                    self.emit_instruction(Instruction::MoveArg {
                        dest: index as u8,
                        src: argument.unwrap_register(),
                    });
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
                    _ => panic!("Function should be either coming from a constant or a register"),
                };

                self.emit_instruction(instruction);

                dest
            }
            ExprKind::MemberAccess { object, property } => {
                let dest = self.allocate_register();

                let object = self.visit_expression(object);
                let object = self.materialize(object);
                let key = self.visit_expression(property);
                let key = self.materialize(key);

                self.emit_instruction(Instruction::GetField {
                    dest: dest.unwrap_register(),
                    object: object.unwrap_register(),
                    key: key.unwrap_register(),
                });

                dest
            }
            ExprKind::Block(expressions) => {
                let mut last = Self::unit();

                for expression in expressions {
                    last = self.visit_expression(expression);
                }

                last
            }
            ExprKind::UncheckedBlock(expressions) => {
                self.emit_instruction(Instruction::EnterUncheckedBlock);
                let mut last = Self::unit();
                for expression in expressions {
                    last = self.visit_expression(expression);
                }
                self.emit_instruction(Instruction::ExitUncheckedBlock);

                last
            }
            ExprKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let dest = self.allocate_register();

                let src = self.visit_expression(condition);
                let src = self.materialize(src);

                let jump_if_false = self.emit_instruction(Instruction::JumpIfFalse {
                    src: src.unwrap_register(),
                    offset: 0,
                });

                let src = self.visit_expression(then_branch);
                let src = self.materialize(src);

                self.emit_instruction(Instruction::Move {
                    dest: dest.unwrap_register(),
                    src: src.unwrap_register(),
                });

                let jump_end = self.emit_instruction(Instruction::Jump { offset: 0 });

                self.patch_jump(
                    jump_if_false,
                    self.instructions.len() as i32 - jump_if_false as i32,
                );

                let src = if let Some(else_branch) = else_branch {
                    self.visit_expression(else_branch)
                } else {
                    Self::unit()
                };
                let src = self.materialize(src);

                self.emit_instruction(Instruction::Move {
                    dest: dest.unwrap_register(),
                    src: src.unwrap_register(),
                });

                self.patch_jump(jump_end, self.instructions.len() as i32 - jump_end as i32);

                dest
            }
            ExprKind::ForLoop {
                init,
                condition,
                block,
                increment,
            } => self.visit_loop(Some(init), condition, block, Some(increment)),
            ExprKind::WhileLoop { condition, block } => {
                self.visit_loop(None, condition, block, None)
            }
            ExprKind::Return(expression) => {
                let src = match expression {
                    Some(expression) => self.visit_expression(expression),
                    None => Self::unit(),
                };
                let src = self.materialize(src);
                self.emit_instruction(Instruction::Return {
                    src: src.unwrap_register(),
                });

                Self::unit()
            }
            ExprKind::Print(expression) => {
                let src = self.visit_expression(expression);
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
            ExprKind::Identifier(name) => {
                if let Some(&operand) = self.registers.get(name) {
                    operand
                } else if let Some(&index) = self.global_functions.get(name) {
                    self.constants.push_function_index(index)
                } else {
                    panic!("Unknown identifier: {name}")
                }
            }
            ExprKind::BooleanLiteral(value) => {
                let numeric = if *value { 1.0 } else { 0.0 };

                Operand::Immediate(Imm::try_to_encode(numeric).unwrap())
            }
            ExprKind::StringLiteral(value) => self.constants.push_string(value.clone()),
            ExprKind::NumberLiteral(value) => {
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

                    let value_op = match value {
                        Some(v) => {
                            let v = self.visit_expression(v);
                            self.materialize(v)
                        }
                        None => {
                            let v = self.visit_expression(key);
                            self.materialize(v)
                        }
                    };

                    self.emit_instruction(Instruction::SetField {
                        object: dest.unwrap_register(),
                        key: key_op.unwrap_register(),
                        value: value_op.unwrap_register(),
                    });
                }

                dest
            }
            ExprKind::Function { .. } => {
                unreachable!("Nested Function expression must be referenced via Identifier")
            }
        }
    }

    fn visit_binary_op(&mut self, operator: BinaryOp, left: &Expr, right: &Expr) -> Operand {
        let src1 = self.visit_expression(left);
        let src2 = self.visit_expression(right);
        let dest = self.allocate_register().unwrap_register();

        let instruction = match (src1, src2) {
            (Operand::Register(src1), Operand::Register(src2)) => match operator {
                BinaryOp::Add => Instruction::Add { dest, src1, src2 },
                BinaryOp::Subtract => Instruction::Subtract { dest, src1, src2 },
                BinaryOp::Multiply => Instruction::Multiply { dest, src1, src2 },
                BinaryOp::Divide => Instruction::Divide { dest, src1, src2 },
                BinaryOp::Modulo => Instruction::Modulo { dest, src1, src2 },
                BinaryOp::Equal => Instruction::Equal { dest, src1, src2 },
                BinaryOp::NotEqual => Instruction::NotEqual { dest, src1, src2 },
                BinaryOp::Less => Instruction::Less { dest, src1, src2 },
                BinaryOp::LessEqual => Instruction::LessEqual { dest, src1, src2 },
                BinaryOp::Greater => Instruction::Greater { dest, src1, src2 },
                BinaryOp::GreaterEqual => Instruction::GreaterEqual { dest, src1, src2 },
            },
            (Operand::Register(src1), Operand::Immediate(src2)) => match operator {
                BinaryOp::Add => Instruction::AddI { dest, src1, src2 },
                BinaryOp::Subtract => Instruction::SubtractRI { dest, src1, src2 },
                BinaryOp::Multiply => Instruction::MultiplyI { dest, src1, src2 },
                BinaryOp::Divide => Instruction::DivideRI { dest, src1, src2 },
                BinaryOp::Modulo => Instruction::ModuloRI { dest, src1, src2 },
                BinaryOp::Equal => Instruction::EqualI { dest, src1, src2 },
                BinaryOp::NotEqual => Instruction::NotEqualI { dest, src1, src2 },
                BinaryOp::Less => Instruction::LessI { dest, src1, src2 },
                BinaryOp::LessEqual => Instruction::LessEqualI { dest, src1, src2 },
                BinaryOp::Greater => Instruction::GreaterI { dest, src1, src2 },
                BinaryOp::GreaterEqual => Instruction::GreaterEqualI { dest, src1, src2 },
            },
            (Operand::Immediate(src1), Operand::Register(src2)) => match operator {
                BinaryOp::Add => Instruction::AddI {
                    dest,
                    src1: src2,
                    src2: src1,
                },
                BinaryOp::Multiply => Instruction::MultiplyI {
                    dest,
                    src1: src2,
                    src2: src1,
                },
                BinaryOp::Equal => Instruction::EqualI {
                    dest,
                    src1: src2,
                    src2: src1,
                },
                BinaryOp::NotEqual => Instruction::NotEqualI {
                    dest,
                    src1: src2,
                    src2: src1,
                },
                BinaryOp::Subtract => Instruction::SubtractIR { dest, src1, src2 },
                BinaryOp::Divide => Instruction::DivideIR { dest, src1, src2 },
                BinaryOp::Modulo => Instruction::ModuloIR { dest, src1, src2 },
                BinaryOp::Less => Instruction::GreaterI {
                    dest,
                    src1: src2,
                    src2: src1,
                },
                BinaryOp::LessEqual => Instruction::GreaterEqualI {
                    dest,
                    src1: src2,
                    src2: src1,
                },
                BinaryOp::Greater => Instruction::LessI {
                    dest,
                    src1: src2,
                    src2: src1,
                },
                BinaryOp::GreaterEqual => Instruction::LessEqualI {
                    dest,
                    src1: src2,
                    src2: src1,
                },
            },
            (Operand::Constant(src1), Operand::Register(src2)) => {
                let src1 = self.materialize(Operand::Constant(src1)).unwrap_register();
                match operator {
                    BinaryOp::Add => Instruction::Add { dest, src1, src2 },
                    BinaryOp::Subtract => Instruction::Subtract { dest, src1, src2 },
                    BinaryOp::Multiply => Instruction::Multiply { dest, src1, src2 },
                    BinaryOp::Divide => Instruction::Divide { dest, src1, src2 },
                    BinaryOp::Modulo => Instruction::Modulo { dest, src1, src2 },
                    BinaryOp::Equal => Instruction::Equal { dest, src1, src2 },
                    BinaryOp::NotEqual => Instruction::NotEqual { dest, src1, src2 },
                    BinaryOp::Less => Instruction::Less { dest, src1, src2 },
                    BinaryOp::LessEqual => Instruction::LessEqual { dest, src1, src2 },
                    BinaryOp::Greater => Instruction::Greater { dest, src1, src2 },
                    BinaryOp::GreaterEqual => Instruction::GreaterEqual { dest, src1, src2 },
                }
            }
            (Operand::Register(src1), Operand::Constant(src2)) => {
                let src2 = self.materialize(Operand::Constant(src2)).unwrap_register();
                match operator {
                    BinaryOp::Add => Instruction::Add { dest, src1, src2 },
                    BinaryOp::Subtract => Instruction::Subtract { dest, src1, src2 },
                    BinaryOp::Multiply => Instruction::Multiply { dest, src1, src2 },
                    BinaryOp::Divide => Instruction::Divide { dest, src1, src2 },
                    BinaryOp::Modulo => Instruction::Modulo { dest, src1, src2 },
                    BinaryOp::Equal => Instruction::Equal { dest, src1, src2 },
                    BinaryOp::NotEqual => Instruction::NotEqual { dest, src1, src2 },
                    BinaryOp::Less => Instruction::Less { dest, src1, src2 },
                    BinaryOp::LessEqual => Instruction::LessEqual { dest, src1, src2 },
                    BinaryOp::Greater => Instruction::Greater { dest, src1, src2 },
                    BinaryOp::GreaterEqual => Instruction::GreaterEqual { dest, src1, src2 },
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

    fn visit_loop(
        &mut self,
        init: Option<&Expr>,
        condition: &Expr,
        block: &Expr,
        increment: Option<&Expr>,
    ) -> Operand {
        if let Some(init) = init {
            self.visit_expression(init);
        }

        let src = self.visit_expression(condition);
        let src = self.materialize(src);

        let jump_if_false = self.emit_instruction(Instruction::JumpIfFalse {
            src: src.unwrap_register(),
            offset: 0,
        });

        let loop_body = self.instructions.len();

        self.visit_expression(block);

        if let Some(increment) = increment {
            self.visit_expression(increment);
        }

        let src = self.visit_expression(condition);
        let src = self.materialize(src);

        let jump_if_true = self.emit_instruction(Instruction::JumpIfTrue {
            src: src.unwrap_register(),
            offset: 0,
        });

        self.patch_jump(jump_if_true, loop_body as i32 - jump_if_true as i32);

        self.patch_jump(
            jump_if_false,
            self.instructions.len() as i32 - jump_if_false as i32,
        );

        Self::unit()
    }
}
