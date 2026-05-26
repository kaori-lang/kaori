use core::panic;

use crate::{
    bytecode::{
        collect_free_variables::collect_free_variables,
        environment::{Environment, Local, LocalKind},
        function::Function,
        instruction::{Const, Instruction},
        register_allocator::{Register, RegisterAllocator},
    },
    diagnostics::error::Error,
    interpreter::INTERNER,
    report_error,
    syntax::{
        ast::{Ast, AstNode, NodeId},
        ops::{BinaryOp, CompoundOp, UnaryOp},
    },
};

fn as_number_const(ast: &Ast, function: &mut Function, id: NodeId) -> Option<Const> {
    match *ast.get_node(id) {
        AstNode::NumberLiteral(value) => Some(function.store_number_const(value)),
        _ => None,
    }
}

pub fn lower_ast(ast: Ast) -> Result<Vec<Function>, Error> {
    let id = ast.last();

    let mut functions = Vec::new();
    functions.push(None);

    let mut env = Environment::new();
    let mut function = Function::new(0);
    let mut regalloc = RegisterAllocator::default();

    let src = lower_expression(
        &ast,
        &mut functions,
        &mut function,
        &mut env,
        &mut regalloc,
        id,
        None,
    )?;

    if !block_returns(&ast, id) {
        function.emit_instruction(Instruction::Return { src: src.into() });
    }

    functions[0] = Some(function);

    Ok(functions.into_iter().map(|f| f.unwrap()).collect())
}

fn lower_block(
    ast: &Ast,
    functions: &mut Vec<Option<Function>>,
    function: &mut Function,
    env: &mut Environment,
    regalloc: &mut RegisterAllocator,
    ids: &[NodeId],
    dest: Register,
) -> Result<Register, Error> {
    for id in ids.iter().copied() {
        if let AstNode::Function {
            name: Some(name), ..
        } = ast.get_node(id)
        {
            let register = regalloc.allocate_local();

            env.insert_local(Local {
                name: name.symbol,
                register,
                kind: LocalKind::Variable,
            });
        }
    }

    for id in ids.iter().copied() {
        lower_statement(ast, functions, function, env, regalloc, id)?;
    }

    if last.unwrap().register.is_none() {
        let src = function.store_nil_const();

        function.emit_instruction(Instruction::LoadConst {
            dest: dest.unwrap().into(),
            src,
        });
    }

    Ok(dest)
}

fn lower_statement(
    ast: &Ast,
    functions: &mut Vec<Option<Function>>,
    function: &mut Function,
    env: &mut Environment,
    regalloc: &mut RegisterAllocator,
    id: NodeId,
) -> Result<Option<Register>, Error> {
    let register = match *ast.get_node(id) {
        AstNode::Variable { left, right } => {
            let dest = regalloc.allocate_local();

            env.insert_local(Local {
                name: left.symbol,
                register: dest,
                kind: LocalKind::Variable,
            });

            lower_expression(ast, functions, function, env, regalloc, right, Some(dest))?;

            None
        }
        AstNode::Mut { left, right } => {
            let dest = regalloc.allocate_local();

            env.insert_local(Local {
                name: left.symbol,
                register: dest,
                kind: LocalKind::Mut,
            });

            lower_expression(ast, functions, function, env, regalloc, right, Some(dest))?;

            None
        }
        AstNode::WhileLoop { condition, block } => {
            let condition_register =
                lower_expression(ast, functions, function, env, regalloc, condition, None)?;

            let jump_if_false = lower_jump_if_false(function, condition_register);
            regalloc.free_temp(condition_register);

            let loop_body = function.instructions.len();
            lower_expression(ast, functions, function, env, regalloc, block, None)?;

            let condition_register =
                lower_expression(ast, functions, function, env, regalloc, condition, None)?;

            let jump_if_true = lower_jump_if_true(function, condition_register);
            regalloc.free_temp(condition_register);

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

            None
        }
        AstNode::Return(expression) => {
            let src = lower_expression(ast, functions, function, env, regalloc, expression, None)?;

            function.emit_instruction(Instruction::Return { src: src.into() });

            regalloc.free_temp(src);

            None
        }
        AstNode::Break => todo!(),
        AstNode::Continue => todo!(),
        AstNode::NativeFunction { .. } => todo!(),
        _ => {
            let register = lower_expression(ast, functions, function, env, regalloc, id, None)?;

            Some(register)
        }
    };

    Ok(register)
}

fn lower_expression(
    ast: &Ast,
    functions: &mut Vec<Option<Function>>,
    function: &mut Function,
    env: &mut Environment,
    regalloc: &mut RegisterAllocator,
    id: NodeId,
    dest: Option<Register>,
) -> Result<Register, Error> {
    let register = match *ast.get_node(id) {
        AstNode::NumberLiteral(value) => {
            let src = function.store_number_const(value);
            let dest = dest.unwrap_or_else(|| regalloc.allocate_temp());

            function.emit_instruction(Instruction::LoadConst {
                dest: dest.into(),
                src,
            });

            dest
        }
        AstNode::StringLiteral(value) => {
            let src = function.store_string_const(value);
            let dest = dest.unwrap_or_else(|| regalloc.allocate_temp());

            function.emit_instruction(Instruction::LoadConst {
                dest: dest.into(),
                src,
            });

            dest
        }
        AstNode::BooleanLiteral(value) => {
            let src = function.store_boolean_const(value);
            let dest = dest.unwrap_or_else(|| regalloc.allocate_temp());

            function.emit_instruction(Instruction::LoadConst {
                dest: dest.into(),
                src,
            });

            dest
        }
        AstNode::NilLiteral => {
            let src = function.store_nil_const();
            let dest = dest.unwrap_or_else(|| regalloc.allocate_temp());

            function.emit_instruction(Instruction::LoadConst {
                dest: dest.into(),
                src,
            });

            dest
        }
        AstNode::Identifier(name) => {
            let Some(Local { register, kind, .. }) = env.lookup(name.symbol) else {
                let slice = INTERNER.lock().unwrap().resolve(name.symbol);

                return Err(report_error!(name.span, "{} is not declared", slice));
            };

            let dest = match kind {
                LocalKind::Constant | LocalKind::Variable => {
                    if let Some(dest) = dest
                        && dest != register
                    {
                        function.emit_instruction(Instruction::Move {
                            dest: dest.into(),
                            src: register.into(),
                        });

                        dest
                    } else {
                        register
                    }
                }
                LocalKind::Mut => todo!(),
            };

            dest
        }
        AstNode::Assign { left, right } => {
            let dest = lower_expression(ast, functions, function, env, regalloc, left, None)?;

            let src = lower_expression(ast, functions, function, env, regalloc, right, Some(dest))?;

            if src != dest {
                regalloc.free_temp(src);
            }

            dest
        }
        AstNode::CompoundAssign {
            operator,
            left,
            right,
        } => {
            let dest = lower_expression(ast, functions, function, env, regalloc, left, None)?;

            if let Some(src2) = as_number_const(ast, function, right) {
                function.emit_instruction(match operator {
                    CompoundOp::Add => Instruction::AddK {
                        dest: dest.into(),
                        src1: dest.into(),
                        src2,
                    },
                    CompoundOp::Subtract => Instruction::SubtractRK {
                        dest: dest.into(),
                        src1: dest.into(),
                        src2,
                    },
                    CompoundOp::Multiply => Instruction::MultiplyK {
                        dest: dest.into(),
                        src1: dest.into(),
                        src2,
                    },
                    CompoundOp::Divide => Instruction::DivideRK {
                        dest: dest.into(),
                        src1: dest.into(),
                        src2,
                    },
                    CompoundOp::Modulo => Instruction::ModuloRK {
                        dest: dest.into(),
                        src1: dest.into(),
                        src2,
                    },
                });
            } else {
                let src2 = lower_expression(ast, functions, function, env, regalloc, right, None)?;

                function.emit_instruction(match operator {
                    CompoundOp::Add => Instruction::Add {
                        dest: dest.into(),
                        src1: dest.into(),
                        src2: src2.into(),
                    },
                    CompoundOp::Subtract => Instruction::Subtract {
                        dest: dest.into(),
                        src1: dest.into(),
                        src2: src2.into(),
                    },
                    CompoundOp::Multiply => Instruction::Multiply {
                        dest: dest.into(),
                        src1: dest.into(),
                        src2: src2.into(),
                    },
                    CompoundOp::Divide => Instruction::Divide {
                        dest: dest.into(),
                        src1: dest.into(),
                        src2: src2.into(),
                    },
                    CompoundOp::Modulo => Instruction::Modulo {
                        dest: dest.into(),
                        src1: dest.into(),
                        src2: src2.into(),
                    },
                });

                regalloc.free_temp(src2);
            }

            dest
        }
        AstNode::Binary {
            operator,
            left,
            right,
        } => {
            let dest = dest.unwrap_or_else(|| regalloc.allocate_temp());

            if let Some(src2) = as_number_const(ast, function, right) {
                let src1 = lower_expression(ast, functions, function, env, regalloc, left, None)?;

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

                regalloc.free_temp(src1);

                dest
            } else if let Some(src1) = as_number_const(ast, function, left) {
                let src2 = lower_expression(ast, functions, function, env, regalloc, right, None)?;

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

                regalloc.free_temp(src2);

                dest
            } else {
                let src1 = lower_expression(ast, functions, function, env, regalloc, left, None)?;

                let src2 = lower_expression(ast, functions, function, env, regalloc, right, None)?;

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

                regalloc.free_temp(src1);
                regalloc.free_temp(src2);

                dest
            }
        }
        AstNode::Unary { operator, right } => {
            let dest = dest.unwrap_or_else(|| regalloc.allocate_temp());

            let src = lower_expression(ast, functions, function, env, regalloc, right, None)?;

            function.emit_instruction(match operator {
                UnaryOp::Negate => Instruction::Negate {
                    dest: dest.into(),
                    src: src.into(),
                },
            });

            regalloc.free_temp(src);

            dest
        }
        AstNode::LogicalNot(expression) => {
            let dest = dest.unwrap_or_else(|| regalloc.allocate_temp());

            let src = lower_expression(ast, functions, function, env, regalloc, expression, None)?;

            function.emit_instruction(Instruction::Not {
                dest: dest.into(),
                src: src.into(),
            });

            regalloc.free_temp(src);

            dest
        }
        AstNode::LogicalAnd { left, right } => {
            let dest = dest.unwrap_or_else(|| regalloc.allocate_temp());

            lower_expression(ast, functions, function, env, regalloc, left, Some(dest))?;

            let jump_if_false = lower_jump_if_false(function, dest);

            lower_expression(ast, functions, function, env, regalloc, right, Some(dest))?;

            patch_jump(
                function,
                jump_if_false,
                function.instructions.len() as i32 - jump_if_false as i32,
            );

            dest
        }
        AstNode::LogicalOr { left, right } => {
            let dest = dest.unwrap_or_else(|| regalloc.allocate_temp());

            lower_expression(ast, functions, function, env, regalloc, left, Some(dest))?;

            let jump_if_true = lower_jump_if_true(function, dest);

            lower_expression(ast, functions, function, env, regalloc, right, Some(dest))?;

            patch_jump(
                function,
                jump_if_true,
                function.instructions.len() as i32 - jump_if_true as i32,
            );

            dest
        }
        AstNode::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let dest = dest.unwrap_or_else(|| regalloc.allocate_temp());

            lower_expression(
                ast,
                functions,
                function,
                env,
                regalloc,
                condition,
                Some(dest),
            )?;

            let jump_if_false = lower_jump_if_false(function, dest);

            lower_expression(
                ast,
                functions,
                function,
                env,
                regalloc,
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
                functions,
                function,
                env,
                regalloc,
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
        AstNode::Block(ref expressions) => {
            let dest = dest.unwrap_or_else(|| regalloc.allocate_temp());

            env.push_scope();

            lower_block(ast, functions, function, env, regalloc, expressions, dest)?;

            env.pop_scope();

            dest
        }
        AstNode::Function {
            ref parameters,
            block,
            name,
        } => {
            let index = functions.len();
            functions.push(None);

            let arity = parameters.len() as u8;
            let mut inner_function = Function::new(arity);

            let mut inner_env = Environment::with_parent(std::mem::take(env));
            let mut inner_regalloc = RegisterAllocator::default();

            for parameter in parameters.iter().copied() {
                let register = inner_regalloc.allocate_local();

                inner_env.insert_local(Local {
                    name: parameter.symbol,
                    register,
                    kind: LocalKind::Variable,
                });
            }

            let free_variables = collect_free_variables(ast, id);

            for name in free_variables.iter().copied() {
                if let Some(mut local) = inner_env.lookup_in_parent(name.symbol) {
                    let register = inner_regalloc.allocate_local();
                    local.register = register;
                    inner_env.insert_local(local);
                } else {
                    let slice = INTERNER.lock().unwrap().resolve(name.symbol);

                    return Err(report_error!(name.span, "{} is not declared", slice));
                }
            }

            let src = lower_expression(
                ast,
                functions,
                &mut inner_function,
                &mut inner_env,
                &mut inner_regalloc,
                block,
                None,
            )?;

            if !block_returns(ast, block) {
                inner_function.emit_instruction(Instruction::Return { src: src.into() });
            }

            functions[index] = Some(inner_function);

            *env = std::mem::take(&mut inner_env.parent.unwrap_or_default());

            let dest = match name {
                Some(name) => env.lookup(name.symbol).unwrap().register,
                None => dest.unwrap_or_else(|| regalloc.allocate_temp()),
            };

            function.emit_instruction(Instruction::CreateClosure {
                dest: dest.into(),
                src: index as u32,
            });

            for name in free_variables.iter().copied() {
                let Local { register, .. } = env
                    .lookup(name.symbol)
                    .expect("name must've been declared to reach this point of the code");

                function.emit_instruction(Instruction::CaptureValue {
                    dest: dest.into(),
                    src: register.into(),
                });
            }

            dest
        }
        AstNode::FunctionCall {
            callee,
            ref arguments,
        } => {
            let callee_src =
                lower_expression(ast, functions, function, env, regalloc, callee, None)?;

            for (index, argument) in arguments.iter().enumerate() {
                let arg_dest = Register::Local(index as u8);

                lower_expression(
                    ast,
                    functions,
                    function,
                    env,
                    regalloc,
                    *argument,
                    Some(arg_dest),
                )?;
            }

            let dest = dest.unwrap_or_else(|| regalloc.allocate_temp());

            function.emit_instruction(Instruction::Call {
                dest: dest.into(),
                src: callee_src.into(),
                arity: arguments.len() as u8,
            });

            regalloc.free_temp(callee_src);

            dest
        }
        AstNode::MemberAccess { object, property } => {
            let dest = dest.unwrap_or_else(|| regalloc.allocate_temp());

            let object = lower_expression(ast, functions, function, env, regalloc, object, None)?;

            let key = lower_expression(ast, functions, function, env, regalloc, property, None)?;

            function.emit_instruction(Instruction::GetField {
                dest: dest.into(),
                object: object.into(),
                key: key.into(),
            });

            regalloc.free_temp(object);
            regalloc.free_temp(key);

            dest
        }
        AstNode::DictLiteral { ref fields } => {
            let dest = dest.unwrap_or_else(|| regalloc.allocate_temp());

            function.emit_instruction(Instruction::CreateDict { dest: dest.into() });

            for (key, value) in fields.iter().copied() {
                let key = lower_expression(ast, functions, function, env, regalloc, key, None)?;

                let value = lower_expression(ast, functions, function, env, regalloc, value, None)?;

                function.emit_instruction(Instruction::SetField {
                    object: dest.into(),
                    key: key.into(),
                    value: value.into(),
                });

                regalloc.free_temp(key);
                regalloc.free_temp(value);
            }

            dest
        }
        AstNode::Mut { .. }
        | AstNode::NativeFunction { .. }
        | AstNode::Variable { .. }
        | AstNode::Return(..)
        | AstNode::Break
        | AstNode::Continue
        | AstNode::ForLoop { .. }
        | AstNode::WhileLoop { .. } => {
            unreachable!("Statement nodes can't appear inside of expressions")
        }
    };

    Ok(register)
}

fn block_returns(ast: &Ast, id: NodeId) -> bool {
    match *ast.get_node(id) {
        AstNode::Return(..) => true,
        AstNode::Block(ref expressions) => {
            expressions.iter().copied().any(|e| block_returns(ast, e))
        }
        AstNode::If {
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
