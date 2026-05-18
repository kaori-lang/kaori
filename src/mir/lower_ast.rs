use std::collections::HashMap;

use crate::{
    mir::{
        function::Function,
        instruction::{ConstIndex, Instruction, Register},
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

    fn touches_register(instruction: Instruction, register: Register) -> bool {
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
                dest == register || src1 == register || src2 == register
            }
            Instruction::AddK { dest, src1, .. }
            | Instruction::SubtractRK { dest, src1, .. }
            | Instruction::MultiplyK { dest, src1, .. }
            | Instruction::DivideRK { dest, src1, .. }
            | Instruction::ModuloRK { dest, src1, .. }
            | Instruction::EqualK { dest, src1, .. }
            | Instruction::NotEqualK { dest, src1, .. }
            | Instruction::LessK { dest, src1, .. }
            | Instruction::LessEqualK { dest, src1, .. }
            | Instruction::GreaterK { dest, src1, .. }
            | Instruction::GreaterEqualK { dest, src1, .. } => dest == register || src1 == register,
            Instruction::SubtractKR { dest, src2, .. }
            | Instruction::DivideKR { dest, src2, .. }
            | Instruction::ModuloKR { dest, src2, .. } => dest == register || src2 == register,
            Instruction::Not { dest, src }
            | Instruction::Negate { dest, src }
            | Instruction::Move { dest, src }
            | Instruction::CaptureValue { dest, src } => dest == register || src == register,
            Instruction::LoadK { dest, .. }
            | Instruction::CreateDict { dest }
            | Instruction::CreateClosure { dest, .. } => dest == register,
            Instruction::SetField { object, key, value } => {
                object == register || key == register || value == register
            }
            Instruction::GetField { dest, object, key } => {
                dest == register || object == register || key == register
            }
            Instruction::Call { dest, src, .. } => dest == register || src == register,
            Instruction::Return { src } => src == register,
            Instruction::JumpIfFalse { src, .. } | Instruction::JumpIfTrue { src, .. } => {
                src == register
            }
            Instruction::JumpIfLess { src1, src2, .. }
            | Instruction::JumpIfLessEqual { src1, src2, .. }
            | Instruction::JumpIfGreater { src1, src2, .. }
            | Instruction::JumpIfGreaterEqual { src1, src2, .. }
            | Instruction::JumpIfEqual { src1, src2, .. }
            | Instruction::JumpIfNotEqual { src1, src2, .. } => {
                src1 == register || src2 == register
            }
            Instruction::JumpIfLessK { src1, .. }
            | Instruction::JumpIfLessEqualK { src1, .. }
            | Instruction::JumpIfGreaterK { src1, .. }
            | Instruction::JumpIfGreaterEqualK { src1, .. }
            | Instruction::JumpIfEqualK { src1, .. }
            | Instruction::JumpIfNotEqualK { src1, .. } => src1 == register,
            Instruction::Jump { .. } | Instruction::Nop => false,
        }
    }

    fn as_number_const(&self, function: &mut Function, expr: ExprId) -> Option<ConstIndex> {
        match *self.ast.get(expr) {
            Expr::NumberLiteral(value) => Some(function.push_number(value)),
            _ => None,
        }
    }

    pub fn lower(&self) -> Vec<Function> {
        let entry = self.ast.entry();
        let mut functions = Vec::new();

        let index = functions.len();
        functions.push(None);

        let mut function = Function::new(index, 0);
        let mut names = Vec::new();

        let src = self.lower_expression(&mut functions, &mut function, &mut names, entry, None);

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
        dest: Option<Register>,
    ) -> Register {
        for expression in expressions.iter().copied() {
            let expression = self.ast.get(expression);
            if let Expr::Function { name, .. } = &expression
                && let Some(name) = name
            {
                self.lower_expression(functions, function, names, *name, None);
            }
        }

        let dest = dest.unwrap_or_else(|| function.allocate_register());

        expressions.iter().copied().fold(dest, |_, expression| {
            self.lower_expression(functions, function, names, expression, Some(dest))
        })
    }

    fn lower_expression(
        &self,
        functions: &mut Vec<Option<Function>>,
        function: &mut Function,
        names: &mut Vec<(StringIndex, Register)>,
        expression: ExprId,
        dest: Option<Register>,
    ) -> Register {
        match *self.ast.get(expression) {
            Expr::NumberLiteral(value) => {
                let src = function.push_number(value);
                let dest = dest.unwrap_or_else(|| function.allocate_register());

                function.emit_instruction(Instruction::LoadK { dest, src });

                dest
            }
            Expr::StringLiteral(value) => {
                let src = function.push_string(value);
                let dest = dest.unwrap_or_else(|| function.allocate_register());

                function.emit_instruction(Instruction::LoadK { dest, src });

                dest
            }
            Expr::BooleanLiteral(_) => {
                let dest = dest.unwrap_or_else(|| function.allocate_register());

                todo!()
            }
            Expr::Identifier(name) => {
                let register = Self::lookup_or_declare(names, function, name);

                if let Some(dest) = dest
                    && dest != register
                {
                    function.emit_instruction(Instruction::Move {
                        dest,
                        src: register,
                    });
                    dest
                } else {
                    register
                }
            }
            Expr::DeclareAssign { left, right } => {
                let dest = Self::lookup_or_declare(
                    names,
                    function,
                    match *self.ast.get(left) {
                        Expr::Identifier(name) => name,
                        _ => panic!("DeclareAssign left must be Identifier"),
                    },
                );

                self.lower_expression(functions, function, names, right, Some(dest));

                dest
            }
            Expr::Assign { left, right } => {
                let dest = self.lower_expression(functions, function, names, left, None);

                self.lower_expression(functions, function, names, right, Some(dest));

                dest
            }
            Expr::CompoundAssign {
                operator,
                left,
                right,
            } => {
                let dest = self.lower_expression(functions, function, names, left, None);

                if let Some(src2) = self.as_number_const(function, right) {
                    function.emit_instruction(match operator {
                        AssignOp::AddAssign => Instruction::AddK {
                            dest,
                            src1: dest,
                            src2,
                        },
                        AssignOp::SubtractAssign => Instruction::SubtractRK {
                            dest,
                            src1: dest,
                            src2,
                        },
                        AssignOp::MultiplyAssign => Instruction::MultiplyK {
                            dest,
                            src1: dest,
                            src2,
                        },
                        AssignOp::DivideAssign => Instruction::DivideRK {
                            dest,
                            src1: dest,
                            src2,
                        },
                        AssignOp::ModuloAssign => Instruction::ModuloRK {
                            dest,
                            src1: dest,
                            src2,
                        },
                    });
                } else {
                    let src2 = self.lower_expression(functions, function, names, right, None);
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
                }

                dest
            }
            Expr::Binary {
                operator,
                left,
                right,
            } => {
                let dest = dest.unwrap_or_else(|| function.allocate_register());

                if let Some(src2) = self.as_number_const(function, right) {
                    let src1 = self.lower_expression(functions, function, names, left, None);

                    function.emit_instruction(match operator {
                        BinaryOp::Add => Instruction::AddK { dest, src1, src2 },
                        BinaryOp::Subtract => Instruction::SubtractRK { dest, src1, src2 },
                        BinaryOp::Multiply => Instruction::MultiplyK { dest, src1, src2 },
                        BinaryOp::Divide => Instruction::DivideRK { dest, src1, src2 },
                        BinaryOp::Modulo => Instruction::ModuloRK { dest, src1, src2 },
                        BinaryOp::Less => Instruction::LessK { dest, src1, src2 },
                        BinaryOp::LessEqual => Instruction::LessEqualK { dest, src1, src2 },
                        BinaryOp::Greater => Instruction::GreaterK { dest, src1, src2 },
                        BinaryOp::GreaterEqual => Instruction::GreaterEqualK { dest, src1, src2 },
                        BinaryOp::Equal => Instruction::EqualK { dest, src1, src2 },
                        BinaryOp::NotEqual => Instruction::NotEqualK { dest, src1, src2 },
                    });

                    return dest;
                }

                if let Some(src1) = self.as_number_const(function, left) {
                    let src2 = self.lower_expression(functions, function, names, right, None);

                    function.emit_instruction(match operator {
                        BinaryOp::Add => Instruction::AddK {
                            dest,
                            src1: src2,
                            src2: src1,
                        },
                        BinaryOp::Multiply => Instruction::MultiplyK {
                            dest,
                            src1: src2,
                            src2: src1,
                        },
                        BinaryOp::Equal => Instruction::EqualK {
                            dest,
                            src1: src2,
                            src2: src1,
                        },
                        BinaryOp::NotEqual => Instruction::NotEqualK {
                            dest,
                            src1: src2,
                            src2: src1,
                        },
                        BinaryOp::Subtract => Instruction::SubtractKR { dest, src1, src2 },
                        BinaryOp::Divide => Instruction::DivideKR { dest, src1, src2 },
                        BinaryOp::Modulo => Instruction::ModuloKR { dest, src1, src2 },
                        BinaryOp::Less => Instruction::GreaterK {
                            dest,
                            src1: src2,
                            src2: src1,
                        },
                        BinaryOp::LessEqual => Instruction::GreaterEqualK {
                            dest,
                            src1: src2,
                            src2: src1,
                        },
                        BinaryOp::Greater => Instruction::LessK {
                            dest,
                            src1: src2,
                            src2: src1,
                        },
                        BinaryOp::GreaterEqual => Instruction::LessEqualK {
                            dest,
                            src1: src2,
                            src2: src1,
                        },
                    });
                    return dest;
                }

                let src1 = self.lower_expression(functions, function, names, left, None);
                let src2 = self.lower_expression(functions, function, names, right, None);

                function.emit_instruction(match operator {
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
                });

                dest
            }
            Expr::Unary { operator, right } => {
                let dest = dest.unwrap_or_else(|| function.allocate_register());
                let src = self.lower_expression(functions, function, names, right, None);

                function.emit_instruction(match operator {
                    UnaryOp::Negate => Instruction::Negate { dest, src },
                });

                dest
            }
            Expr::LogicalNot(expression) => {
                let dest = dest.unwrap_or_else(|| function.allocate_register());
                let src = self.lower_expression(functions, function, names, expression, None);
                function.emit_instruction(Instruction::Not { dest, src });

                dest
            }
            Expr::LogicalAnd { left, right } => {
                let dest = dest.unwrap_or_else(|| function.allocate_register());

                self.lower_expression(functions, function, names, left, Some(dest));

                let jump_if_false = lower_conditional_jump(function, dest, true);

                self.lower_expression(functions, function, names, right, Some(dest));

                patch_jump(
                    function,
                    jump_if_false,
                    function.instructions.len() as i32 - jump_if_false as i32,
                );
                dest
            }
            Expr::LogicalOr { left, right } => {
                let dest = dest.unwrap_or_else(|| function.allocate_register());
                self.lower_expression(functions, function, names, left, Some(dest));

                let jump_if_true = lower_conditional_jump(function, dest, false);

                self.lower_expression(functions, function, names, right, Some(dest));

                patch_jump(
                    function,
                    jump_if_true,
                    function.instructions.len() as i32 - jump_if_true as i32,
                );
                dest
            }
            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let dest = dest.unwrap_or_else(|| function.allocate_register());

                let condition = self.lower_expression(functions, function, names, condition, None);

                let jump_if_false = lower_conditional_jump(function, condition, true);

                self.lower_expression(functions, function, names, then_branch, Some(dest));

                let jump_end = function.emit_instruction(Instruction::Jump { offset: 0 });

                patch_jump(
                    function,
                    jump_if_false,
                    function.instructions.len() as i32 - jump_if_false as i32,
                );

                if let Some(else_branch) = else_branch {
                    self.lower_expression(functions, function, names, else_branch, Some(dest));
                } else {
                    let src = function.push_number(0.0);
                    function.emit_instruction(Instruction::LoadK { dest, src });
                }

                patch_jump(
                    function,
                    jump_end,
                    function.instructions.len() as i32 - jump_end as i32,
                );

                dest
            }
            Expr::WhileLoop { condition, block } => {
                let dest = dest.unwrap_or_else(|| function.allocate_register());

                let condition_register =
                    self.lower_expression(functions, function, names, condition, None);

                let jump_if_false = lower_conditional_jump(function, condition_register, true);

                let loop_body = function.instructions.len();

                self.lower_expression(functions, function, names, block, Some(dest));

                let condition_register =
                    self.lower_expression(functions, function, names, condition, None);

                let jump_if_true = lower_conditional_jump(function, condition_register, false);

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

                dest
            }

            Expr::Block(ref expressions) => {
                let size = names.len();
                let result = self.lower_block(functions, function, names, expressions, dest);
                names.truncate(size);
                result
            }
            Expr::Function {
                ref parameters,
                block,
                name,
            } => {
                let index = functions.len();
                functions.push(None);

                let dest = match name {
                    Some(name) => self.lower_expression(functions, function, names, name, None),
                    None => dest.unwrap_or_else(|| function.allocate_register()),
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
                let mut inner_function = Function::new(index, arity);
                let mut inner_names = Vec::new();

                for parameter in parameters.iter().copied() {
                    self.lower_expression(
                        functions,
                        &mut inner_function,
                        &mut inner_names,
                        parameter,
                        None,
                    );
                }

                for capture in self.captures.get(&expression).unwrap().iter().copied() {
                    Self::lookup_or_declare(&mut inner_names, &mut inner_function, capture);
                }

                let src = self.lower_expression(
                    functions,
                    &mut inner_function,
                    &mut inner_names,
                    block,
                    None,
                );

                if !self.expression_returns(block) {
                    inner_function.emit_instruction(Instruction::Return { src });
                }

                functions[index] = Some(inner_function);

                dest
            }
            Expr::FunctionCall {
                callee,
                ref arguments,
            } => {
                let callee_src = self.lower_expression(functions, function, names, callee, None);

                for (index, argument) in arguments.iter().enumerate() {
                    let dest = Register(-((index + 1) as i16));
                    self.lower_expression(functions, function, names, *argument, Some(dest));
                }

                let dest = dest.unwrap_or_else(|| function.allocate_register());

                function.emit_instruction(Instruction::Call {
                    dest,
                    src: callee_src,
                    arity: arguments.len() as u8,
                });

                dest
            }
            Expr::MemberAccess { object, property } => {
                let dest = dest.unwrap_or_else(|| function.allocate_register());
                let object = self.lower_expression(functions, function, names, object, None);
                let key = self.lower_expression(functions, function, names, property, None);

                function.emit_instruction(Instruction::GetField { dest, object, key });

                dest
            }
            Expr::DictLiteral { .. } => {
                let dest = dest.unwrap_or_else(|| function.allocate_register());
                function.emit_instruction(Instruction::CreateDict { dest });

                dest
            }
            Expr::Return(expression) => {
                let src = match expression {
                    Some(expr) => self.lower_expression(functions, function, names, expr, None),
                    None => function.emit_nil(),
                };
                function.emit_instruction(Instruction::Return { src });

                src
            }
            Expr::NativeFunction { .. } => todo!(),
            Expr::ForLoop { .. } => todo!(),
            Expr::Break => todo!(),
            Expr::Continue => todo!(),
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
        match *self.ast.get(expression) {
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

fn lower_conditional_jump(function: &mut Function, register: Register, invert: bool) -> usize {
    let last = function.instructions.last().copied();

    match last {
        Some(Instruction::Less { dest, src1, src2 }) => {
            function.instructions.pop();
            function.remove_live_range(dest); // dest is dead, fused into jump
            function.emit_instruction(if invert {
                Instruction::JumpIfGreaterEqual {
                    src1,
                    src2,
                    offset: 0,
                }
            } else {
                Instruction::JumpIfLess {
                    src1,
                    src2,
                    offset: 0,
                }
            })
        }
        Some(Instruction::LessK { dest, src1, src2 }) => {
            function.instructions.pop();
            function.remove_live_range(dest);
            function.emit_instruction(if invert {
                Instruction::JumpIfGreaterEqualK {
                    src1,
                    src2,
                    offset: 0,
                }
            } else {
                Instruction::JumpIfLessK {
                    src1,
                    src2,
                    offset: 0,
                }
            })
        }
        Some(Instruction::LessEqual { dest, src1, src2 }) => {
            function.instructions.pop();
            function.remove_live_range(dest);
            function.emit_instruction(if invert {
                Instruction::JumpIfGreater {
                    src1,
                    src2,
                    offset: 0,
                }
            } else {
                Instruction::JumpIfLessEqual {
                    src1,
                    src2,
                    offset: 0,
                }
            })
        }
        Some(Instruction::LessEqualK { dest, src1, src2 }) => {
            function.instructions.pop();
            function.remove_live_range(dest);
            function.emit_instruction(if invert {
                Instruction::JumpIfGreaterK {
                    src1,
                    src2,
                    offset: 0,
                }
            } else {
                Instruction::JumpIfLessEqualK {
                    src1,
                    src2,
                    offset: 0,
                }
            })
        }
        Some(Instruction::Greater { dest, src1, src2 }) => {
            function.instructions.pop();
            function.remove_live_range(dest);
            function.emit_instruction(if invert {
                Instruction::JumpIfLessEqual {
                    src1,
                    src2,
                    offset: 0,
                }
            } else {
                Instruction::JumpIfGreater {
                    src1,
                    src2,
                    offset: 0,
                }
            })
        }
        Some(Instruction::GreaterK { dest, src1, src2 }) => {
            function.instructions.pop();
            function.remove_live_range(dest);
            function.emit_instruction(if invert {
                Instruction::JumpIfLessEqualK {
                    src1,
                    src2,
                    offset: 0,
                }
            } else {
                Instruction::JumpIfGreaterK {
                    src1,
                    src2,
                    offset: 0,
                }
            })
        }
        Some(Instruction::GreaterEqual { dest, src1, src2 }) => {
            function.instructions.pop();
            function.remove_live_range(dest);
            function.emit_instruction(if invert {
                Instruction::JumpIfLess {
                    src1,
                    src2,
                    offset: 0,
                }
            } else {
                Instruction::JumpIfGreaterEqual {
                    src1,
                    src2,
                    offset: 0,
                }
            })
        }
        Some(Instruction::GreaterEqualK { dest, src1, src2 }) => {
            function.instructions.pop();
            function.remove_live_range(dest);
            function.emit_instruction(if invert {
                Instruction::JumpIfLessK {
                    src1,
                    src2,
                    offset: 0,
                }
            } else {
                Instruction::JumpIfGreaterEqualK {
                    src1,
                    src2,
                    offset: 0,
                }
            })
        }
        Some(Instruction::Equal { dest, src1, src2 }) => {
            function.instructions.pop();
            function.remove_live_range(dest);
            function.emit_instruction(if invert {
                Instruction::JumpIfNotEqual {
                    src1,
                    src2,
                    offset: 0,
                }
            } else {
                Instruction::JumpIfEqual {
                    src1,
                    src2,
                    offset: 0,
                }
            })
        }
        Some(Instruction::EqualK { dest, src1, src2 }) => {
            function.instructions.pop();
            function.remove_live_range(dest);
            function.emit_instruction(if invert {
                Instruction::JumpIfNotEqualK {
                    src1,
                    src2,
                    offset: 0,
                }
            } else {
                Instruction::JumpIfEqualK {
                    src1,
                    src2,
                    offset: 0,
                }
            })
        }
        Some(Instruction::NotEqual { dest, src1, src2 }) => {
            function.instructions.pop();
            function.remove_live_range(dest);
            function.emit_instruction(if invert {
                Instruction::JumpIfEqual {
                    src1,
                    src2,
                    offset: 0,
                }
            } else {
                Instruction::JumpIfNotEqual {
                    src1,
                    src2,
                    offset: 0,
                }
            })
        }
        Some(Instruction::NotEqualK { dest, src1, src2 }) => {
            function.instructions.pop();
            function.remove_live_range(dest);
            function.emit_instruction(if invert {
                Instruction::JumpIfEqualK {
                    src1,
                    src2,
                    offset: 0,
                }
            } else {
                Instruction::JumpIfNotEqualK {
                    src1,
                    src2,
                    offset: 0,
                }
            })
        }
        _ => function.emit_instruction(if invert {
            Instruction::JumpIfFalse {
                src: register,
                offset: 0,
            }
        } else {
            Instruction::JumpIfTrue {
                src: register,
                offset: 0,
            }
        }),
    }
}

fn patch_jump(function: &mut Function, index: usize, new_offset: i32) {
    match &mut function.instructions[index] {
        Instruction::Jump { offset }
        | Instruction::JumpIfTrue { offset, .. }
        | Instruction::JumpIfFalse { offset, .. }
        | Instruction::JumpIfLess { offset, .. }
        | Instruction::JumpIfLessK { offset, .. }
        | Instruction::JumpIfLessEqual { offset, .. }
        | Instruction::JumpIfLessEqualK { offset, .. }
        | Instruction::JumpIfGreater { offset, .. }
        | Instruction::JumpIfGreaterK { offset, .. }
        | Instruction::JumpIfGreaterEqual { offset, .. }
        | Instruction::JumpIfGreaterEqualK { offset, .. }
        | Instruction::JumpIfEqual { offset, .. }
        | Instruction::JumpIfEqualK { offset, .. }
        | Instruction::JumpIfNotEqual { offset, .. }
        | Instruction::JumpIfNotEqualK { offset, .. } => *offset = new_offset,
        _ => panic!("tried to patch a non-jump instruction at index {index}"),
    }
}
