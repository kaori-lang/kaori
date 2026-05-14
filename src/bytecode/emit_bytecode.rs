use std::collections::HashMap;

use crate::{
    bytecode::{function::Function, function_scope::Scope, instruction::Instruction},
    syntax::{
        ast::{Ast, Expr, ExprId},
        ops::{AssignOp, BinaryOp, UnaryOp},
    },
    util::string_interner::StringIndex,
};

#[derive(Default)]
struct Registers(u8);

impl Registers {
    pub fn allocate_register(&mut self) -> Register {
        let register = self.0;

        self.0 += 1;

        register
    }
}

type Register = u8;

pub struct CompilerContext {
    ast: Ast,
    captures: HashMap<ExprId, Vec<StringIndex>>,
}

impl CompilerContext {
    pub fn new(ast: Ast, captures: HashMap<ExprId, Vec<StringIndex>>) -> Self {
        Self { ast, captures }
    }

    fn nil(function: &mut Function, registers: &mut Registers) -> Register {
        let dest = registers.allocate_register();

        let src = function.push_number(0.0);

        function.instructions.push(Instruction::LoadK {
            dest,
            src: src as u16,
        });

        dest
    }

    fn lookup_or_declare(
        scopes: &mut Scope,
        registers: &mut Registers,
        name: StringIndex,
    ) -> Register {
        if let Some(register) = scopes.lookup(name) {
            register
        } else {
            let register = registers.allocate_register();

            scopes.insert_symbol(name, register);

            register
        }
    }

    pub fn compile(&self) -> Vec<Function> {
        let entry = self.ast.entry();
        let mut functions = Vec::new();

        functions.push(None);

        let mut function = Function::default();
        let mut scope = Scope::default();
        let mut registers = Registers::default();

        let src = self.compile_expression(
            &mut functions,
            &mut function,
            &mut scope,
            &mut registers,
            entry,
        );

        if !self.expression_returns(entry) {
            function.emit_instruction(Instruction::Return { src });
        }

        patch_function_arguments(&mut function);

        functions[0] = Some(function);

        functions
            .into_iter()
            .map(|function| function.unwrap())
            .collect()
    }

    fn compile_block(
        &self,
        functions: &mut Vec<Option<Function>>,
        function: &mut Function,
        scope: &mut Scope,
        registers: &mut Registers,
        expressions: &[ExprId],
    ) -> Register {
        for expression in expressions.iter().copied() {
            let expression = self.ast.get(expression);

            if let Expr::Function { name, .. } = &expression
                && let Some(name) = name
            {
                self.compile_expression(functions, function, scope, registers, *name);
            }
        }

        expressions
            .iter()
            .copied()
            .fold(Self::nil(function, registers), |_, expression| {
                self.compile_expression(functions, function, scope, registers, expression)
            })
    }

    fn compile_expression(
        &self,
        functions: &mut Vec<Option<Function>>,
        function: &mut Function,
        scope: &mut Scope,
        registers: &mut Registers,
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

                let mut function = Function::default();

                let dest = match name {
                    Some(name) => {
                        self.compile_expression(functions, &mut function, scope, registers, name)
                    }
                    None => registers.allocate_register(),
                };

                function.emit_instruction(Instruction::CreateClosure {
                    dest,
                    src: index as u32,
                });

                for capture in self.captures.get(&expression).unwrap().iter().copied() {
                    let src = Self::lookup_or_declare(scope, registers, capture);

                    function.emit_instruction(Instruction::CaptureValue { dest, src });
                }

                let mut scope = Scope::default();

                scope.enter_scope();

                for parameter in parameters.iter().copied() {
                    self.compile_expression(
                        functions,
                        &mut function,
                        &mut scope,
                        registers,
                        parameter,
                    );
                }

                for capture in self.captures.get(&expression).unwrap().iter().copied() {
                    Self::lookup_or_declare(&mut scope, registers, capture);
                }

                let src =
                    self.compile_expression(functions, &mut function, &mut scope, registers, block);

                if !self.expression_returns(block) {
                    function.emit_instruction(Instruction::Return { src });
                }

                patch_function_arguments(&mut function);

                functions[index] = Some(function);

                scope.exit_scope();

                dest
            }
            Expr::DeclareAssign { left, right } => {
                let src = self.compile_expression(functions, function, scope, registers, right);
                let dest = self.compile_expression(functions, function, scope, registers, left);

                function.emit_instruction(Instruction::Move { dest, src });

                dest
            }
            Expr::Assign {
                operator,
                left,
                right,
            } => {
                let dest = self.compile_expression(functions, function, scope, registers, left);

                let src = match operator {
                    AssignOp::Assign => {
                        self.compile_expression(functions, function, scope, registers, right)
                    }
                    AssignOp::AddAssign => self.compile_binary_op(
                        functions,
                        function,
                        scope,
                        registers,
                        BinaryOp::Add,
                        left,
                        right,
                    ),
                    AssignOp::SubtractAssign => self.compile_binary_op(
                        functions,
                        function,
                        scope,
                        registers,
                        BinaryOp::Subtract,
                        left,
                        right,
                    ),
                    AssignOp::MultiplyAssign => self.compile_binary_op(
                        functions,
                        function,
                        scope,
                        registers,
                        BinaryOp::Multiply,
                        left,
                        right,
                    ),
                    AssignOp::DivideAssign => self.compile_binary_op(
                        functions,
                        function,
                        scope,
                        registers,
                        BinaryOp::Divide,
                        left,
                        right,
                    ),
                    AssignOp::ModuloAssign => self.compile_binary_op(
                        functions,
                        function,
                        scope,
                        registers,
                        BinaryOp::Modulo,
                        left,
                        right,
                    ),
                };

                function.emit_instruction(Instruction::Move { dest, src });

                dest
            }
            Expr::LogicalAnd { left, right } => {
                let dest = registers.allocate_register();

                let src = self.compile_expression(functions, function, scope, registers, left);

                function.emit_instruction(Instruction::Move { dest, src });

                let jump_if_false = function.emit_instruction(Instruction::JumpIfFalse {
                    src: dest,
                    offset: 0,
                });

                let src = self.compile_expression(functions, function, scope, registers, right);

                function.emit_instruction(Instruction::Move { dest, src });

                patch_jump(
                    function,
                    jump_if_false,
                    function.instructions.len() as i32 - jump_if_false as i32,
                );

                dest
            }
            Expr::LogicalOr { left, right } => {
                let dest = registers.allocate_register();

                let src = self.compile_expression(functions, function, scope, registers, left);

                function.emit_instruction(Instruction::Move { dest, src });

                let jump_if_true = function.emit_instruction(Instruction::JumpIfTrue {
                    src: dest,
                    offset: 0,
                });

                let src = self.compile_expression(functions, function, scope, registers, right);

                function.emit_instruction(Instruction::Move { dest, src });

                patch_jump(
                    function,
                    jump_if_true,
                    function.instructions.len() as i32 - jump_if_true as i32,
                );

                dest
            }
            Expr::LogicalNot(expression) => {
                let src =
                    self.compile_expression(functions, function, scope, registers, expression);
                let dest = registers.allocate_register();
                function.emit_instruction(Instruction::Not { dest, src });

                dest
            }
            Expr::Binary {
                operator,
                left,
                right,
            } => {
                let src1 = self.compile_expression(functions, function, scope, registers, left);
                let src2 = self.compile_expression(functions, function, scope, registers, right);
                let dest = registers.allocate_register();

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
                let src = self.compile_expression(functions, function, scope, registers, right);
                let dest = registers.allocate_register();

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
                let dest = registers.allocate_register();
                let callee_src =
                    self.compile_expression(functions, function, scope, registers, callee);

                for (index, argument) in arguments.iter().enumerate() {
                    let argument =
                        self.compile_expression(functions, function, scope, registers, *argument);
                    function.emit_instruction(Instruction::MoveArg {
                        dest: index as Register,
                        src: argument,
                    });
                }

                function.emit_instruction(Instruction::Call {
                    dest,
                    src: callee_src,
                    arity: arguments.len() as Register,
                });

                dest
            }
            Expr::MemberAccess { object, property } => {
                let object = self.compile_expression(functions, function, scope, registers, object);
                let key = self.compile_expression(functions, function, scope, registers, property);
                let dest = registers.allocate_register();

                function.emit_instruction(Instruction::GetField { dest, object, key });

                dest
            }
            Expr::Block(ref expressions) => {
                scope.enter_scope();
                let dest = self.compile_block(functions, function, scope, registers, expressions);
                scope.exit_scope();

                dest
            }
            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let dest = registers.allocate_register();

                let src = self.compile_expression(functions, function, scope, registers, condition);

                let jump_if_false =
                    function.emit_instruction(Instruction::JumpIfFalse { src, offset: 0 });

                let src =
                    self.compile_expression(functions, function, scope, registers, then_branch);
                function.emit_instruction(Instruction::Move { dest, src });

                let jump_end = function.emit_instruction(Instruction::Jump { offset: 0 });

                patch_jump(
                    function,
                    jump_if_false,
                    function.instructions.len() as i32 - jump_if_false as i32,
                );

                let src = if let Some(else_branch) = else_branch {
                    self.compile_expression(functions, function, scope, registers, else_branch)
                } else {
                    Self::nil(function, registers)
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
                let src = self.compile_expression(functions, function, scope, registers, condition);

                let jump_if_false =
                    function.emit_instruction(Instruction::JumpIfFalse { src, offset: 0 });

                let loop_body = function.instructions.len();

                self.compile_expression(functions, function, scope, registers, block);

                let src = self.compile_expression(functions, function, scope, registers, condition);

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

                Self::nil(function, registers)
            }
            Expr::Return(expression) => {
                let src = match expression {
                    Some(expr) => {
                        self.compile_expression(functions, function, scope, registers, expr)
                    }
                    None => Self::nil(function, registers),
                };

                function.emit_instruction(Instruction::Return { src });

                Self::nil(function, registers)
            }
            Expr::Break => todo!(),
            Expr::Continue => todo!(),
            Expr::Identifier(name) => Self::lookup_or_declare(scope, registers, name),
            Expr::StringLiteral(value) => {
                let src = function.push_string(value);
                let dest = registers.allocate_register();
                function.emit_instruction(Instruction::LoadK {
                    dest,
                    src: src as u16,
                });

                dest
            }
            Expr::NumberLiteral(value) => {
                let src = function.push_number(value);
                let dest = registers.allocate_register();

                function.emit_instruction(Instruction::LoadK {
                    dest,
                    src: src as u16,
                });

                dest
            }
            Expr::DictLiteral { ref fields } => {
                let dest = registers.allocate_register();
                function.emit_instruction(Instruction::CreateDict { dest });

                for (key, value) in fields.iter().copied() {
                    todo!()
                }

                dest
            }
        }
    }

    fn compile_binary_op(
        &self,
        functions: &mut Vec<Option<Function>>,
        function: &mut Function,
        scope: &mut Scope,
        registers: &mut Registers,
        operator: BinaryOp,
        left: ExprId,
        right: ExprId,
    ) -> Register {
        let src1 = self.compile_expression(functions, function, scope, registers, left);
        let src2 = self.compile_expression(functions, function, scope, registers, right);
        let dest = registers.allocate_register();

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

    /*     fn compile_loop(
           &mut self,
           scope: &mut Scope,
           init: Option<ExprId>,
           condition: &Expr,
           block: &Expr,
           increment: Option<&Expr>,
       ) -> Operand {
           if let Some(init) = init {
               self.compile_expression(functions, function, scope, registers,  init);
           }

           let src = self.compile_expression(functions, function, scope, registers,  condition);
           let src = materialize(scope, src);

           let jump_if_false = function.emit_instruction(Instruction::JumpIfFalse {
               src,
               offset: 0,
           });

           let loop_body = function.instructions.len();

           self.compile_expression(functions, function, scope, registers,  block);

           if let Some(increment) = increment {
               self.compile_expression(functions, function, scope, registers,  increment);
           }

           let src = self.compile_expression(functions, function, scope, registers,  condition);
           let src = materialize(scope, src);

           let jump_if_true = function.emit_instruction(Instruction::JumpIfTrue {
               src,
               offset: 0,
           });

           patch_jump(function, jump_if_true, loop_body as i32 - jump_if_true as i32);
           patch_jump(
               scope,
               jump_if_false,
               function.instructions.len() as i32 - jump_if_false as i32,
           );

           Self::nil(function,registers)
       }

    */

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

fn patch_function_arguments(function: &mut Function) {
    for instruction in &mut function.instructions {
        if let Instruction::MoveArg { dest, .. } = instruction {
            *dest += 0;
        }
    }
}
