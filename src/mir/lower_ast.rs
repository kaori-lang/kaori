use std::collections::HashMap;

use crate::{
    mir::{
        function::Function,
        instruction::{Instruction, Register},
    },
    syntax::{
        ast::{Ast, Expr, ExprId},
        ops::{AssignOp, BinaryOp, UnaryOp},
    },
    util::string_interner::StringIndex,
};

pub struct ResolvedAst {
    ast: Ast,
    captures: HashMap<ExprId, Vec<StringIndex>>,
}

impl ResolvedAst {
    pub fn new(ast: Ast, captures: HashMap<ExprId, Vec<StringIndex>>) -> Self {
        Self { ast, captures }
    }

    fn lookup_or_declare(
        names: &mut Vec<(StringIndex, Register)>,
        function: &mut Function,
        name: StringIndex,
    ) -> Register {
        for (found_name, register) in names.iter().copied().rev() {
            if found_name == name {
                return register;
            }
        }

        let register = function.allocate_register();

        names.push((name, register));

        register
    }

    fn touches_register(instruction: Instruction, reg: Register) -> bool {
        match instruction {
            Instruction::Add { dest, src1, src2 }
            | Instruction::Subtract { dest, src1, src2 }
            | Instruction::Multiply { dest, src1, src2 }
            | Instruction::Divide { dest, src1, src2 }
            | Instruction::Modulo { dest, src1, src2 }
            | Instruction::Equal { dest, src1, src2 }
            | Instruction::NotEqual { dest, src1, src2 }
            | Instruction::Less { dest, src1, src2 }
            | Instruction::LessEqual { dest, src1, src2 }
            | Instruction::Greater { dest, src1, src2 }
            | Instruction::GreaterEqual { dest, src1, src2 } => {
                dest == reg || src1 == reg || src2 == reg
            }

            Instruction::SubtractRK { dest, src1, .. }
            | Instruction::DivideRK { dest, src1, .. }
            | Instruction::ModuloRK { dest, src1, .. } => dest == reg || src1 == reg,

            Instruction::DivideKR { dest, src2, .. } | Instruction::ModuloKR { dest, src2, .. } => {
                dest == reg || src2 == reg
            }

            Instruction::Not { dest, src }
            | Instruction::Negate { dest, src }
            | Instruction::Move { dest, src }
            | Instruction::CaptureValue { dest, src } => dest == reg || src == reg,

            Instruction::CreateDict { dest } | Instruction::CreateClosure { dest, .. } => {
                dest == reg
            }

            Instruction::SetField { object, key, value } => {
                object == reg || key == reg || value == reg
            }

            Instruction::GetField { dest, object, key } => {
                dest == reg || object == reg || key == reg
            }

            Instruction::Call { dest, src, .. } => dest == reg || src == reg,

            Instruction::Return { src } => src == reg,

            Instruction::JumpIfFalse { src, .. } | Instruction::JumpIfTrue { src, .. } => {
                src == reg
            }
            Instruction::MoveArg { src, .. } => src == reg,
            _ => false,
        }
    }

    pub fn lower(&self) -> Vec<Function> {
        let entry = self.ast.entry();
        let mut functions = Vec::new();

        let index = functions.len();
        functions.push(None);

        let mut function = Function::new(index, 0);
        let mut names = Vec::new();

        let src = self.lower_expression(&mut functions, &mut function, &mut names, entry);

        if !self.expression_returns(entry) {
            function.emit_instruction(Instruction::Return { src });
        }

        functions[0] = Some(function);

        functions
            .into_iter()
            .map(|function| function.unwrap())
            .collect()
    }

    fn lower_block(
        &self,
        functions: &mut Vec<Option<Function>>,
        function: &mut Function,
        names: &mut Vec<(StringIndex, Register)>,
        expressions: &[ExprId],
    ) -> Register {
        for expression in expressions.iter().copied() {
            let expression = self.ast.get(expression);

            if let Expr::Function { name, .. } = &expression
                && let Some(name) = name
            {
                self.lower_expression(functions, function, names, *name);
            }
        }

        expressions
            .iter()
            .copied()
            .fold(Register(0), |_, expression| {
                self.lower_expression(functions, function, names, expression)
            })
    }

    fn lower_expression(
        &self,
        functions: &mut Vec<Option<Function>>,
        function: &mut Function,
        names: &mut Vec<(StringIndex, Register)>,
        expression: ExprId,
    ) -> Register {
        match *self.ast.get(expression) {
            Expr::NativeFunction { .. } => {
                todo!()
            }
            Expr::Function {
                ref parameters,
                block,
                name,
            } => {
                let index = functions.len();

                functions.push(None);

                let dest = match name {
                    Some(name) => self.lower_expression(functions, function, names, name),
                    None => function.allocate_register(),
                };

                function.emit_instruction(Instruction::CreateClosure {
                    dest,
                    src: index as u32,
                });

                for capture in self.captures.get(&expression).unwrap().iter().copied() {
                    let src = Self::lookup_or_declare(names, function, capture);

                    function.emit_instruction(Instruction::CaptureValue { dest, src });
                }

                let arity = parameters.len();
                let mut function = Function::new(index, arity);
                let mut names = Vec::new();

                for parameter in parameters.iter().copied() {
                    self.lower_expression(functions, &mut function, &mut names, parameter);
                }

                for capture in self.captures.get(&expression).unwrap().iter().copied() {
                    Self::lookup_or_declare(&mut names, &mut function, capture);
                }

                let src = self.lower_expression(functions, &mut function, &mut names, block);

                if !self.expression_returns(block) {
                    function.emit_instruction(Instruction::Return { src });
                }

                functions[index] = Some(function);

                dest
            }
            Expr::DeclareAssign { left, right } => {
                let src = self.lower_expression(functions, function, names, right);
                let dest = self.lower_expression(functions, function, names, left);

                function.emit_instruction(Instruction::Move { dest, src });

                dest
            }
            Expr::Assign { left, right } => {
                let dest = self.lower_expression(functions, function, names, left);
                let src = self.lower_expression(functions, function, names, right);

                function.emit_instruction(Instruction::Move { dest, src });

                dest
            }
            Expr::CompoundAssign {
                operator,
                left,
                right,
            } => {
                let dest = self.lower_expression(functions, function, names, left);
                let src2 = self.lower_expression(functions, function, names, right);

                function.emit_instruction(match operator {
                    AssignOp::AddAssign => Instruction::Add {
                        dest,
                        src1: dest,
                        src2,
                    },
                    AssignOp::SubtractAssign => Instruction::Subtract {
                        dest,
                        src1: dest,
                        src2,
                    },
                    AssignOp::MultiplyAssign => Instruction::Multiply {
                        dest,
                        src1: dest,
                        src2,
                    },
                    AssignOp::DivideAssign => Instruction::Divide {
                        dest,
                        src1: dest,
                        src2,
                    },
                    AssignOp::ModuloAssign => Instruction::Modulo {
                        dest,
                        src1: dest,
                        src2,
                    },
                });

                dest
            }
            Expr::LogicalAnd { left, right } => {
                let src = self.lower_expression(functions, function, names, left);
                let dest = function.allocate_register();

                function.emit_instruction(Instruction::Move { dest, src });

                let jump_if_false = function.emit_instruction(Instruction::JumpIfFalse {
                    src: dest,
                    offset: 0,
                });

                let src = self.lower_expression(functions, function, names, right);

                function.emit_instruction(Instruction::Move { dest, src });

                patch_jump(
                    function,
                    jump_if_false,
                    function.instructions.len() as i32 - jump_if_false as i32,
                );

                dest
            }
            Expr::LogicalOr { left, right } => {
                let src = self.lower_expression(functions, function, names, left);
                let dest = function.allocate_register();

                function.emit_instruction(Instruction::Move { dest, src });

                let jump_if_true = function.emit_instruction(Instruction::JumpIfTrue {
                    src: dest,
                    offset: 0,
                });

                let src = self.lower_expression(functions, function, names, right);

                function.emit_instruction(Instruction::Move { dest, src });

                patch_jump(
                    function,
                    jump_if_true,
                    function.instructions.len() as i32 - jump_if_true as i32,
                );

                dest
            }
            Expr::LogicalNot(expression) => {
                let src = self.lower_expression(functions, function, names, expression);
                let dest = function.allocate_register();

                function.emit_instruction(Instruction::Not { dest, src });

                dest
            }
            Expr::Binary {
                operator,
                left,
                right,
            } => {
                let src1 = self.lower_expression(functions, function, names, left);
                let src2 = self.lower_expression(functions, function, names, right);
                let dest = function.allocate_register();

                let instruction = match operator {
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
                };

                function.emit_instruction(instruction);
                dest
            }
            Expr::Unary { operator, right } => {
                let src = self.lower_expression(functions, function, names, right);
                let dest = function.allocate_register();

                let instruction = match operator {
                    UnaryOp::Negate => Instruction::Negate { dest, src },
                };

                function.emit_instruction(instruction);

                dest
            }
            Expr::FunctionCall {
                callee,
                ref arguments,
            } => {
                let callee_src = self.lower_expression(functions, function, names, callee);

                for (index, argument) in arguments.iter().enumerate() {
                    let dest = Register(index as u16);

                    let argument = self.lower_expression(functions, function, names, *argument);
                    function.emit_instruction(Instruction::MoveArg {
                        dest,
                        src: argument,
                    });
                }

                let dest = function.allocate_register();

                function.emit_instruction(Instruction::Call {
                    dest,
                    src: callee_src,
                    arity: arguments.len() as u8,
                });

                dest
            }
            Expr::MemberAccess { object, property } => {
                let object = self.lower_expression(functions, function, names, object);
                let key = self.lower_expression(functions, function, names, property);
                let dest = function.allocate_register();

                function.emit_instruction(Instruction::GetField { dest, object, key });

                dest
            }
            Expr::Block(ref expressions) => {
                let size = names.len();

                let dest = self.lower_block(functions, function, names, expressions);

                names.truncate(size);

                dest
            }
            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let src = self.lower_expression(functions, function, names, condition);

                let jump_if_false =
                    function.emit_instruction(Instruction::JumpIfFalse { src, offset: 0 });

                let src = self.lower_expression(functions, function, names, then_branch);
                let dest = function.allocate_register();

                function.emit_instruction(Instruction::Move { dest, src });

                let jump_end = function.emit_instruction(Instruction::Jump { offset: 0 });

                patch_jump(
                    function,
                    jump_if_false,
                    function.instructions.len() as i32 - jump_if_false as i32,
                );

                let src = if let Some(else_branch) = else_branch {
                    self.lower_expression(functions, function, names, else_branch)
                } else {
                    function.emit_nil()
                };

                function.emit_instruction(Instruction::Move { dest, src });

                patch_jump(
                    function,
                    jump_end,
                    function.instructions.len() as i32 - jump_end as i32,
                );

                dest
            }
            Expr::ForLoop { .. } => todo!(),
            Expr::WhileLoop { condition, block } => {
                let src = self.lower_expression(functions, function, names, condition);

                let jump_if_false =
                    function.emit_instruction(Instruction::JumpIfFalse { src, offset: 0 });

                let loop_body = function.instructions.len();

                self.lower_expression(functions, function, names, block);

                let src = self.lower_expression(functions, function, names, condition);

                let jump_if_true =
                    function.emit_instruction(Instruction::JumpIfTrue { src, offset: 0 });

                patch_jump(
                    function,
                    jump_if_true,
                    loop_body as i32 - jump_if_true as i32,
                );
                patch_jump(
                    function,
                    jump_if_false,
                    function.instructions.len() as i32 - jump_if_false as i32,
                );

                // if current scope register was used inside loop body, update live range to outlive it
                let instructions_len = function.instructions.len();

                for (_, register) in names.iter().copied() {
                    for index in loop_body..instructions_len {
                        let instruction = function.instructions[index];

                        if Self::touches_register(instruction, register) {
                            function.update_live_range(register, instructions_len);
                            break;
                        }
                    }
                }

                function.emit_nil()
            }
            Expr::Return(expression) => {
                let src = match expression {
                    Some(expr) => self.lower_expression(functions, function, names, expr),
                    None => function.emit_nil(),
                };

                function.emit_instruction(Instruction::Return { src });

                function.emit_nil()
            }
            Expr::Break => todo!(),
            Expr::Continue => todo!(),
            Expr::Identifier(name) => Self::lookup_or_declare(names, function, name),
            Expr::StringLiteral(value) => {
                let src = function.push_string(value);
                let dest = function.allocate_register();

                function.emit_instruction(Instruction::LoadK { dest, src });

                dest
            }
            Expr::NumberLiteral(value) => {
                let src = function.push_number(value);
                let dest = function.allocate_register();

                function.emit_instruction(Instruction::LoadK { dest, src });

                dest
            }
            Expr::BooleanLiteral(value) => {
                let dest = function.allocate_register();

                dest
            }
            Expr::DictLiteral { ref fields } => {
                let dest = function.allocate_register();
                function.emit_instruction(Instruction::CreateDict { dest });

                for (key, value) in fields.iter().copied() {
                    todo!()
                }

                dest
            }
        }
    }

    fn block_returns(&self, expressions: &[ExprId]) -> bool {
        for expression in expressions.iter().copied() {
            if self.expression_returns(expression) {
                return true;
            }
        }

        false
    }

    fn expression_returns(&self, expression: ExprId) -> bool {
        let expression = self.ast.get(expression);

        match *expression {
            Expr::Return(..) => true,
            Expr::Block(ref expressions) => self.block_returns(expressions),
            Expr::If {
                then_branch,
                else_branch: Some(else_branch),
                ..
            } => self.expression_returns(then_branch) && self.expression_returns(else_branch),
            _ => false,
        }
    }
}

fn patch_jump(function: &mut Function, index: usize, new_offset: i32) {
    match &mut function.instructions[index] {
        Instruction::Jump { offset }
        | Instruction::JumpIfTrue { offset, .. }
        | Instruction::JumpIfFalse { offset, .. } => *offset = new_offset,
        _ => panic!("tried to patch a non-jump instruction at index {index}"),
    }
}
