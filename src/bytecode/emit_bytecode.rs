use crate::{
    bytecode::{
        function::Function, function_scope::FunctionScope, immediate::Imm,
        instruction::Instruction, operand::Operand,
    },
    runtime::value::Value,
    syntax::{
        ast::{Ast, Expr, ExprId},
        ops::{AssignOp, BinaryOp, UnaryOp},
    },
    util::string_interner::StringIndex,
};

#[derive(Default)]
pub struct Compiler {
    functions: Vec<Option<Function>>,
    constants: Vec<Value>,
}

impl Compiler {
    fn get_or_insert(&mut self, value: Value) -> usize {
        if let Some(index) = self.constants.iter().copied().position(|c| c == value) {
            return index;
        }

        let index = self.constants.len();
        self.constants.push(value);

        index
    }

    pub fn push_string(&mut self, value: StringIndex) -> usize {
        self.get_or_insert(Value::string(value))
    }

    pub fn push_number(&mut self, value: f64) -> usize {
        self.get_or_insert(Value::number(value))
    }

    pub fn compile(mut self, ast: &Ast) -> (Vec<Function>, Vec<Value>) {
        let entry = ast.entry();

        let index = self.functions.len();
        let function = None;
        self.functions.push(function);

        let mut scope = FunctionScope::default();
        let src = self.compile_expression(ast, &mut scope, entry);

        if !self.expression_returns(ast, entry) {
            let src = materialize(&mut scope, src);

            scope.emit_instruction(Instruction::Return {
                src: src.unwrap_register(),
            });
        }

        patch_function_arguments(&mut scope);

        let function = Function {
            instructions: scope.instructions,
            registers_count: scope.next_register,
            arity: 0,
        };

        self.functions[index] = Some(function);

        let functions = self
            .functions
            .into_iter()
            .map(|f| f.unwrap())
            .collect::<Vec<Function>>();

        (functions, self.constants)
    }

    fn compile_block(
        &mut self,
        ast: &Ast,
        scope: &mut FunctionScope,
        expressions: &[ExprId],
    ) -> Operand {
        for expression in expressions.iter().copied() {
            let expression = ast.get(expression);

            if let Expr::Function { name, .. } = &expression
                && let Some(name) = name
            {
                self.compile_expression(ast, scope, *name);
            }
        }

        expressions.iter().copied().fold(unit(), |_, expression| {
            self.compile_expression(ast, scope, expression)
        })
    }

    fn compile_expression(
        &mut self,
        ast: &Ast,
        scope: &mut FunctionScope,
        expression: ExprId,
    ) -> Operand {
        let expression = ast.get(expression);

        match *expression {
            Expr::NativeFunction { .. } => {
                todo!()
            }
            Expr::Function {
                ref parameters,
                block,
                name,
            } => {
                let index = self.functions.len();
                let function = None;
                self.functions.push(function);

                let dest = self.compile_expression(ast, scope, name.unwrap());

                scope.emit_instruction(Instruction::CreateClosure {
                    dest: dest.unwrap_register(),
                    src: index as u32,
                });

                let mut scope = FunctionScope::default();

                scope.enter_scope();

                for parameter in parameters.iter().copied() {
                    self.compile_expression(ast, &mut scope, parameter);
                }

                let src = self.compile_expression(ast, &mut scope, block);

                if !self.expression_returns(ast, block) {
                    let src = materialize(&mut scope, src);
                    scope.emit_instruction(Instruction::Return {
                        src: src.unwrap_register(),
                    });
                }

                patch_function_arguments(&mut scope);

                scope.exit_scope();

                let function = Function {
                    instructions: scope.instructions,
                    registers_count: scope.next_register,
                    arity: parameters.len() as u8,
                };

                self.functions[index] = Some(function);

                dest
            }
            Expr::DeclareAssign { left, right } => {
                let src = self.compile_expression(ast, scope, right);
                let src = materialize(scope, src);

                let dest = self.compile_expression(ast, scope, left);

                scope.emit_instruction(Instruction::Move {
                    dest: dest.unwrap_register(),
                    src: src.unwrap_register(),
                });

                dest
            }
            Expr::Assign {
                operator,
                left,
                right,
            } => {
                let dest = self.compile_expression(ast, scope, left);

                let src = match operator {
                    AssignOp::Assign => self.compile_expression(ast, scope, right),
                    AssignOp::AddAssign => {
                        self.compile_binary_op(ast, scope, BinaryOp::Add, left, right)
                    }
                    AssignOp::SubtractAssign => {
                        self.compile_binary_op(ast, scope, BinaryOp::Subtract, left, right)
                    }
                    AssignOp::MultiplyAssign => {
                        self.compile_binary_op(ast, scope, BinaryOp::Multiply, left, right)
                    }
                    AssignOp::DivideAssign => {
                        self.compile_binary_op(ast, scope, BinaryOp::Divide, left, right)
                    }
                    AssignOp::ModuloAssign => {
                        self.compile_binary_op(ast, scope, BinaryOp::Modulo, left, right)
                    }
                };
                let src = materialize(scope, src);

                scope.emit_instruction(Instruction::Move {
                    dest: dest.unwrap_register(),
                    src: src.unwrap_register(),
                });

                dest
            }
            Expr::LogicalAnd { left, right } => {
                let dest = scope.allocate_register();

                let src = self.compile_expression(ast, scope, left);
                let src = materialize(scope, src);
                scope.emit_instruction(Instruction::Move {
                    dest,
                    src: src.unwrap_register(),
                });

                let jump_if_false = scope.emit_instruction(Instruction::JumpIfFalse {
                    src: dest,
                    offset: 0,
                });

                let src = self.compile_expression(ast, scope, right);
                let src = materialize(scope, src);
                scope.emit_instruction(Instruction::Move {
                    dest,
                    src: src.unwrap_register(),
                });

                patch_jump(
                    scope,
                    jump_if_false,
                    scope.instructions.len() as i32 - jump_if_false as i32,
                );

                Operand::Register(dest)
            }
            Expr::LogicalOr { left, right } => {
                let dest = scope.allocate_register();

                let src = self.compile_expression(ast, scope, left);
                let src = materialize(scope, src);
                scope.emit_instruction(Instruction::Move {
                    dest,
                    src: src.unwrap_register(),
                });

                let jump_if_true = scope.emit_instruction(Instruction::JumpIfTrue {
                    src: dest,
                    offset: 0,
                });

                let src = self.compile_expression(ast, scope, right);
                let src = materialize(scope, src);
                scope.emit_instruction(Instruction::Move {
                    dest,
                    src: src.unwrap_register(),
                });

                patch_jump(
                    scope,
                    jump_if_true,
                    scope.instructions.len() as i32 - jump_if_true as i32,
                );

                Operand::Register(dest)
            }
            Expr::LogicalNot(expression) => {
                let src = self.compile_expression(ast, scope, expression);
                let src = materialize(scope, src);
                let dest = scope.allocate_register();
                scope.emit_instruction(Instruction::Not {
                    dest,
                    src: src.unwrap_register(),
                });
                Operand::Register(dest)
            }
            Expr::Binary {
                operator,
                left,
                right,
            } => self.compile_binary_op(ast, scope, operator, left, right),
            Expr::Unary { operator, right } => {
                let src = self.compile_expression(ast, scope, right);
                let src = materialize(scope, src);
                let dest = scope.allocate_register();

                let instruction = match operator {
                    UnaryOp::Negate => Instruction::Negate {
                        dest,
                        src: src.unwrap_register(),
                    },
                };

                scope.emit_instruction(instruction);
                Operand::Register(dest)
            }
            Expr::FunctionCall {
                callee,
                ref arguments,
            } => {
                let dest = scope.allocate_register();
                let callee_src = self.compile_expression(ast, scope, callee);

                for (index, argument) in arguments.iter().enumerate() {
                    let argument = self.compile_expression(ast, scope, *argument);
                    let argument = materialize(scope, argument);
                    scope.emit_instruction(Instruction::MoveArg {
                        dest: index as u8,
                        src: argument.unwrap_register(),
                    });
                }

                scope.emit_instruction(Instruction::Call {
                    dest,
                    src: callee_src.unwrap_register(),
                    arity: arguments.len() as u8,
                });

                Operand::Register(dest)
            }
            Expr::MemberAccess { object, property } => {
                let dest = scope.allocate_register();

                let object = self.compile_expression(ast, scope, object);
                let object = materialize(scope, object);
                let key = self.compile_expression(ast, scope, property);
                let key = materialize(scope, key);

                scope.emit_instruction(Instruction::GetField {
                    dest,
                    object: object.unwrap_register(),
                    key: key.unwrap_register(),
                });

                Operand::Register(dest)
            }
            Expr::Block(ref expressions) => {
                scope.enter_scope();
                let dest = self.compile_block(ast, scope, expressions);
                scope.exit_scope();

                dest
            }
            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let dest = scope.allocate_register();

                let src = self.compile_expression(ast, scope, condition);
                let src = materialize(scope, src);

                let jump_if_false = scope.emit_instruction(Instruction::JumpIfFalse {
                    src: src.unwrap_register(),
                    offset: 0,
                });

                let src = self.compile_expression(ast, scope, then_branch);
                let src = materialize(scope, src);
                scope.emit_instruction(Instruction::Move {
                    dest,
                    src: src.unwrap_register(),
                });

                let jump_end = scope.emit_instruction(Instruction::Jump { offset: 0 });

                patch_jump(
                    scope,
                    jump_if_false,
                    scope.instructions.len() as i32 - jump_if_false as i32,
                );

                let src = if let Some(else_branch) = else_branch {
                    self.compile_expression(ast, scope, else_branch)
                } else {
                    unit()
                };
                let src = materialize(scope, src);
                scope.emit_instruction(Instruction::Move {
                    dest,
                    src: src.unwrap_register(),
                });

                patch_jump(
                    scope,
                    jump_end,
                    scope.instructions.len() as i32 - jump_end as i32,
                );

                Operand::Register(dest)
            }
            Expr::ForLoop { .. } => unit(),
            Expr::WhileLoop { condition, block } => {
                let src = self.compile_expression(ast, scope, condition);
                let src = materialize(scope, src);

                let jump_if_false = scope.emit_instruction(Instruction::JumpIfFalse {
                    src: src.unwrap_register(),
                    offset: 0,
                });

                let loop_body = scope.instructions.len();

                self.compile_expression(ast, scope, block);

                let src = self.compile_expression(ast, scope, condition);
                let src = materialize(scope, src);

                let jump_if_true = scope.emit_instruction(Instruction::JumpIfTrue {
                    src: src.unwrap_register(),
                    offset: 0,
                });

                patch_jump(scope, jump_if_true, loop_body as i32 - jump_if_true as i32);
                patch_jump(
                    scope,
                    jump_if_false,
                    scope.instructions.len() as i32 - jump_if_false as i32,
                );

                unit()
            }
            Expr::Return(expression) => {
                let src = match expression {
                    Some(expr) => self.compile_expression(ast, scope, expr),
                    None => unit(),
                };
                let src = materialize(scope, src);
                scope.emit_instruction(Instruction::Return {
                    src: src.unwrap_register(),
                });
                unit()
            }
            Expr::Break => todo!(),
            Expr::Continue => todo!(),
            Expr::Identifier(name) => {
                let found = scope.lookup_or_declare(name);

                Operand::Register(found)
            }
            Expr::StringLiteral(value) => {
                let index = self.push_string(value);

                Operand::Constant(index)
            }
            Expr::NumberLiteral(value) => {
                if let Some(imm) = Imm::try_to_encode(value) {
                    Operand::Immediate(imm)
                } else {
                    let index = self.push_number(value);

                    Operand::Constant(index)
                }
            }
            Expr::DictLiteral { ref fields } => {
                let dest = scope.allocate_register();
                scope.emit_instruction(Instruction::CreateDict { dest });

                for (key, value) in fields.iter().copied() {
                    let key_op = self.compile_expression(ast, scope, key);
                    let key_op = materialize(scope, key_op);

                    let value_op = match value {
                        Some(v) => {
                            let v = self.compile_expression(ast, scope, v);
                            materialize(scope, v)
                        }
                        None => {
                            let v = self.compile_expression(ast, scope, key);
                            materialize(scope, v)
                        }
                    };

                    scope.emit_instruction(Instruction::SetField {
                        object: dest,
                        key: key_op.unwrap_register(),
                        value: value_op.unwrap_register(),
                    });
                }

                Operand::Register(dest)
            }
        }
    }

    fn compile_binary_op(
        &mut self,
        ast: &Ast,
        scope: &mut FunctionScope,
        operator: BinaryOp,
        left: ExprId,
        right: ExprId,
    ) -> Operand {
        let src1 = self.compile_expression(ast, scope, left);
        let src2 = self.compile_expression(ast, scope, right);
        let dest = scope.allocate_register();

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
                let src1 = materialize(scope, Operand::Constant(src1)).unwrap_register();
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
                let src2 = materialize(scope, Operand::Constant(src2)).unwrap_register();
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

        scope.emit_instruction(instruction);
        Operand::Register(dest)
    }

    /*     fn compile_loop(
           &mut self,
           scope: &mut FunctionScope,
           init: Option<ExprId>,
           condition: &Expr,
           block: &Expr,
           increment: Option<&Expr>,
       ) -> Operand {
           if let Some(init) = init {
               self.compile_expression(ast, scope, init);
           }

           let src = self.compile_expression(ast, scope, condition);
           let src = materialize(scope, src);

           let jump_if_false = scope.emit_instruction(Instruction::JumpIfFalse {
               src: src.unwrap_register(),
               offset: 0,
           });

           let loop_body = scope.instructions.len();

           self.compile_expression(ast, scope, block);

           if let Some(increment) = increment {
               self.compile_expression(ast, scope, increment);
           }

           let src = self.compile_expression(ast, scope, condition);
           let src = materialize(scope, src);

           let jump_if_true = scope.emit_instruction(Instruction::JumpIfTrue {
               src: src.unwrap_register(),
               offset: 0,
           });

           patch_jump(scope, jump_if_true, loop_body as i32 - jump_if_true as i32);
           patch_jump(
               scope,
               jump_if_false,
               scope.instructions.len() as i32 - jump_if_false as i32,
           );

           unit()
       }

    */

    fn block_returns(&self, ast: &Ast, expressions: &[ExprId]) -> bool {
        for expression in expressions.iter().copied() {
            if self.expression_returns(ast, expression) {
                return true;
            }
        }

        false
    }

    fn expression_returns(&self, ast: &Ast, expression: ExprId) -> bool {
        let expression = ast.get(expression);

        match *expression {
            Expr::Return(..) => true,
            Expr::Block(ref expressions) => self.block_returns(ast, expressions),
            Expr::If {
                then_branch,
                else_branch: Some(else_branch),
                ..
            } => {
                self.expression_returns(ast, then_branch)
                    && self.expression_returns(ast, else_branch)
            }
            _ => false,
        }
    }
}
fn materialize(scope: &mut FunctionScope, src: Operand) -> Operand {
    match src {
        Operand::Register(_) => src,
        Operand::Constant(src) => {
            let dest = scope.allocate_register();
            scope.emit_instruction(Instruction::LoadK {
                dest,
                src: src as u32,
            });
            Operand::Register(dest)
        }
        Operand::Immediate(src) => {
            let dest = scope.allocate_register();
            scope.emit_instruction(Instruction::LoadImm { dest, src });
            Operand::Register(dest)
        }
    }
}

fn unit() -> Operand {
    Operand::Immediate(Imm::try_to_encode(0.0).unwrap())
}

fn patch_jump(scope: &mut FunctionScope, index: usize, new_offset: i32) {
    match &mut scope.instructions[index] {
        Instruction::Jump { offset }
        | Instruction::JumpIfTrue { offset, .. }
        | Instruction::JumpIfFalse { offset, .. } => *offset = new_offset,
        _ => panic!("tried to patch a non-jump instruction at index {index}"),
    }
}

fn patch_function_arguments(scope: &mut FunctionScope) {
    for instruction in &mut scope.instructions {
        if let Instruction::MoveArg { dest, .. } = instruction {
            *dest += scope.next_register;
        }
    }
}
