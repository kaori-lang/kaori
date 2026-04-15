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

use super::{constants::Constants, function::Function, instruction::Instruction};

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
    registers: HashMap<NodeId, u8>,
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

    pub fn allocate_register(&mut self) -> u8 {
        let register = self.next_register;
        self.next_register += 1;
        register
    }

    fn emit_instruction(&mut self, instruction: Instruction) -> usize {
        let index = self.instructions.len();

        self.instructions.push(instruction);

        index
    }

    fn patch_jump(&mut self, index: usize) {
        let offset = self.instructions.len() as i16 - index as i16;

        match &mut self.instructions[index] {
            Instruction::JumpIfFalse { offset: o, .. } => *o = offset,
            Instruction::JumpIfTrue { offset: o, .. } => *o = offset,
            Instruction::Jump { offset: o } => *o = offset,
            _ => panic!("tried to patch a non-jump instruction at index {index}"),
        }
    }

    fn patch_arguments(&mut self) {
        for index in self.pending_arguments.iter().copied() {
            if let Instruction::Move { dest, src } = &mut self.instructions[index] {
                *dest += self.next_register;
            }
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

                let src = self.constants.push_nil();
                let dest = self.allocate_register();

                self.emit_instruction(Instruction::LoadConst { dest, src });
                self.emit_instruction(Instruction::Return { src: dest });

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
                self.emit_instruction(Instruction::Print { src });
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

                let jump_to_else =
                    self.emit_instruction(Instruction::JumpIfFalse { src, offset: 0 });

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

                let src = self.visit_expression(condition);

                let jump_to_end =
                    self.emit_instruction(Instruction::JumpIfFalse { src, offset: 0 });

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
                    let src = self.constants.push_nil();
                    let dest = self.allocate_register();
                    self.emit_instruction(Instruction::LoadConst { dest, src });
                    dest
                };

                self.emit_instruction(Instruction::Return { src });
            }
        };

        Ok(())
    }

    fn visit_expression(&mut self, expression: &Expr) -> u8 {
        match &expression.kind {
            ExprKind::Parameter(id) => {
                let register = self.allocate_register();
                self.registers.insert(*id, register);
                register
            }

            ExprKind::DeclareAssign { id, right } => {
                let dest = self.visit_expression(right);
                self.registers.insert(*id, dest);

                dest
            }

            ExprKind::Assign { left, right } => {
                let dest = self.visit_expression(left);
                let src = self.visit_expression(right);
                self.emit_instruction(Instruction::Move { dest, src });
                dest
            }

            ExprKind::LogicalAnd { left, right } => {
                let dest = self.visit_expression(left);

                let jump_to_end = self.emit_instruction(Instruction::JumpIfFalse {
                    src: dest,
                    offset: 0,
                });

                let src = self.visit_expression(right);
                self.emit_instruction(Instruction::Move { dest, src });

                self.patch_jump(jump_to_end);

                dest
            }

            ExprKind::LogicalOr { left, right } => {
                let dest = self.visit_expression(left);

                let jump_to_end = self.emit_instruction(Instruction::JumpIfTrue {
                    src: dest,
                    offset: 0,
                });

                let src = self.visit_expression(right);
                self.emit_instruction(Instruction::Move { dest, src });

                self.patch_jump(jump_to_end);

                dest
            }

            ExprKind::LogicalNot { expr } => {
                let src = self.visit_expression(expr);
                let dest = self.allocate_register();
                self.emit_instruction(Instruction::Not { dest, src });
                dest
            }

            ExprKind::Binary {
                operator,
                left,
                right,
            } => {
                let src1 = self.visit_expression(left);
                let src2 = self.visit_expression(right);
                let dest = self.allocate_register();

                let instruction = match operator.kind {
                    BinaryOpKind::Add => Instruction::Add { dest, src1, src2 },
                    BinaryOpKind::Subtract => Instruction::Subtract { dest, src1, src2 },
                    BinaryOpKind::Multiply => Instruction::Multiply { dest, src1, src2 },
                    BinaryOpKind::Divide => Instruction::Divide { dest, src1, src2 },
                    BinaryOpKind::Modulo => Instruction::Modulo { dest, src1, src2 },
                    BinaryOpKind::Equal => Instruction::Equal { dest, src1, src2 },
                    BinaryOpKind::NotEqual => Instruction::NotEqual { dest, src1, src2 },
                    BinaryOpKind::Greater => Instruction::Greater { dest, src1, src2 },
                    BinaryOpKind::GreaterEqual => Instruction::GreaterEqual { dest, src1, src2 },
                    BinaryOpKind::Less => Instruction::Less { dest, src1, src2 },
                    BinaryOpKind::LessEqual => Instruction::LessEqual { dest, src1, src2 },
                    BinaryOpKind::Power => Instruction::Power { dest, src1, src2 },
                };

                self.emit_instruction(instruction);
                dest
            }

            ExprKind::Unary { right, operator } => {
                let src = self.visit_expression(right);
                let dest = self.allocate_register();

                let instruction = match operator.kind {
                    UnaryOpKind::Negate => Instruction::Negate { dest, src },
                };

                self.emit_instruction(instruction);
                dest
            }

            ExprKind::FunctionCall { callee, arguments } => {
                let dest = self.allocate_register();

                let arguments_src = arguments
                    .iter()
                    .map(|argument| self.visit_expression(argument))
                    .collect::<Vec<u8>>();

                let src = self.visit_expression(callee);

                for (dest, src) in arguments_src.iter().copied().enumerate() {
                    let argument = self.emit_instruction(Instruction::Move {
                        dest: dest as u8,
                        src,
                    });

                    self.pending_arguments.push(argument);
                }

                self.emit_instruction(Instruction::Call { dest, src });
                dest
            }

            ExprKind::MemberAccess { object, property } => {
                let dest = self.allocate_register();
                let object = self.visit_expression(object);
                let key = self.visit_expression(property);
                self.emit_instruction(Instruction::GetField { dest, object, key });
                dest
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

                self.emit_instruction(Instruction::LoadConst { dest, src });
                dest
            }

            ExprKind::String(value) => {
                let src = self.constants.push_string(value.to_owned());
                let dest = self.allocate_register();
                self.emit_instruction(Instruction::LoadConst { dest, src });
                dest
            }

            ExprKind::Boolean(value) => {
                let src = self.constants.push_boolean(*value);
                let dest = self.allocate_register();
                self.emit_instruction(Instruction::LoadConst { dest, src });
                dest
            }

            ExprKind::Number(value) => {
                let src = self.constants.push_number(*value);
                let dest = self.allocate_register();
                self.emit_instruction(Instruction::LoadConst { dest, src });
                dest
            }

            ExprKind::DictLiteral { fields } => {
                let dest = self.allocate_register();
                self.emit_instruction(Instruction::CreateDict { dest });

                for (key, value) in fields {
                    let key = self.visit_expression(key);
                    let value = self.visit_expression(value);
                    self.emit_instruction(Instruction::SetField {
                        object: dest,
                        key,
                        value,
                    });
                }

                dest
            }
        }
    }
}
