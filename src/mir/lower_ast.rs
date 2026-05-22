use core::panic;

use crate::{
    diagnostics::error::Error,
    mir::{
        function::Function,
        instruction::{ConstIndex, Instruction, Operand, Register},
    },
    program::INTERNER,
    report_error,
    syntax::{
        ast::{Ast, Expr, ExprId},
        ops::{AssignOp, BinaryOp, UnaryOp},
    },
    util::string_interner::StringIndex,
};

#[derive(Default)]
struct Environment {
    parent: Option<Box<Environment>>,
    scopes: Vec<Vec<Local>>,
}

#[derive(Clone, Copy)]
struct Local {
    name: StringIndex,
    register: Register,
    kind: LocalKind,
}

#[derive(Clone, Copy)]
enum LocalKind {
    Variable,
    Mut,
    Constant,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            parent: None,
            scopes: vec![Vec::new()],
        }
    }

    pub fn with_parent(parent: Environment) -> Self {
        Self {
            parent: Some(Box::new(parent)),
            scopes: vec![Vec::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(Vec::new());
    }

    pub fn pop_scope(&mut self) {
        assert!(
            self.scopes.len() > 1,
            "tried to pop a scope with empty array"
        );

        self.scopes.pop();
    }

    pub fn insert_local(&mut self, local: Local) {
        self.scopes
            .last_mut()
            .expect("scopes must never be empty")
            .push(local);
    }

    pub fn lookup(&self, name: StringIndex) -> Option<Local> {
        for scope in self.scopes.iter().rev() {
            for local in scope.iter().copied().rev() {
                if local.name == name {
                    return Some(local);
                }
            }
        }

        if let Some(parent) = &self.parent {
            if let Some(mut local) = parent.lookup(name) {
                if let LocalKind::Variable = local.kind {
                    local.kind = LocalKind::Constant;
                }

                Some(local)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn lookup_in_parent(&self, name: StringIndex) -> Option<Local> {
        self.parent.as_ref()?.lookup(name)
    }
}

fn as_number_const(ast: &Ast, function: &mut Function, expr: ExprId) -> Option<ConstIndex> {
    match *ast.get(expr) {
        Expr::NumberLiteral(value) => Some(function.push_number(value)),
        _ => None,
    }
}

fn collect_free_variables(
    ast: &Ast,
    expression: ExprId,
    bound: &mut Vec<StringIndex>,
    free: &mut Vec<StringIndex>,
) {
    match *ast.get(expression) {
        Expr::Identifier(name) => {
            if !bound.contains(&name) && !free.contains(&name) {
                free.push(name);
            }
        }
        Expr::Variable { left, right } => {
            collect_free_variables(ast, right, bound, free);
            let Expr::Identifier(name) = *ast.get(left) else {
                unreachable!("let lhs must be an identifier");
            };

            bound.push(name);
        }
        Expr::Mut { left, right } => {
            collect_free_variables(ast, right, bound, free);
            let Expr::Identifier(name) = *ast.get(left) else {
                unreachable!("mut lhs must be an identifier");
            };

            bound.push(name);
        }
        Expr::Function { name, .. } => {
            if let Some(name_id) = name {
                let Expr::Identifier(name) = *ast.get(name_id) else {
                    unreachable!("function name must be an identifier");
                };

                bound.push(name);
            }
        }
        Expr::Block(ref expressions) => {
            let bound_size = bound.len();
            for &expr in expressions.iter() {
                if let Expr::Function {
                    name: Some(name_id),
                    ..
                } = *ast.get(expr)
                {
                    let Expr::Identifier(name) = *ast.get(name_id) else {
                        unreachable!();
                    };
                    bound.push(name);
                }
            }
            for &expr in expressions.iter() {
                collect_free_variables(ast, expr, bound, free);
            }
            bound.truncate(bound_size);
        }
        Expr::Assign { left, right }
        | Expr::Binary { left, right, .. }
        | Expr::LogicalAnd { left, right }
        | Expr::LogicalOr { left, right }
        | Expr::CompoundAssign { left, right, .. } => {
            collect_free_variables(ast, left, bound, free);
            collect_free_variables(ast, right, bound, free);
        }
        Expr::Unary { right, .. } => {
            collect_free_variables(ast, right, bound, free);
        }
        Expr::LogicalNot(expr) => {
            collect_free_variables(ast, expr, bound, free);
        }
        Expr::Return(Some(expr)) => {
            collect_free_variables(ast, expr, bound, free);
        }
        Expr::If {
            condition,
            then_branch,
            else_branch,
        } => {
            collect_free_variables(ast, condition, bound, free);
            collect_free_variables(ast, then_branch, bound, free);
            if let Some(else_branch) = else_branch {
                collect_free_variables(ast, else_branch, bound, free);
            }
        }
        Expr::WhileLoop { condition, block } => {
            collect_free_variables(ast, condition, bound, free);
            collect_free_variables(ast, block, bound, free);
        }
        Expr::FunctionCall {
            callee,
            ref arguments,
        } => {
            collect_free_variables(ast, callee, bound, free);
            for &arg in arguments.iter() {
                collect_free_variables(ast, arg, bound, free);
            }
        }
        Expr::MemberAccess { object, .. } => {
            collect_free_variables(ast, object, bound, free);
        }
        Expr::DictLiteral { ref fields } => {
            for &(key, value) in fields.iter() {
                collect_free_variables(ast, key, bound, free);
                if let Some(value) = value {
                    collect_free_variables(ast, value, bound, free);
                }
            }
        }
        Expr::Return(None)
        | Expr::Break
        | Expr::Continue
        | Expr::NativeFunction { .. }
        | Expr::NumberLiteral(_)
        | Expr::StringLiteral(_)
        | Expr::BooleanLiteral(_) => {}
        Expr::ForLoop { .. } => todo!(),
    }
}

pub fn lower_ast(ast: Ast) -> Result<Vec<Function>, Error> {
    let entry = ast.entry();
    let mut functions = Vec::new();
    functions.push(None);

    let mut env = Environment::new();
    let mut function = Function::new(0);

    let src = lower_expression(&ast, &mut functions, &mut function, &mut env, entry, None)?;

    if !expression_returns(&ast, entry) {
        function.emit_instruction(Instruction::Return { src });
    }

    functions[0] = Some(function);

    let functions = functions.into_iter().map(|f| f.unwrap()).collect();

    Ok(functions)
}

fn lower_block(
    ast: &Ast,
    functions: &mut Vec<Option<Function>>,
    function: &mut Function,
    env: &mut Environment,
    expressions: &[ExprId],
    dest: Option<Register>,
) -> Result<Register, Error> {
    for &expression in expressions.iter() {
        if let Expr::Function {
            name: Some(name_id),
            ..
        } = *ast.get(expression)
        {
            let Expr::Identifier(name) = *ast.get(name_id) else {
                unreachable!("function name must be parsed as identifier");
            };
            let register = function.allocate_register();
            let local = Local {
                name,
                register,
                kind: LocalKind::Variable,
            };

            env.insert_local(local);
        }
    }

    let dest = dest.unwrap_or_else(|| function.allocate_register());

    for expression in expressions.iter().copied().rev().skip(1).rev() {
        lower_expression(ast, functions, function, env, expression, None)?;
    }

    Ok(match expressions.last().copied() {
        Some(expression) => {
            lower_expression(ast, functions, function, env, expression, Some(dest))?
        }
        _ => dest,
    })
}

fn resolve_lhs_expression(
    ast: &Ast,
    functions: &mut Vec<Option<Function>>,
    function: &mut Function,
    env: &mut Environment,
    expression: ExprId,
    dest: Option<Register>,
) -> Result<Register, Error> {
    match *ast.get(expression) {
        Expr::Identifier(name) => {
            todo!()
        }
        Expr::MemberAccess { object, property } => {
            todo!()
        }
        _ => panic!("this is not a valid lhs"),
    }

    todo!()
}

fn lower_expression(
    ast: &Ast,
    functions: &mut Vec<Option<Function>>,
    function: &mut Function,
    env: &mut Environment,
    expression: ExprId,
    dest: Option<Register>,
) -> Result<Register, Error> {
    let register = match *ast.get(expression) {
        Expr::NumberLiteral(value) => {
            let src = function.push_number(value);
            let dest = dest.unwrap_or_else(|| function.allocate_register());
            function.emit_instruction(Instruction::LoadConst { dest, src });
            dest
        }
        Expr::StringLiteral(value) => {
            let src = function.push_string(value);
            let dest = dest.unwrap_or_else(|| function.allocate_register());
            function.emit_instruction(Instruction::LoadConst { dest, src });
            dest
        }
        Expr::BooleanLiteral(_) => {
            let dest = dest.unwrap_or_else(|| function.allocate_register());
            todo!()
        }
        Expr::Identifier(name) => {
            let Some(Local {
                name,
                register,
                kind,
            }) = env.lookup(name)
            else {
                let span = ast
                    .span(expression)
                    .expect("identifier expression must have a span");
                let slice = INTERNER.lock().unwrap().resolve(name);

                return Err(report_error!(span.clone(), "{} is not declared", slice));
            };

            match kind {
                LocalKind::Constant | LocalKind::Variable => {
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
                LocalKind::Mut => {
                    todo!()
                }
            }
        }
        Expr::Variable { left, right } => {
            let Expr::Identifier(name) = *ast.get(left) else {
                unreachable!("let lhs must be an identifier");
            };

            let dest = function.allocate_register();
            let local = Local {
                name,
                register: dest,
                kind: LocalKind::Variable,
            };

            env.insert_local(local);
            lower_expression(ast, functions, function, env, right, Some(dest))?;
            dest
        }
        Expr::Mut { left, right } => {
            let Expr::Identifier(name) = *ast.get(left) else {
                unreachable!("mut lhs must be an identifier");
            };

            let dest = function.allocate_register();
            let local = Local {
                name,
                register: dest,
                kind: LocalKind::Mut,
            };

            env.insert_local(local);
            lower_expression(ast, functions, function, env, right, Some(dest))?;
            dest
        }
        Expr::Assign { left, right } => {
            let dest = lower_expression(ast, functions, function, env, left, None)?;
            lower_expression(ast, functions, function, env, right, Some(dest))?;
            dest
        }
        Expr::CompoundAssign {
            operator,
            left,
            right,
        } => {
            let dest = lower_expression(ast, functions, function, env, left, None)?;

            if let Some(src2) = as_number_const(ast, function, right) {
                function.emit_instruction(match operator {
                    AssignOp::AddAssign => Instruction::Add {
                        dest,
                        src1: dest.into(),
                        src2: src2.into(),
                    },
                    AssignOp::SubtractAssign => Instruction::Subtract {
                        dest,
                        src1: dest.into(),
                        src2: src2.into(),
                    },
                    AssignOp::MultiplyAssign => Instruction::Multiply {
                        dest,
                        src1: dest.into(),
                        src2: src2.into(),
                    },
                    AssignOp::DivideAssign => Instruction::Divide {
                        dest,
                        src1: dest.into(),
                        src2: src2.into(),
                    },
                    AssignOp::ModuloAssign => Instruction::Modulo {
                        dest,
                        src1: dest.into(),
                        src2: src2.into(),
                    },
                });
            } else {
                let src2 = lower_expression(ast, functions, function, env, right, None)?;

                function.emit_instruction(match operator {
                    AssignOp::AddAssign => Instruction::Add {
                        dest,
                        src1: dest.into(),
                        src2: src2.into(),
                    },
                    AssignOp::SubtractAssign => Instruction::Subtract {
                        dest,
                        src1: dest.into(),
                        src2: src2.into(),
                    },
                    AssignOp::MultiplyAssign => Instruction::Multiply {
                        dest,
                        src1: dest.into(),
                        src2: src2.into(),
                    },
                    AssignOp::DivideAssign => Instruction::Divide {
                        dest,
                        src1: dest.into(),
                        src2: src2.into(),
                    },
                    AssignOp::ModuloAssign => Instruction::Modulo {
                        dest,
                        src1: dest.into(),
                        src2: src2.into(),
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

            if let Some(src2) = as_number_const(ast, function, right) {
                let src1 = lower_expression(ast, functions, function, env, left, None)?;

                function.emit_instruction(match operator {
                    BinaryOp::Add => Instruction::Add {
                        dest,
                        src1: src1.into(),
                        src2: src2.into(),
                    },
                    BinaryOp::Subtract => Instruction::Subtract {
                        dest,
                        src1: src1.into(),
                        src2: src2.into(),
                    },
                    BinaryOp::Multiply => Instruction::Multiply {
                        dest,
                        src1: src1.into(),
                        src2: src2.into(),
                    },
                    BinaryOp::Divide => Instruction::Divide {
                        dest,
                        src1: src1.into(),
                        src2: src2.into(),
                    },
                    BinaryOp::Modulo => Instruction::Modulo {
                        dest,
                        src1: src1.into(),
                        src2: src2.into(),
                    },
                    BinaryOp::Equal => Instruction::Equal {
                        dest,
                        src1: src1.into(),
                        src2: src2.into(),
                    },
                    BinaryOp::NotEqual => Instruction::NotEqual {
                        dest,
                        src1: src1.into(),
                        src2: src2.into(),
                    },
                    BinaryOp::Less => Instruction::Less {
                        dest,
                        src1: src1.into(),
                        src2: src2.into(),
                    },
                    BinaryOp::LessEqual => Instruction::LessEqual {
                        dest,
                        src1: src1.into(),
                        src2: src2.into(),
                    },
                    BinaryOp::Greater => Instruction::Greater {
                        dest,
                        src1: src1.into(),
                        src2: src2.into(),
                    },
                    BinaryOp::GreaterEqual => Instruction::GreaterEqual {
                        dest,
                        src1: src1.into(),
                        src2: src2.into(),
                    },
                });

                return Ok(dest);
            }

            if let Some(src1) = as_number_const(ast, function, left) {
                let src2 = lower_expression(ast, functions, function, env, right, None)?;

                function.emit_instruction(match operator {
                    BinaryOp::Add => Instruction::Add {
                        dest,
                        src1: src1.into(),
                        src2: src2.into(),
                    },
                    BinaryOp::Subtract => Instruction::Subtract {
                        dest,
                        src1: src1.into(),
                        src2: src2.into(),
                    },
                    BinaryOp::Multiply => Instruction::Multiply {
                        dest,
                        src1: src1.into(),
                        src2: src2.into(),
                    },
                    BinaryOp::Divide => Instruction::Divide {
                        dest,
                        src1: src1.into(),
                        src2: src2.into(),
                    },
                    BinaryOp::Modulo => Instruction::Modulo {
                        dest,
                        src1: src1.into(),
                        src2: src2.into(),
                    },
                    BinaryOp::Equal => Instruction::Equal {
                        dest,
                        src1: src1.into(),
                        src2: src2.into(),
                    },
                    BinaryOp::NotEqual => Instruction::NotEqual {
                        dest,
                        src1: src1.into(),
                        src2: src2.into(),
                    },
                    BinaryOp::Less => Instruction::Less {
                        dest,
                        src1: src1.into(),
                        src2: src2.into(),
                    },
                    BinaryOp::LessEqual => Instruction::LessEqual {
                        dest,
                        src1: src1.into(),
                        src2: src2.into(),
                    },
                    BinaryOp::Greater => Instruction::Greater {
                        dest,
                        src1: src1.into(),
                        src2: src2.into(),
                    },
                    BinaryOp::GreaterEqual => Instruction::GreaterEqual {
                        dest,
                        src1: src1.into(),
                        src2: src2.into(),
                    },
                });

                return Ok(dest);
            }

            let src1 = lower_expression(ast, functions, function, env, left, None)?;
            let src2 = lower_expression(ast, functions, function, env, right, None)?;

            function.emit_instruction(match operator {
                BinaryOp::Add => Instruction::Add {
                    dest,
                    src1: src1.into(),
                    src2: src2.into(),
                },
                BinaryOp::Subtract => Instruction::Subtract {
                    dest,
                    src1: src1.into(),
                    src2: src2.into(),
                },
                BinaryOp::Multiply => Instruction::Multiply {
                    dest,
                    src1: src1.into(),
                    src2: src2.into(),
                },
                BinaryOp::Divide => Instruction::Divide {
                    dest,
                    src1: src1.into(),
                    src2: src2.into(),
                },
                BinaryOp::Modulo => Instruction::Modulo {
                    dest,
                    src1: src1.into(),
                    src2: src2.into(),
                },
                BinaryOp::Equal => Instruction::Equal {
                    dest,
                    src1: src1.into(),
                    src2: src2.into(),
                },
                BinaryOp::NotEqual => Instruction::NotEqual {
                    dest,
                    src1: src1.into(),
                    src2: src2.into(),
                },
                BinaryOp::Less => Instruction::Less {
                    dest,
                    src1: src1.into(),
                    src2: src2.into(),
                },
                BinaryOp::LessEqual => Instruction::LessEqual {
                    dest,
                    src1: src1.into(),
                    src2: src2.into(),
                },
                BinaryOp::Greater => Instruction::Greater {
                    dest,
                    src1: src1.into(),
                    src2: src2.into(),
                },
                BinaryOp::GreaterEqual => Instruction::GreaterEqual {
                    dest,
                    src1: src1.into(),
                    src2: src2.into(),
                },
            });

            dest
        }
        Expr::Unary { operator, right } => {
            let dest = dest.unwrap_or_else(|| function.allocate_register());
            let src = lower_expression(ast, functions, function, env, right, None)?;

            function.emit_instruction(match operator {
                UnaryOp::Negate => Instruction::Negate { dest, src },
            });

            dest
        }
        Expr::LogicalNot(expression) => {
            let dest = dest.unwrap_or_else(|| function.allocate_register());
            let src = lower_expression(ast, functions, function, env, expression, None)?;
            function.emit_instruction(Instruction::Not { dest, src });
            dest
        }
        Expr::LogicalAnd { left, right } => {
            let dest = dest.unwrap_or_else(|| function.allocate_register());
            lower_expression(ast, functions, function, env, left, Some(dest))?;
            let jump = lower_jump_if_false(function, dest);
            lower_expression(ast, functions, function, env, right, Some(dest))?;
            patch_jump(
                function,
                jump,
                function.instructions.len() as i32 - jump as i32,
            );
            dest
        }
        Expr::LogicalOr { left, right } => {
            let dest = dest.unwrap_or_else(|| function.allocate_register());
            lower_expression(ast, functions, function, env, left, Some(dest))?;
            let jump = lower_jump_if_true(function, dest);
            lower_expression(ast, functions, function, env, right, Some(dest))?;
            patch_jump(
                function,
                jump,
                function.instructions.len() as i32 - jump as i32,
            );
            dest
        }
        Expr::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let dest = dest.unwrap_or_else(|| function.allocate_register());
            let condition = lower_expression(ast, functions, function, env, condition, None)?;

            let jump_if_false = lower_jump_if_false(function, condition);
            lower_expression(ast, functions, function, env, then_branch, Some(dest))?;

            let jump_end = function.emit_instruction(Instruction::Jump { offset: 0 });
            patch_jump(
                function,
                jump_if_false,
                function.instructions.len() as i32 - jump_if_false as i32,
            );

            if let Some(else_branch) = else_branch {
                lower_expression(ast, functions, function, env, else_branch, Some(dest))?;
            } else {
                let src = function.push_number(0.0);
                function.emit_instruction(Instruction::LoadConst { dest, src });
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
                lower_expression(ast, functions, function, env, condition, None)?;

            let jump_if_false = lower_jump_if_false(function, condition_register);

            let loop_body = function.instructions.len();
            lower_expression(ast, functions, function, env, block, Some(dest))?;

            let condition_register =
                lower_expression(ast, functions, function, env, condition, None)?;
            let jump_if_true = lower_jump_if_true(function, condition_register);

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

            for scope in env.scopes.iter() {
                for local in scope.iter().copied() {
                    for index in loop_body..instructions_len {
                        let instruction = function.instructions[index];

                        if touches_register(instruction, local.register) {
                            function.update_live_range(local.register, instructions_len);
                            break;
                        }
                    }
                }
            }

            dest
        }
        Expr::Block(ref expressions) => {
            env.push_scope();
            let register = lower_block(ast, functions, function, env, expressions, dest)?;
            env.pop_scope();
            register
        }
        Expr::Function {
            ref parameters,
            block,
            name,
        } => {
            let index = functions.len();
            functions.push(None);

            let arity = parameters.len() as u8;
            let mut inner_function = Function::new(arity);

            let parent = std::mem::take(env);
            let mut inner_env = Environment::with_parent(parent);

            for parameter in parameters.iter().copied() {
                let Expr::Identifier(name) = *ast.get(parameter) else {
                    unreachable!("parameter must be parsed as identifier");
                };

                let register = inner_function.allocate_register();
                let local = Local {
                    name,
                    register,
                    kind: LocalKind::Variable,
                };

                inner_env.insert_local(local);
            }

            let mut bound: Vec<StringIndex> = parameters
                .iter()
                .copied()
                .map(|p| {
                    let Expr::Identifier(name) = *ast.get(p) else {
                        unreachable!()
                    };
                    name
                })
                .collect();

            let mut free_names = Vec::new();
            collect_free_variables(ast, block, &mut bound, &mut free_names);

            for name in free_names.iter().copied() {
                if let Some(mut local) = inner_env.lookup_in_parent(name) {
                    let register = inner_function.allocate_register();
                    local.register = register;
                    inner_env.insert_local(local);
                } else {
                    panic!("tried to capture undeclared name")
                }
            }

            let src = lower_expression(
                ast,
                functions,
                &mut inner_function,
                &mut inner_env,
                block,
                None,
            )?;

            if !expression_returns(ast, block) {
                inner_function.emit_instruction(Instruction::Return { src });
            }

            functions[index] = Some(inner_function);

            *env = *inner_env.parent.take().unwrap();

            let dest = match name {
                Some(name) => lower_expression(ast, functions, function, env, name, None)?,
                None => dest.unwrap_or_else(|| function.allocate_register()),
            };

            function.emit_instruction(Instruction::CreateClosure {
                dest,
                src: index as u32,
            });

            for name in free_names.iter().copied() {
                let Local { register, .. } = env
                    .lookup(name)
                    .expect("name must've been declared to reach this point of the code");

                function.emit_instruction(Instruction::CaptureValue {
                    dest,
                    src: register,
                });
            }

            dest
        }
        Expr::FunctionCall {
            callee,
            ref arguments,
        } => {
            let callee_src = lower_expression(ast, functions, function, env, callee, None)?;

            for (index, argument) in arguments.iter().enumerate() {
                let dest = Register(-((index + 1) as i16));
                lower_expression(ast, functions, function, env, *argument, Some(dest))?;
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
            let object = lower_expression(ast, functions, function, env, object, None)?;
            let key = lower_expression(ast, functions, function, env, property, None)?;
            function.emit_instruction(Instruction::GetField { dest, object, key });
            dest
        }
        Expr::DictLiteral { ref fields } => {
            let dest = dest.unwrap_or_else(|| function.allocate_register());
            function.emit_instruction(Instruction::CreateDict { dest });
            for &(key, value) in fields.iter() {
                let key = lower_expression(ast, functions, function, env, key, None)?;
                let value = lower_expression(ast, functions, function, env, value.unwrap(), None)?;
                function.emit_instruction(Instruction::SetField {
                    object: dest,
                    key,
                    value,
                });
            }
            dest
        }
        Expr::Return(expression) => {
            let src = match expression {
                Some(expr) => lower_expression(ast, functions, function, env, expr, None)?,
                None => function.emit_nil(),
            };

            function.emit_instruction(Instruction::Return { src });
            src
        }
        Expr::NativeFunction { .. } => todo!(),
        Expr::ForLoop { .. } => todo!(),
        Expr::Break => todo!(),
        Expr::Continue => todo!(),
    };

    Ok(register)
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
            dest == register
                || src1 == Operand::Register(register)
                || src2 == Operand::Register(register)
        }
        Instruction::Not { dest, src }
        | Instruction::Negate { dest, src }
        | Instruction::Move { dest, src }
        | Instruction::CaptureValue { dest, src }
        | Instruction::CreateCell { dest, src }
        | Instruction::SetCell { dest, src }
        | Instruction::GetCell { dest, src } => dest == register || src == register,
        Instruction::LoadConst { dest, .. }
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
        Instruction::JumpIfEqual { src1, src2, .. }
        | Instruction::JumpIfNotEqual { src1, src2, .. }
        | Instruction::JumpIfLess { src1, src2, .. }
        | Instruction::JumpIfNotLess { src1, src2, .. }
        | Instruction::JumpIfLessEqual { src1, src2, .. }
        | Instruction::JumpIfNotLessEqual { src1, src2, .. }
        | Instruction::JumpIfGreater { src1, src2, .. }
        | Instruction::JumpIfNotGreater { src1, src2, .. }
        | Instruction::JumpIfGreaterEqual { src1, src2, .. }
        | Instruction::JumpIfNotGreaterEqual { src1, src2, .. } => {
            src1 == Operand::Register(register) || src2 == Operand::Register(register)
        }
        Instruction::Jump { .. } | Instruction::Nop => false,
    }
}

fn expression_returns(ast: &Ast, expression: ExprId) -> bool {
    match *ast.get(expression) {
        Expr::Return(..) => true,
        Expr::Block(ref expressions) => expressions
            .iter()
            .copied()
            .any(|e| expression_returns(ast, e)),
        Expr::If {
            then_branch,
            else_branch: Some(else_branch),
            ..
        } => expression_returns(ast, then_branch) && expression_returns(ast, else_branch),
        _ => false,
    }
}

fn lower_jump_if_true(function: &mut Function, register: Register) -> usize {
    let last = function.instructions.last().copied();

    match last {
        Some(Instruction::Equal { dest, src1, src2 }) => {
            function.instructions.pop();
            function.remove_live_range(dest);
            function.emit_instruction(Instruction::JumpIfEqual {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::NotEqual { dest, src1, src2 }) => {
            function.instructions.pop();
            function.remove_live_range(dest);
            function.emit_instruction(Instruction::JumpIfNotEqual {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::Less { dest, src1, src2 }) => {
            function.instructions.pop();
            function.remove_live_range(dest);
            function.emit_instruction(Instruction::JumpIfLess {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::LessEqual { dest, src1, src2 }) => {
            function.instructions.pop();
            function.remove_live_range(dest);
            function.emit_instruction(Instruction::JumpIfLessEqual {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::Greater { dest, src1, src2 }) => {
            function.instructions.pop();
            function.remove_live_range(dest);
            function.emit_instruction(Instruction::JumpIfGreater {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::GreaterEqual { dest, src1, src2 }) => {
            function.instructions.pop();
            function.remove_live_range(dest);
            function.emit_instruction(Instruction::JumpIfGreaterEqual {
                src1,
                src2,
                offset: 0,
            })
        }
        _ => function.emit_instruction(Instruction::JumpIfTrue {
            src: register,
            offset: 0,
        }),
    }
}

fn lower_jump_if_false(function: &mut Function, register: Register) -> usize {
    let last = function.instructions.last().copied();

    match last {
        Some(Instruction::Equal { dest, src1, src2 }) => {
            function.instructions.pop();
            function.remove_live_range(dest);
            function.emit_instruction(Instruction::JumpIfNotEqual {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::NotEqual { dest, src1, src2 }) => {
            function.instructions.pop();
            function.remove_live_range(dest);
            function.emit_instruction(Instruction::JumpIfEqual {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::Less { dest, src1, src2 }) => {
            function.instructions.pop();
            function.remove_live_range(dest);
            function.emit_instruction(Instruction::JumpIfNotLess {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::LessEqual { dest, src1, src2 }) => {
            function.instructions.pop();
            function.remove_live_range(dest);
            function.emit_instruction(Instruction::JumpIfNotLessEqual {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::Greater { dest, src1, src2 }) => {
            function.instructions.pop();
            function.remove_live_range(dest);
            function.emit_instruction(Instruction::JumpIfNotGreater {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::GreaterEqual { dest, src1, src2 }) => {
            function.instructions.pop();
            function.remove_live_range(dest);
            function.emit_instruction(Instruction::JumpIfNotGreaterEqual {
                src1,
                src2,
                offset: 0,
            })
        }
        _ => function.emit_instruction(Instruction::JumpIfFalse {
            src: register,
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
        | Instruction::JumpIfNotLess { offset, .. }
        | Instruction::JumpIfLessEqual { offset, .. }
        | Instruction::JumpIfNotLessEqual { offset, .. }
        | Instruction::JumpIfGreater { offset, .. }
        | Instruction::JumpIfNotGreater { offset, .. }
        | Instruction::JumpIfGreaterEqual { offset, .. }
        | Instruction::JumpIfNotGreaterEqual { offset, .. } => *offset = new_offset,
        _ => panic!("tried to patch a non-jump instruction at index {index}"),
    }
}
