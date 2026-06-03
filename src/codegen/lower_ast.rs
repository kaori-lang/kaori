use core::panic;

use crate::{
    codegen::environment::{Environment, Register},
    codegen::free_variables::FreeVariables,
    compiler::{Compiler, INTERNER},
    diagnostics::error::Error,
    report_error,
    runtime::{
        function::Function,
        instruction::{Const, Instruction},
    },
    syntax::{
        ast::{Ast, Node, NodeId},
        ops::{BinaryOp, UnaryOp},
    },
};

pub fn lower_ast(ast: Ast, compiler: &mut Compiler) -> Result<usize, Error> {
    let id = ast.last();

    let mut free_variables = FreeVariables::default();
    let mut env = Environment::new();
    let mut function = Function::new(0);

    let mut lowerer = Lower::new(&ast, compiler, &mut free_variables, &mut env, &mut function);

    let src = lowerer.lower_expression(id, None)?;

    //lowerer.prevent_return(id)?;

    lowerer
        .function
        .emit_instruction(Instruction::Return { src: src.into() });

    lowerer.patch_arguments();

    function.frame_size = env.frame_size;

    let index = compiler.functions.len();

    compiler.functions.push(function);

    Ok(index)
}

struct Lower<'a> {
    ast: &'a Ast,
    compiler: &'a mut Compiler,
    free_variables: &'a mut FreeVariables,
    env: &'a mut Environment,
    function: &'a mut Function,
    unpatched_arguments: Vec<usize>,
}

impl<'a> Lower<'a> {
    fn new(
        ast: &'a Ast,
        compiler: &'a mut Compiler,
        free_variables: &'a mut FreeVariables,
        env: &'a mut Environment,
        function: &'a mut Function,
    ) -> Self {
        Self {
            ast,
            compiler,
            free_variables,
            env,
            function,
            unpatched_arguments: Vec::new(),
        }
    }

    fn lower_statement(&mut self, id: NodeId) -> Result<(), Error> {
        match *self.ast.node(id) {
            Node::Variable { left, right } => {
                let dest = self.env.declare_local(left.value);

                self.lower_expression(right, Some(dest))?;
            }
            Node::Constant { left, right } => {
                let dest = self.env.declare_local(left.value);

                self.lower_expression(right, Some(dest))?;
            }
            Node::Ref { left, right } => {
                let dest = self.env.declare_local(left.value);

                let src = self.lower_expression(right, None)?;

                self.function.emit_instruction(Instruction::CreateRef {
                    dest: dest.into(),
                    src: src.into(),
                });
            }
            Node::Assign { left, right } => match *self.ast.node(left) {
                Node::Identifier(..) => {
                    let dest = self.lower_expression(left, None)?;

                    let src = self.lower_expression(right, Some(dest))?;

                    if src != dest {
                        self.env.free_temp(src);
                    }
                }
                Node::MemberAccess { object, property } => {
                    let value = self.lower_expression(right, None)?;
                    let object = self.lower_expression(object, None)?;
                    let key = {
                        let dest = self.env.allocate_temp();
                        let src = self.function.store_string_const(property.value);

                        self.function.emit_instruction(Instruction::LoadConst {
                            dest: dest.into(),
                            src,
                        });

                        dest
                    };

                    self.function.emit_instruction(Instruction::SetField {
                        object: object.into(),
                        key: key.into(),
                        value: value.into(),
                    });
                }
                Node::Unary {
                    operator: UnaryOp::Deref,
                    operand,
                } => {
                    let dest = self.lower_expression(operand, None)?;
                    let src = self.lower_expression(right, None)?;

                    self.function.emit_instruction(Instruction::DerefSet {
                        dest: dest.into(),
                        src: src.into(),
                    });
                }
                _ => {
                    return Err(report_error!(
                        self.ast.span(left),
                        self.compiler.path,
                        "expected a valid lhs"
                    ));
                }
            },
            Node::WhileLoop { condition, block } => {
                let src = self.lower_expression(condition, None)?;

                let jump_if_false = self.lower_jump_if_false(src);
                self.env.free_temp(src);

                let loop_body = self.function.instructions.len();

                self.lower_statement(block)?;

                let src = self.lower_expression(condition, None)?;

                let jump_if_true = self.lower_jump_if_true(src);
                self.env.free_temp(src);

                self.patch_jump(jump_if_true, loop_body as i32 - jump_if_true as i32);
                self.patch_jump(
                    jump_if_false,
                    self.function.instructions.len() as i32 - jump_if_false as i32,
                );
            }
            Node::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let src = self.lower_expression(condition, None)?;
                self.env.free_temp(src);

                let jump_if_false = self.lower_jump_if_false(src);

                self.lower_statement(then_branch)?;

                let jump_end = self
                    .function
                    .emit_instruction(Instruction::Jump { offset: 0 });

                self.patch_jump(
                    jump_if_false,
                    self.function.instructions.len() as i32 - jump_if_false as i32,
                );

                self.lower_statement(else_branch)?;

                self.patch_jump(
                    jump_end,
                    self.function.instructions.len() as i32 - jump_end as i32,
                );
            }
            Node::Return(expression) => {
                let src = self.lower_expression(expression, None)?;

                self.function
                    .emit_instruction(Instruction::Return { src: src.into() });

                self.env.free_temp(src);
            }
            Node::Block {
                ref statements,
                tail,
            } => {
                self.env.push_scope();

                for id in statements.iter().copied() {
                    if let Node::Function { name, .. } = self.ast.node(id) {
                        self.env.declare_local(name.value);
                    }
                }

                if let Some(id) = tail
                    && let Node::Function { name, .. } = self.ast.node(id)
                {
                    self.env.declare_local(name.value);
                }

                for id in statements.iter().copied() {
                    self.lower_statement(id)?;
                }

                if let Some(id) = tail {
                    self.lower_statement(id)?;
                }

                self.env.pop_scope();
            }
            Node::Function {
                ref parameters,
                block,
                name,
            } => {
                let mut function = Function::new(parameters.len());
                let mut env = Environment::with_parent(std::mem::take(self.env));

                let mut inner_self = Lower::new(
                    self.ast,
                    self.compiler,
                    self.free_variables,
                    &mut env,
                    &mut function,
                );

                inner_self.env.declare_function(name.value);

                for parameter in parameters.iter().copied() {
                    inner_self.env.declare_local(parameter.value);
                }

                let captured_values = inner_self.free_variables.analyze_function(self.ast, id);

                for capture in captured_values.iter().copied() {
                    if inner_self.env.lookup_in_parent(capture.value).is_some() {
                        inner_self.env.declare_local(capture.value);
                    } else {
                        return Err(report_error!(
                            capture.span,
                            inner_self.compiler.path,
                            "undeclared variable",
                        ));
                    }
                }

                let src = inner_self.lower_expression(block, None)?;

                if !inner_self.block_returns(block) {
                    inner_self
                        .function
                        .emit_instruction(Instruction::Return { src: src.into() });
                }

                inner_self.patch_arguments();

                *self.env = std::mem::take(&mut env.parent.unwrap_or_default());

                let (_, dest) = self
                    .env
                    .lookup(name.value)
                    .expect("function must be declared");

                let index = self.compiler.functions.len();
                self.compiler.functions.push(function);

                let src = self.function.store_function_const(index);

                self.function.emit_instruction(Instruction::LoadConst {
                    dest: dest.into(),
                    src,
                });

                if !captured_values.is_empty() {
                    self.function.emit_instruction(Instruction::CreateClosure {
                        dest: dest.into(),
                        captures: captured_values.len() as u8,
                    });
                }

                for capture in captured_values.iter().copied() {
                    let (_, register) = self
                        .env
                        .lookup(capture.value)
                        .expect("name must've been declared to reach this point of the code");

                    self.function.emit_instruction(Instruction::CaptureValue {
                        src: register.into(),
                    });
                }
            }
            Node::Break => todo!(),
            Node::Continue => todo!(),
            _ => {
                let register = self.lower_expression(id, None)?;
                self.env.free_temp(register);
            }
        }

        Ok(())
    }

    fn lower_expression(&mut self, id: NodeId, dest: Option<Register>) -> Result<Register, Error> {
        let register = match *self.ast.node(id) {
            Node::Number(value) => {
                let src = self.function.store_number_const(value);
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                self.function.emit_instruction(Instruction::LoadConst {
                    dest: dest.into(),
                    src,
                });

                dest
            }
            Node::String(value) => {
                let src = self.function.store_string_const(value);
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                self.function.emit_instruction(Instruction::LoadConst {
                    dest: dest.into(),
                    src,
                });

                dest
            }
            Node::Boolean(value) => {
                let src = self.function.store_boolean_const(value);
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                self.function.emit_instruction(Instruction::LoadConst {
                    dest: dest.into(),
                    src,
                });

                dest
            }
            Node::Nil => {
                let src = self.function.store_nil_const();
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                self.function.emit_instruction(Instruction::LoadConst {
                    dest: dest.into(),
                    src,
                });

                dest
            }
            Node::Identifier(name) => {
                let Some((_, register)) = self.env.lookup(name.value) else {
                    let slice = INTERNER.lock().unwrap().resolve(name.value);

                    return Err(report_error!(
                        name.span,
                        self.compiler.path,
                        "{} is not declared",
                        slice
                    ));
                };

                match dest {
                    Some(dest) if dest == register => dest,
                    Some(dest) => {
                        self.function.emit_instruction(Instruction::Move {
                            dest: dest.into(),
                            src: register.into(),
                        });

                        dest
                    }
                    None => register,
                }
            }
            Node::Binary {
                operator,
                left,
                right,
            } => {
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                if let Some(src2) = self.as_number_const(right) {
                    let src1 = self.lower_expression(left, None)?;

                    self.function.emit_instruction(match operator {
                        BinaryOp::Add => Instruction::AddK {
                            dest: dest.into(),
                            src1: src1.into(),
                            src2,
                        },
                        BinaryOp::Subtract => Instruction::SubtractRK {
                            dest: dest.into(),
                            src1: src1.into(),
                            src2,
                        },
                        BinaryOp::Multiply => Instruction::MultiplyK {
                            dest: dest.into(),
                            src1: src1.into(),
                            src2,
                        },
                        BinaryOp::Divide => Instruction::DivideRK {
                            dest: dest.into(),
                            src1: src1.into(),
                            src2,
                        },
                        BinaryOp::Modulo => Instruction::ModuloRK {
                            dest: dest.into(),
                            src1: src1.into(),
                            src2,
                        },
                        BinaryOp::Equal => Instruction::EqualK {
                            dest: dest.into(),
                            src1: src1.into(),
                            src2,
                        },
                        BinaryOp::NotEqual => Instruction::NotEqualK {
                            dest: dest.into(),
                            src1: src1.into(),
                            src2,
                        },
                        BinaryOp::Less => Instruction::LessK {
                            dest: dest.into(),
                            src1: src1.into(),
                            src2,
                        },
                        BinaryOp::LessEqual => Instruction::LessEqualK {
                            dest: dest.into(),
                            src1: src1.into(),
                            src2,
                        },
                        BinaryOp::Greater => Instruction::GreaterK {
                            dest: dest.into(),
                            src1: src1.into(),
                            src2,
                        },
                        BinaryOp::GreaterEqual => Instruction::GreaterEqualK {
                            dest: dest.into(),
                            src1: src1.into(),
                            src2,
                        },
                    });

                    self.env.free_temp(src1);

                    dest
                } else if let Some(src1) = self.as_number_const(left) {
                    let src2 = self.lower_expression(right, None)?;

                    self.function.emit_instruction(match operator {
                        BinaryOp::Add => Instruction::AddK {
                            dest: dest.into(),
                            src1: src2.into(),
                            src2: src1,
                        },
                        BinaryOp::Multiply => Instruction::MultiplyK {
                            dest: dest.into(),
                            src1: src2.into(),
                            src2: src1,
                        },
                        BinaryOp::Equal => Instruction::EqualK {
                            dest: dest.into(),
                            src1: src2.into(),
                            src2: src1,
                        },
                        BinaryOp::NotEqual => Instruction::NotEqualK {
                            dest: dest.into(),
                            src1: src2.into(),
                            src2: src1,
                        },
                        BinaryOp::Subtract => Instruction::SubtractKR {
                            dest: dest.into(),
                            src1,
                            src2: src2.into(),
                        },
                        BinaryOp::Divide => Instruction::DivideKR {
                            dest: dest.into(),
                            src1,
                            src2: src2.into(),
                        },
                        BinaryOp::Modulo => Instruction::ModuloKR {
                            dest: dest.into(),
                            src1,
                            src2: src2.into(),
                        },
                        BinaryOp::Less => Instruction::GreaterK {
                            dest: dest.into(),
                            src1: src2.into(),
                            src2: src1,
                        },
                        BinaryOp::LessEqual => Instruction::GreaterEqualK {
                            dest: dest.into(),
                            src1: src2.into(),
                            src2: src1,
                        },
                        BinaryOp::Greater => Instruction::LessK {
                            dest: dest.into(),
                            src1: src2.into(),
                            src2: src1,
                        },
                        BinaryOp::GreaterEqual => Instruction::LessEqualK {
                            dest: dest.into(),
                            src1: src2.into(),
                            src2: src1,
                        },
                    });

                    self.env.free_temp(src2);

                    dest
                } else {
                    let src1 = self.lower_expression(left, None)?;
                    let src2 = self.lower_expression(right, None)?;

                    self.function.emit_instruction(match operator {
                        BinaryOp::Add => Instruction::Add {
                            dest: dest.into(),
                            src1: src1.into(),
                            src2: src2.into(),
                        },
                        BinaryOp::Subtract => Instruction::Subtract {
                            dest: dest.into(),
                            src1: src1.into(),
                            src2: src2.into(),
                        },
                        BinaryOp::Multiply => Instruction::Multiply {
                            dest: dest.into(),
                            src1: src1.into(),
                            src2: src2.into(),
                        },
                        BinaryOp::Divide => Instruction::Divide {
                            dest: dest.into(),
                            src1: src1.into(),
                            src2: src2.into(),
                        },
                        BinaryOp::Modulo => Instruction::Modulo {
                            dest: dest.into(),
                            src1: src1.into(),
                            src2: src2.into(),
                        },
                        BinaryOp::Equal => Instruction::Equal {
                            dest: dest.into(),
                            src1: src1.into(),
                            src2: src2.into(),
                        },
                        BinaryOp::NotEqual => Instruction::NotEqual {
                            dest: dest.into(),
                            src1: src1.into(),
                            src2: src2.into(),
                        },
                        BinaryOp::Less => Instruction::Less {
                            dest: dest.into(),
                            src1: src1.into(),
                            src2: src2.into(),
                        },
                        BinaryOp::LessEqual => Instruction::LessEqual {
                            dest: dest.into(),
                            src1: src1.into(),
                            src2: src2.into(),
                        },
                        BinaryOp::Greater => Instruction::Greater {
                            dest: dest.into(),
                            src1: src1.into(),
                            src2: src2.into(),
                        },
                        BinaryOp::GreaterEqual => Instruction::GreaterEqual {
                            dest: dest.into(),
                            src1: src1.into(),
                            src2: src2.into(),
                        },
                    });

                    self.env.free_temp(src1);
                    self.env.free_temp(src2);

                    dest
                }
            }
            Node::Unary { operator, operand } => {
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());
                let src = self.lower_expression(operand, None)?;

                self.function.emit_instruction(match operator {
                    UnaryOp::Negate => Instruction::Negate {
                        dest: dest.into(),
                        src: src.into(),
                    },
                    UnaryOp::Deref => Instruction::Deref {
                        dest: dest.into(),
                        src: src.into(),
                    },
                });

                self.env.free_temp(src);

                dest
            }
            Node::LogicalNot(expression) => {
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());
                let src = self.lower_expression(expression, None)?;

                self.function.emit_instruction(Instruction::Not {
                    dest: dest.into(),
                    src: src.into(),
                });

                self.env.free_temp(src);

                dest
            }
            Node::LogicalAnd { left, right } => {
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                self.lower_expression(left, Some(dest))?;

                let jump_if_false = self.lower_jump_if_false(dest);

                self.lower_expression(right, Some(dest))?;

                self.patch_jump(
                    jump_if_false,
                    self.function.instructions.len() as i32 - jump_if_false as i32,
                );

                dest
            }
            Node::LogicalOr { left, right } => {
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                self.lower_expression(left, Some(dest))?;

                let jump_if_true = self.lower_jump_if_true(dest);

                self.lower_expression(right, Some(dest))?;

                self.patch_jump(
                    jump_if_true,
                    self.function.instructions.len() as i32 - jump_if_true as i32,
                );

                dest
            }
            Node::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let src = self.lower_expression(condition, None)?;
                self.env.free_temp(src);

                let jump_if_false = self.lower_jump_if_false(src);

                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                self.lower_expression(then_branch, Some(dest))?;

                let jump_end = self
                    .function
                    .emit_instruction(Instruction::Jump { offset: 0 });

                self.patch_jump(
                    jump_if_false,
                    self.function.instructions.len() as i32 - jump_if_false as i32,
                );

                self.lower_expression(else_branch, Some(dest))?;

                self.patch_jump(
                    jump_end,
                    self.function.instructions.len() as i32 - jump_end as i32,
                );

                dest
            }
            Node::Block {
                ref statements,
                tail,
            } => {
                self.env.push_scope();

                for id in statements.iter().copied() {
                    if let Node::Function { name, .. } = self.ast.node(id) {
                        self.env.declare_local(name.value);
                    }
                }

                if let Some(id) = tail
                    && let Node::Function { name, .. } = self.ast.node(id)
                {
                    self.env.declare_local(name.value);
                }

                for id in statements.iter().copied() {
                    self.lower_statement(id)?;
                }

                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                match tail {
                    Some(id) => {
                        self.lower_expression(id, Some(dest))?;
                    }
                    None => {
                        let src = self.function.store_nil_const();

                        self.function.emit_instruction(Instruction::LoadConst {
                            dest: dest.into(),
                            src,
                        });
                    }
                };

                self.env.pop_scope();

                dest
            }
            Node::FunctionCall {
                callee,
                ref arguments,
            } => {
                for (index, argument) in arguments.iter().enumerate() {
                    let dest = Register::Local(index + 1);

                    self.lower_expression(*argument, Some(dest))?;

                    let index = self.function.instructions.len() - 1;

                    self.unpatched_arguments.push(index);
                }

                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());
                let src = self.lower_expression(callee, None)?;
                self.function.emit_instruction(Instruction::Call {
                    dest: dest.into(),
                    src: src.into(),
                });

                self.env.free_temp(src);

                dest
            }
            Node::MemberAccess { object, property } => {
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                let object = self.lower_expression(object, None)?;

                let key = {
                    let dest = self.env.allocate_temp();
                    let src = self.function.store_string_const(property.value);

                    self.function.emit_instruction(Instruction::LoadConst {
                        dest: dest.into(),
                        src,
                    });

                    dest
                };

                self.function.emit_instruction(Instruction::GetField {
                    dest: dest.into(),
                    object: object.into(),
                    key: key.into(),
                });

                self.env.free_temp(object);
                self.env.free_temp(key);

                dest
            }
            Node::Map { ref entries } => {
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                self.function
                    .emit_instruction(Instruction::CreateMap { dest: dest.into() });

                for (key, value) in entries.iter().copied() {
                    let key = self.lower_expression(key, None)?;
                    let value = self.lower_expression(value, None)?;

                    self.function.emit_instruction(Instruction::SetField {
                        object: dest.into(),
                        key: key.into(),
                        value: value.into(),
                    });

                    self.env.free_temp(key);
                    self.env.free_temp(value);
                }

                dest
            }
            Node::Lambda {
                ref parameters,
                block,
            } => {
                let mut function = Function::new(parameters.len());
                let mut env = Environment::with_parent(std::mem::take(self.env));
                let mut inner_self = Lower::new(
                    self.ast,
                    self.compiler,
                    self.free_variables,
                    &mut env,
                    &mut function,
                );
                for parameter in parameters.iter().copied() {
                    inner_self.env.declare_local(parameter.value);
                }

                let captured_values = inner_self.free_variables.analyze_function(self.ast, id);

                for capture in captured_values.iter().copied() {
                    if inner_self.env.lookup_in_parent(capture.value).is_some() {
                        inner_self.env.declare_local(capture.value);
                    } else {
                        return Err(report_error!(
                            capture.span,
                            self.compiler.path,
                            "undeclared variable",
                        ));
                    }
                }

                let src = inner_self.lower_expression(block, None)?;

                if !inner_self.block_returns(block) {
                    inner_self
                        .function
                        .emit_instruction(Instruction::Return { src: src.into() });
                }

                inner_self.patch_arguments();

                *self.env = std::mem::take(&mut env.parent.unwrap_or_default());

                let index = self.compiler.functions.len();
                self.compiler.functions.push(function);

                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                let src = self.function.store_function_const(index);

                self.function.emit_instruction(Instruction::LoadConst {
                    dest: dest.into(),
                    src,
                });

                if !captured_values.is_empty() {
                    self.function.emit_instruction(Instruction::CreateClosure {
                        dest: dest.into(),
                        captures: captured_values.len() as u8,
                    });
                }

                for capture in captured_values.iter().copied() {
                    let (_, register) = self
                        .env
                        .lookup(capture.value)
                        .expect("name must've been declared to reach this point of the code");

                    self.function.emit_instruction(Instruction::CaptureValue {
                        src: register.into(),
                    });
                }

                dest
            }
            Node::Import { path } => {
                let index = self.compiler.compile_file(path)?;

                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());
                let src = self.function.store_function_const(index);

                self.function.emit_instruction(Instruction::LoadConst {
                    dest: dest.into(),
                    src,
                });
                self.function.emit_instruction(Instruction::Call {
                    dest: dest.into(),
                    src: dest.into(),
                });

                dest
            }
            Node::WhileLoop { .. }
            | Node::Function { .. }
            | Node::Return(..)
            | Node::Break
            | Node::Continue
            | Node::Variable { .. }
            | Node::Constant { .. }
            | Node::Ref { .. }
            | Node::Assign { .. } => {
                self.lower_statement(id)?;

                let src = self.function.store_nil_const();
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                self.function.emit_instruction(Instruction::LoadConst {
                    dest: dest.into(),
                    src,
                });

                dest
            }
        };

        Ok(register)
    }

    fn as_number_const(&mut self, id: NodeId) -> Option<Const> {
        match *self.ast.node(id) {
            Node::Number(value) => Some(self.function.store_number_const(value)),
            _ => None,
        }
    }

    fn lower_jump_if_true(&mut self, register: Register) -> usize {
        let last = self.function.instructions.last().copied();

        match last {
            Some(Instruction::Equal { src1, src2, .. }) => {
                self.function.instructions.pop();
                self.function.emit_instruction(Instruction::JumpIfEqual {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::NotEqual { src1, src2, .. }) => {
                self.function.instructions.pop();
                self.function.emit_instruction(Instruction::JumpIfNotEqual {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::Less { src1, src2, .. }) => {
                self.function.instructions.pop();
                self.function.emit_instruction(Instruction::JumpIfLess {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::LessEqual { src1, src2, .. }) => {
                self.function.instructions.pop();
                self.function
                    .emit_instruction(Instruction::JumpIfLessEqual {
                        src1,
                        src2,
                        offset: 0,
                    })
            }
            Some(Instruction::Greater { src1, src2, .. }) => {
                self.function.instructions.pop();
                self.function.emit_instruction(Instruction::JumpIfGreater {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::GreaterEqual { src1, src2, .. }) => {
                self.function.instructions.pop();
                self.function
                    .emit_instruction(Instruction::JumpIfGreaterEqual {
                        src1,
                        src2,
                        offset: 0,
                    })
            }
            Some(Instruction::EqualK { src1, src2, .. }) => {
                self.function.instructions.pop();
                self.function.emit_instruction(Instruction::JumpIfEqualK {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::NotEqualK { src1, src2, .. }) => {
                self.function.instructions.pop();
                self.function
                    .emit_instruction(Instruction::JumpIfNotEqualK {
                        src1,
                        src2,
                        offset: 0,
                    })
            }
            Some(Instruction::LessK { src1, src2, .. }) => {
                self.function.instructions.pop();
                self.function.emit_instruction(Instruction::JumpIfLessK {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::LessEqualK { src1, src2, .. }) => {
                self.function.instructions.pop();
                self.function
                    .emit_instruction(Instruction::JumpIfLessEqualK {
                        src1,
                        src2,
                        offset: 0,
                    })
            }
            Some(Instruction::GreaterK { src1, src2, .. }) => {
                self.function.instructions.pop();
                self.function.emit_instruction(Instruction::JumpIfGreaterK {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::GreaterEqualK { src1, src2, .. }) => {
                self.function.instructions.pop();
                self.function
                    .emit_instruction(Instruction::JumpIfGreaterEqualK {
                        src1,
                        src2,
                        offset: 0,
                    })
            }
            _ => self.function.emit_instruction(Instruction::JumpIfTrue {
                src: register.into(),
                offset: 0,
            }),
        }
    }

    fn lower_jump_if_false(&mut self, register: Register) -> usize {
        let last = self.function.instructions.last().copied();

        match last {
            Some(Instruction::Equal { src1, src2, .. }) => {
                self.function.instructions.pop();
                self.function.emit_instruction(Instruction::JumpIfNotEqual {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::NotEqual { src1, src2, .. }) => {
                self.function.instructions.pop();
                self.function.emit_instruction(Instruction::JumpIfEqual {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::Less { src1, src2, .. }) => {
                self.function.instructions.pop();
                self.function
                    .emit_instruction(Instruction::JumpIfGreaterEqual {
                        src1,
                        src2,
                        offset: 0,
                    })
            }
            Some(Instruction::LessEqual { src1, src2, .. }) => {
                self.function.instructions.pop();
                self.function.emit_instruction(Instruction::JumpIfGreater {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::Greater { src1, src2, .. }) => {
                self.function.instructions.pop();
                self.function
                    .emit_instruction(Instruction::JumpIfLessEqual {
                        src1,
                        src2,
                        offset: 0,
                    })
            }
            Some(Instruction::GreaterEqual { src1, src2, .. }) => {
                self.function.instructions.pop();
                self.function.emit_instruction(Instruction::JumpIfLess {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::EqualK { src1, src2, .. }) => {
                self.function.instructions.pop();
                self.function
                    .emit_instruction(Instruction::JumpIfNotEqualK {
                        src1,
                        src2,
                        offset: 0,
                    })
            }
            Some(Instruction::NotEqualK { src1, src2, .. }) => {
                self.function.instructions.pop();
                self.function.emit_instruction(Instruction::JumpIfEqualK {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::LessK { src1, src2, .. }) => {
                self.function.instructions.pop();
                self.function
                    .emit_instruction(Instruction::JumpIfGreaterEqualK {
                        src1,
                        src2,
                        offset: 0,
                    })
            }
            Some(Instruction::LessEqualK { src1, src2, .. }) => {
                self.function.instructions.pop();
                self.function.emit_instruction(Instruction::JumpIfGreaterK {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::GreaterK { src1, src2, .. }) => {
                self.function.instructions.pop();
                self.function
                    .emit_instruction(Instruction::JumpIfLessEqualK {
                        src1,
                        src2,
                        offset: 0,
                    })
            }
            Some(Instruction::GreaterEqualK { src1, src2, .. }) => {
                self.function.instructions.pop();
                self.function.emit_instruction(Instruction::JumpIfLessK {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            _ => self.function.emit_instruction(Instruction::JumpIfFalse {
                src: register.into(),
                offset: 0,
            }),
        }
    }

    fn patch_jump(&mut self, index: usize, new_offset: i32) {
        match &mut self.function.instructions[index] {
            Instruction::Jump { offset }
            | Instruction::JumpIfTrue { offset, .. }
            | Instruction::JumpIfFalse { offset, .. }
            | Instruction::JumpIfEqual { offset, .. }
            | Instruction::JumpIfNotEqual { offset, .. }
            | Instruction::JumpIfLess { offset, .. }
            | Instruction::JumpIfLessK { offset, .. }
            | Instruction::JumpIfLessEqual { offset, .. }
            | Instruction::JumpIfLessEqualK { offset, .. }
            | Instruction::JumpIfGreater { offset, .. }
            | Instruction::JumpIfGreaterK { offset, .. }
            | Instruction::JumpIfGreaterEqual { offset, .. }
            | Instruction::JumpIfGreaterEqualK { offset, .. }
            | Instruction::JumpIfEqualK { offset, .. }
            | Instruction::JumpIfNotEqualK { offset, .. } => *offset = new_offset,
            _ => panic!("tried to patch a non-jump instruction at index {index}"),
        }
    }

    fn patch_arguments(&mut self) {
        self.function.frame_size = self.env.frame_size;

        for index in self.unpatched_arguments.iter().copied() {
            match &mut self.function.instructions[index] {
                Instruction::Add { dest, .. }
                | Instruction::AddK { dest, .. }
                | Instruction::Subtract { dest, .. }
                | Instruction::SubtractRK { dest, .. }
                | Instruction::SubtractKR { dest, .. }
                | Instruction::Multiply { dest, .. }
                | Instruction::MultiplyK { dest, .. }
                | Instruction::Divide { dest, .. }
                | Instruction::DivideRK { dest, .. }
                | Instruction::DivideKR { dest, .. }
                | Instruction::Modulo { dest, .. }
                | Instruction::ModuloRK { dest, .. }
                | Instruction::ModuloKR { dest, .. }
                | Instruction::Equal { dest, .. }
                | Instruction::EqualK { dest, .. }
                | Instruction::NotEqual { dest, .. }
                | Instruction::NotEqualK { dest, .. }
                | Instruction::Less { dest, .. }
                | Instruction::LessK { dest, .. }
                | Instruction::LessEqual { dest, .. }
                | Instruction::LessEqualK { dest, .. }
                | Instruction::Greater { dest, .. }
                | Instruction::GreaterK { dest, .. }
                | Instruction::GreaterEqual { dest, .. }
                | Instruction::GreaterEqualK { dest, .. }
                | Instruction::Not { dest, .. }
                | Instruction::Negate { dest, .. }
                | Instruction::Move { dest, .. }
                | Instruction::LoadConst { dest, .. }
                | Instruction::CreateMap { dest }
                | Instruction::GetField { dest, .. }
                | Instruction::CreateClosure { dest, .. }
                | Instruction::CreateRef { dest, .. }
                | Instruction::Deref { dest, .. }
                | Instruction::Call { dest, .. } => {
                    let new_dest = Register::Temp(dest.0 as usize + self.function.frame_size);
                    *dest = new_dest.into();
                }
                _ => unreachable!("instruction at index {index} has no dest to patch"),
            }
        }
    }

    fn block_returns(&self, id: NodeId) -> bool {
        match *self.ast.node(id) {
            Node::Return(..) => true,
            Node::Block {
                ref statements,
                tail,
            } => {
                let statements = statements.iter().copied().any(|e| self.block_returns(e));
                let expression = if let Some(id) = tail {
                    self.block_returns(id)
                } else {
                    false
                };

                statements || expression
            }
            Node::If {
                then_branch,
                else_branch,
                ..
            } => self.block_returns(then_branch) && self.block_returns(else_branch),
            _ => false,
        }
    }

    fn prevent_return(&self, id: NodeId) -> Result<(), Error> {
        match *self.ast.node(id) {
            Node::Return(..) => {
                return Err(report_error!(
                    self.ast.span(id),
                    self.compiler.path,
                    "return is not allowed in the global scope"
                ));
            }
            Node::Block {
                ref statements,
                tail,
            } => {
                for id in statements.iter().copied() {
                    self.prevent_return(id)?;
                }

                if let Some(id) = tail {
                    self.prevent_return(id)?;
                }
            }
            Node::If {
                then_branch,
                else_branch,
                ..
            } => {
                self.prevent_return(then_branch)?;
                self.prevent_return(else_branch)?;
            }
            Node::WhileLoop { block, .. } => self.prevent_return(block)?,
            _ => {}
        };

        Ok(())
    }
}
