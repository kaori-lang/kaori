use core::panic;

use crate::{
    bytecode::{
        collect_free_variables::collect_free_variables,
        environment::{Environment, Register},
        function::Function,
        instruction::{Const, Instruction},
    },
    compiler::{Compiler, INTERNER},
    diagnostics::error::Error,
    report_error,
    syntax::{
        ast::{Ast, Expr, ExprId},
        ops::{BinaryOp, UnaryOp},
    },
};

fn as_number_const(ast: &Ast, function: &mut Function, id: ExprId) -> Option<Const> {
    match *ast.node(id) {
        Expr::Number(value) => Some(function.store_number_const(value)),
        _ => None,
    }
}

pub fn lower_ast(ast: Ast, compiler: &mut Compiler) -> Result<usize, Error> {
    let id = ast.last();

    let mut env = Environment::new();
    let arity = 0;
    let mut function = Function::new(arity);
    let mut pending_args = Vec::new();

    let src = lower_expression(
        &ast,
        compiler,
        &mut function,
        &mut env,
        &mut pending_args,
        id,
        None,
    )?;

    if !block_returns(&ast, id) {
        function.emit_instruction(Instruction::Return { src: src.into() });
    }

    patch_pending_args(&mut function, &pending_args, env.frame_size);
    function.frame_size = env.frame_size;

    let entry_index = compiler.push_function(function);

    Ok(entry_index)
}

fn lower_effect(
    ast: &Ast,
    compiler: &mut Compiler,
    function: &mut Function,
    env: &mut Environment,
    pending_args: &mut Vec<usize>,
    id: ExprId,
) -> Result<(), Error> {
    match *ast.node(id) {
        Expr::Variable { left, right } => {
            let dest = env.declare_local(left.value);

            lower_expression(
                ast,
                compiler,
                function,
                env,
                pending_args,
                right,
                Some(dest),
            )?;
        }
        Expr::Ref { left, right } => {
            let dest = env.declare_local(left.value);

            let src = lower_expression(ast, compiler, function, env, pending_args, right, None)?;

            function.emit_instruction(Instruction::CreateRef {
                dest: dest.into(),
                src: src.into(),
            });
        }
        Expr::Assign { left, right } => match *ast.node(left) {
            Expr::Identifier(..) => {
                let dest =
                    lower_expression(ast, compiler, function, env, pending_args, left, None)?;

                let src = lower_expression(
                    ast,
                    compiler,
                    function,
                    env,
                    pending_args,
                    right,
                    Some(dest),
                )?;

                if src != dest {
                    env.free_temp(src);
                }
            }
            Expr::MemberAccess { object, property } => {
                let value =
                    lower_expression(ast, compiler, function, env, pending_args, right, None)?;
                let object =
                    lower_expression(ast, compiler, function, env, pending_args, object, None)?;
                let key = {
                    let dest = env.allocate_temp();
                    let src = function.store_string_const(property.value);

                    function.emit_instruction(Instruction::LoadConst {
                        dest: dest.into(),
                        src,
                    });

                    dest
                };

                function.emit_instruction(Instruction::SetField {
                    object: object.into(),
                    key: key.into(),
                    value: value.into(),
                });
            }
            Expr::Unary {
                operator: UnaryOp::Deref,
                operand,
            } => {
                let dest =
                    lower_expression(ast, compiler, function, env, pending_args, operand, None)?;
                let src =
                    lower_expression(ast, compiler, function, env, pending_args, right, None)?;

                function.emit_instruction(Instruction::DerefSet {
                    dest: dest.into(),
                    src: src.into(),
                });
            }
            _ => {
                return Err(report_error!(
                    ast.span(left),
                    compiler.path,
                    "expected a valid lhs"
                ));
            }
        },
        Expr::WhileLoop { condition, block } => {
            let src =
                lower_expression(ast, compiler, function, env, pending_args, condition, None)?;

            let jump_if_false = lower_jump_if_false(function, src);
            env.free_temp(src);

            let loop_body = function.instructions.len();
            lower_effect(ast, compiler, function, env, pending_args, block)?;

            let src =
                lower_expression(ast, compiler, function, env, pending_args, condition, None)?;

            let jump_if_true = lower_jump_if_true(function, src);
            env.free_temp(src);

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
        }
        Expr::Return(expression) => {
            let src =
                lower_expression(ast, compiler, function, env, pending_args, expression, None)?;

            function.emit_instruction(Instruction::Return { src: src.into() });

            env.free_temp(src);
        }
        Expr::Block {
            ref expressions,
            tail,
        } => {
            env.push_scope();

            for id in expressions.iter().copied() {
                if let Expr::Function {
                    name: Some(name), ..
                } = ast.node(id)
                {
                    env.declare_local(name.value);
                }
            }

            for id in expressions.iter().copied() {
                lower_effect(ast, compiler, function, env, pending_args, id)?;
            }

            if let Some(id) = tail {
                let span = ast.span(id);

                return Err(report_error!(
                    span,
                    compiler.path,
                    "expected `;` after expression, only block expressions can produce values"
                ));
            }

            env.pop_scope();
        }
        Expr::Break => todo!(),
        Expr::Continue => todo!(),
        _ => {
            let register = lower_expression(ast, compiler, function, env, pending_args, id, None)?;

            env.free_temp(register);
        }
    }

    Ok(())
}

fn lower_expression(
    ast: &Ast,
    compiler: &mut Compiler,
    function: &mut Function,
    env: &mut Environment,
    pending_args: &mut Vec<usize>,
    id: ExprId,
    dest: Option<Register>,
) -> Result<Register, Error> {
    let register = match *ast.node(id) {
        Expr::Number(value) => {
            let src = function.store_number_const(value);
            let dest = dest.unwrap_or_else(|| env.allocate_temp());

            function.emit_instruction(Instruction::LoadConst {
                dest: dest.into(),
                src,
            });

            dest
        }
        Expr::String(value) => {
            let src = function.store_string_const(value);
            let dest = dest.unwrap_or_else(|| env.allocate_temp());

            function.emit_instruction(Instruction::LoadConst {
                dest: dest.into(),
                src,
            });

            dest
        }
        Expr::Boolean(value) => {
            let src = function.store_boolean_const(value);
            let dest = dest.unwrap_or_else(|| env.allocate_temp());

            function.emit_instruction(Instruction::LoadConst {
                dest: dest.into(),
                src,
            });

            dest
        }
        Expr::Nil => {
            let src = function.store_nil_const();
            let dest = dest.unwrap_or_else(|| env.allocate_temp());

            function.emit_instruction(Instruction::LoadConst {
                dest: dest.into(),
                src,
            });

            dest
        }
        Expr::Identifier(name) => {
            let Some((_, register)) = env.lookup(name.value) else {
                let slice = INTERNER.lock().unwrap().resolve(name.value);

                return Err(report_error!(
                    name.span,
                    compiler.path,
                    "{} is not declared",
                    slice
                ));
            };

            match dest {
                Some(dest) if dest == register => dest,
                Some(dest) => {
                    function.emit_instruction(Instruction::Move {
                        dest: dest.into(),
                        src: register.into(),
                    });

                    dest
                }
                None => register,
            }
        }
        Expr::Binary {
            operator,
            left,
            right,
        } => {
            let dest = dest.unwrap_or_else(|| env.allocate_temp());

            if let Some(src2) = as_number_const(ast, function, right) {
                let src1 =
                    lower_expression(ast, compiler, function, env, pending_args, left, None)?;

                function.emit_instruction(match operator {
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

                env.free_temp(src1);

                dest
            } else if let Some(src1) = as_number_const(ast, function, left) {
                let src2 =
                    lower_expression(ast, compiler, function, env, pending_args, right, None)?;

                function.emit_instruction(match operator {
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

                env.free_temp(src2);

                dest
            } else {
                let src1 =
                    lower_expression(ast, compiler, function, env, pending_args, left, None)?;
                let src2 =
                    lower_expression(ast, compiler, function, env, pending_args, right, None)?;

                function.emit_instruction(match operator {
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

                env.free_temp(src1);
                env.free_temp(src2);

                dest
            }
        }
        Expr::Unary { operator, operand } => {
            let dest = dest.unwrap_or_else(|| env.allocate_temp());
            let src = lower_expression(ast, compiler, function, env, pending_args, operand, None)?;

            function.emit_instruction(match operator {
                UnaryOp::Negate => Instruction::Negate {
                    dest: dest.into(),
                    src: src.into(),
                },
                UnaryOp::Deref => Instruction::Deref {
                    dest: dest.into(),
                    src: src.into(),
                },
            });

            env.free_temp(src);

            dest
        }
        Expr::LogicalNot(expression) => {
            let dest = dest.unwrap_or_else(|| env.allocate_temp());

            let src =
                lower_expression(ast, compiler, function, env, pending_args, expression, None)?;

            function.emit_instruction(Instruction::Not {
                dest: dest.into(),
                src: src.into(),
            });

            env.free_temp(src);

            dest
        }
        Expr::LogicalAnd { left, right } => {
            let dest = dest.unwrap_or_else(|| env.allocate_temp());

            lower_expression(ast, compiler, function, env, pending_args, left, Some(dest))?;

            let jump_if_false = lower_jump_if_false(function, dest);

            lower_expression(
                ast,
                compiler,
                function,
                env,
                pending_args,
                right,
                Some(dest),
            )?;

            patch_jump(
                function,
                jump_if_false,
                function.instructions.len() as i32 - jump_if_false as i32,
            );

            dest
        }
        Expr::LogicalOr { left, right } => {
            let dest = dest.unwrap_or_else(|| env.allocate_temp());

            lower_expression(ast, compiler, function, env, pending_args, left, Some(dest))?;

            let jump_if_true = lower_jump_if_true(function, dest);

            lower_expression(
                ast,
                compiler,
                function,
                env,
                pending_args,
                right,
                Some(dest),
            )?;

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
            let src =
                lower_expression(ast, compiler, function, env, pending_args, condition, None)?;
            env.free_temp(src);

            let jump_if_false = lower_jump_if_false(function, src);

            let dest = dest.unwrap_or_else(|| env.allocate_temp());

            lower_expression(
                ast,
                compiler,
                function,
                env,
                pending_args,
                then_branch,
                Some(dest),
            )?;

            let jump_end = function.emit_instruction(Instruction::Jump { offset: 0 });

            patch_jump(
                function,
                jump_if_false,
                function.instructions.len() as i32 - jump_if_false as i32,
            );

            lower_expression(
                ast,
                compiler,
                function,
                env,
                pending_args,
                else_branch,
                Some(dest),
            )?;

            patch_jump(
                function,
                jump_end,
                function.instructions.len() as i32 - jump_end as i32,
            );

            dest
        }
        Expr::Block {
            ref expressions,
            tail,
        } => {
            env.push_scope();

            for id in expressions.iter().copied() {
                if let Expr::Function {
                    name: Some(name), ..
                } = ast.node(id)
                {
                    env.declare_local(name.value);
                }
            }

            for id in expressions.iter().copied() {
                lower_effect(ast, compiler, function, env, pending_args, id)?;
            }

            let dest = match tail {
                Some(id) => lower_expression(ast, compiler, function, env, pending_args, id, dest)?,
                None => {
                    let src = function.store_nil_const();
                    let dest = dest.unwrap_or_else(|| env.allocate_temp());

                    function.emit_instruction(Instruction::LoadConst {
                        dest: dest.into(),
                        src,
                    });

                    dest
                }
            };

            env.pop_scope();

            dest
        }
        Expr::Function {
            ref parameters,
            block,
            name,
        } => {
            let arity = parameters.len();
            let mut inner_function = Function::new(arity);
            let mut inner_env = Environment::with_parent(std::mem::take(env));
            let mut pending_args = Vec::new();

            if let Some(name) = name {
                inner_env.declare_function(name.value);
            }

            for parameter in parameters.iter().copied() {
                inner_env.declare_local(parameter.value);
            }

            let free_variables = collect_free_variables(ast, id);

            for name in free_variables.iter().copied() {
                if inner_env.lookup_in_parent(name.value).is_some() {
                    inner_env.declare_local(name.value);
                } else {
                    let slice = INTERNER.lock().unwrap().resolve(name.value);

                    return Err(report_error!(
                        name.span,
                        compiler.path,
                        "{} is not declared",
                        slice
                    ));
                }
            }

            let src = lower_expression(
                ast,
                compiler,
                &mut inner_function,
                &mut inner_env,
                &mut pending_args,
                block,
                None,
            )?;

            if !block_returns(ast, block) {
                inner_function.emit_instruction(Instruction::Return { src: src.into() });
            }

            patch_pending_args(&mut inner_function, &pending_args, inner_env.frame_size);
            inner_function.frame_size = inner_env.frame_size;
            let index = compiler.push_function(inner_function);

            *env = std::mem::take(&mut inner_env.parent.unwrap_or_default());

            let dest = match name {
                Some(name) => {
                    let (_, register) = env.lookup(name.value).expect("function must be declared");

                    register
                }
                None => dest.unwrap_or_else(|| env.allocate_temp()),
            };

            let src = function.store_function_const(index);

            function.emit_instruction(Instruction::LoadConst {
                dest: dest.into(),
                src,
            });

            // CREATE CLOSURE IF ANY NAME WAS CAPTURED OVERRIDING THE LOADED CONST
            if !free_variables.is_empty() {
                function.emit_instruction(Instruction::CreateClosure {
                    dest: dest.into(),
                    captures: free_variables.len() as u8,
                });
            }

            for name in free_variables.iter().copied() {
                let (_, register) = env
                    .lookup(name.value)
                    .expect("name must've been declared to reach this point of the code");

                function.emit_instruction(Instruction::CaptureValue {
                    src: register.into(),
                });
            }

            dest
        }
        Expr::FunctionCall {
            callee,
            ref arguments,
        } => {
            let src = lower_expression(ast, compiler, function, env, pending_args, callee, None)?;

            for (index, argument) in arguments.iter().enumerate() {
                let dest = Register::Local(index + 1);

                lower_expression(
                    ast,
                    compiler,
                    function,
                    env,
                    pending_args,
                    *argument,
                    Some(dest),
                )?;

                let index = function.instructions.len() - 1;

                pending_args.push(index);
            }

            let dest = dest.unwrap_or_else(|| env.allocate_temp());

            function.emit_instruction(Instruction::Call {
                dest: dest.into(),
                src: src.into(),
            });

            env.free_temp(src);

            dest
        }
        Expr::MemberAccess { object, property } => {
            let dest = dest.unwrap_or_else(|| env.allocate_temp());

            let object =
                lower_expression(ast, compiler, function, env, pending_args, object, None)?;

            let key = {
                let dest = env.allocate_temp();
                let src = function.store_string_const(property.value);

                function.emit_instruction(Instruction::LoadConst {
                    dest: dest.into(),
                    src,
                });

                dest
            };

            function.emit_instruction(Instruction::GetField {
                dest: dest.into(),
                object: object.into(),
                key: key.into(),
            });

            env.free_temp(object);
            env.free_temp(key);

            dest
        }
        Expr::Map { ref entries } => {
            let dest = dest.unwrap_or_else(|| env.allocate_temp());

            function.emit_instruction(Instruction::CreateMap { dest: dest.into() });

            for (key, value) in entries.iter().copied() {
                let key = lower_expression(ast, compiler, function, env, pending_args, key, None)?;

                let value =
                    lower_expression(ast, compiler, function, env, pending_args, value, None)?;

                function.emit_instruction(Instruction::SetField {
                    object: dest.into(),
                    key: key.into(),
                    value: value.into(),
                });

                env.free_temp(key);
                env.free_temp(value);
            }

            dest
        }
        Expr::Import { path } => {
            let index = compiler.compile_file(path)?;

            let dest = dest.unwrap_or_else(|| env.allocate_temp());
            let src = function.store_function_const(index);

            function.emit_instruction(Instruction::LoadConst {
                dest: dest.into(),
                src,
            });

            function.emit_instruction(Instruction::Call {
                dest: dest.into(),
                src: dest.into(),
            });

            dest
        }
        Expr::Variable { .. }
        | Expr::Ref { .. }
        | Expr::Assign { .. }
        | Expr::WhileLoop { .. }
        | Expr::Return(..)
        | Expr::Break
        | Expr::Continue => {
            return Err(report_error!(
                ast.span(id),
                compiler.path,
                "expression does not produce a value and cannot be used in value position"
            ));
        }
    };

    Ok(register)
}

fn block_returns(ast: &Ast, id: ExprId) -> bool {
    match *ast.node(id) {
        Expr::Return(..) => true,
        Expr::Block {
            ref expressions,
            tail,
        } => {
            let expressions = expressions.iter().copied().any(|e| block_returns(ast, e));
            let tail = if let Some(id) = tail {
                block_returns(ast, id)
            } else {
                false
            };

            expressions || tail
        }
        Expr::If {
            then_branch,
            else_branch,
            ..
        } => block_returns(ast, then_branch) && block_returns(ast, else_branch),
        _ => false,
    }
}

fn lower_jump_if_true(function: &mut Function, register: Register) -> usize {
    let last = function.instructions.last().copied();

    match last {
        Some(Instruction::Equal { src1, src2, .. }) => {
            function.instructions.pop();
            function.emit_instruction(Instruction::JumpIfEqual {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::NotEqual { src1, src2, .. }) => {
            function.instructions.pop();
            function.emit_instruction(Instruction::JumpIfNotEqual {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::Less { src1, src2, .. }) => {
            function.instructions.pop();
            function.emit_instruction(Instruction::JumpIfLess {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::LessEqual { src1, src2, .. }) => {
            function.instructions.pop();
            function.emit_instruction(Instruction::JumpIfLessEqual {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::Greater { src1, src2, .. }) => {
            function.instructions.pop();
            function.emit_instruction(Instruction::JumpIfGreater {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::GreaterEqual { src1, src2, .. }) => {
            function.instructions.pop();
            function.emit_instruction(Instruction::JumpIfGreaterEqual {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::EqualK { src1, src2, .. }) => {
            function.instructions.pop();
            function.emit_instruction(Instruction::JumpIfEqualK {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::NotEqualK { src1, src2, .. }) => {
            function.instructions.pop();
            function.emit_instruction(Instruction::JumpIfNotEqualK {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::LessK { src1, src2, .. }) => {
            function.instructions.pop();
            function.emit_instruction(Instruction::JumpIfLessK {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::LessEqualK { src1, src2, .. }) => {
            function.instructions.pop();
            function.emit_instruction(Instruction::JumpIfLessEqualK {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::GreaterK { src1, src2, .. }) => {
            function.instructions.pop();
            function.emit_instruction(Instruction::JumpIfGreaterK {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::GreaterEqualK { src1, src2, .. }) => {
            function.instructions.pop();
            function.emit_instruction(Instruction::JumpIfGreaterEqualK {
                src1,
                src2,
                offset: 0,
            })
        }
        _ => function.emit_instruction(Instruction::JumpIfTrue {
            src: register.into(),
            offset: 0,
        }),
    }
}

fn lower_jump_if_false(function: &mut Function, register: Register) -> usize {
    let last = function.instructions.last().copied();

    match last {
        Some(Instruction::Equal { src1, src2, .. }) => {
            function.instructions.pop();
            function.emit_instruction(Instruction::JumpIfNotEqual {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::NotEqual { src1, src2, .. }) => {
            function.instructions.pop();
            function.emit_instruction(Instruction::JumpIfEqual {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::Less { src1, src2, .. }) => {
            function.instructions.pop();
            function.emit_instruction(Instruction::JumpIfGreaterEqual {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::LessEqual { src1, src2, .. }) => {
            function.instructions.pop();
            function.emit_instruction(Instruction::JumpIfGreater {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::Greater { src1, src2, .. }) => {
            function.instructions.pop();
            function.emit_instruction(Instruction::JumpIfLessEqual {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::GreaterEqual { src1, src2, .. }) => {
            function.instructions.pop();
            function.emit_instruction(Instruction::JumpIfLess {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::EqualK { src1, src2, .. }) => {
            function.instructions.pop();
            function.emit_instruction(Instruction::JumpIfNotEqualK {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::NotEqualK { src1, src2, .. }) => {
            function.instructions.pop();
            function.emit_instruction(Instruction::JumpIfEqualK {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::LessK { src1, src2, .. }) => {
            function.instructions.pop();
            function.emit_instruction(Instruction::JumpIfGreaterEqualK {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::LessEqualK { src1, src2, .. }) => {
            function.instructions.pop();
            function.emit_instruction(Instruction::JumpIfGreaterK {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::GreaterK { src1, src2, .. }) => {
            function.instructions.pop();
            function.emit_instruction(Instruction::JumpIfLessEqualK {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::GreaterEqualK { src1, src2, .. }) => {
            function.instructions.pop();
            function.emit_instruction(Instruction::JumpIfLessK {
                src1,
                src2,
                offset: 0,
            })
        }
        _ => function.emit_instruction(Instruction::JumpIfFalse {
            src: register.into(),
            offset: 0,
        }),
    }
}

fn patch_jump(function: &mut Function, index: usize, new_offset: i32) {
    match &mut function.instructions[index] {
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

fn patch_pending_args(function: &mut Function, pending_args: &[usize], frame_size: usize) {
    for index in pending_args.iter().copied() {
        match &mut function.instructions[index] {
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
                let new_dest = Register::Temp(dest.0 as usize + frame_size);
                *dest = new_dest.into();
            }
            _ => unreachable!("instruction at index {index} has no dest to patch"),
        }
    }
}
