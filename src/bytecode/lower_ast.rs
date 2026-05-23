use core::panic;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::{
    bytecode::{
        function::Function,
        instruction::{Const, Instruction},
    },
    diagnostics::error::Error,
    interpreter::INTERNER,
    report_error,
    syntax::{
        ast::{Ast, Expr, ExprId},
        ops::{AssignOp, BinaryOp, UnaryOp},
    },
    util::string_interner::Symbol,
};

#[derive(Clone, Copy)]
struct Local {
    name: Symbol,
    register: Register,
    kind: LocalKind,
}

#[derive(Clone, Copy)]
enum LocalKind {
    Variable,
    Mut,
    Constant,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Register {
    Temp(u8),
    Local(u8),
    Arg(u8),
}

#[derive(Default)]
struct Environment {
    parent: Option<Box<Environment>>,
    scopes: Vec<Vec<Local>>,
    next_register: u8,
    free_temporary_registers: BinaryHeap<Reverse<u8>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            parent: None,
            scopes: vec![Vec::new()],
            next_register: 0,
            free_temporary_registers: BinaryHeap::new(),
        }
    }

    pub fn with_parent(parent: Environment) -> Self {
        Self {
            parent: Some(Box::new(parent)),
            scopes: vec![Vec::new()],
            next_register: 0,
            free_temporary_registers: BinaryHeap::new(),
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

    pub fn lookup(&self, name: Symbol) -> Option<Local> {
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

    pub fn lookup_in_parent(&self, name: Symbol) -> Option<Local> {
        self.parent.as_ref()?.lookup(name)
    }

    pub fn allocate_register(&mut self) -> Register {
        let r = self.next_register;
        self.next_register += 1;
        Register::Local(r)
    }

    pub fn allocate_temporary_register(&mut self) -> Register {
        if let Some(Reverse(r)) = self.free_temporary_registers.pop() {
            Register::Temp(r)
        } else {
            let r = self.next_register;
            self.next_register += 1;
            Register::Temp(r)
        }
    }

    pub fn free_temporary_register(&mut self, register: Register) {
        if let Register::Temp(r) = register {
            self.free_temporary_registers.push(Reverse(r));
        }
    }
}

fn as_number_const(ast: &Ast, function: &mut Function, expr: ExprId) -> Option<Const> {
    match *ast.node(expr) {
        Expr::NumberLiteral(value) => Some(function.store_number_const(value)),
        _ => None,
    }
}

fn collect_free_variables(
    ast: &Ast,
    expression: ExprId,
    bound: &mut Vec<Symbol>,
    free: &mut Vec<Symbol>,
) {
    match *ast.node(expression) {
        Expr::Identifier(name) => {
            if !bound.contains(&name) && !free.contains(&name) {
                free.push(name);
            }
        }
        Expr::Variable { left, right } => {
            collect_free_variables(ast, right, bound, free);

            let Expr::Identifier(name) = *ast.node(left) else {
                unreachable!("let lhs must be an identifier");
            };
            bound.push(name);
        }
        Expr::Mut { left, right } => {
            collect_free_variables(ast, right, bound, free);
            let Expr::Identifier(name) = *ast.node(left) else {
                unreachable!("mut lhs must be an identifier");
            };
            bound.push(name);
        }
        Expr::Function { name, .. } => {
            if let Some(name_id) = name {
                let Expr::Identifier(name) = *ast.node(name_id) else {
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
                } = *ast.node(expr)
                {
                    let Expr::Identifier(name) = *ast.node(name_id) else {
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
        Expr::Unary { right, .. } => collect_free_variables(ast, right, bound, free),
        Expr::LogicalNot(expr) => collect_free_variables(ast, expr, bound, free),
        Expr::Return(expr) => collect_free_variables(ast, expr, bound, free),
        Expr::If {
            condition,
            then_branch,
            else_branch,
        } => {
            collect_free_variables(ast, condition, bound, free);
            collect_free_variables(ast, then_branch, bound, free);
            collect_free_variables(ast, else_branch, bound, free);
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
        Expr::MemberAccess { object, .. } => collect_free_variables(ast, object, bound, free),
        Expr::DictLiteral { ref fields } => {
            for &(key, value) in fields.iter() {
                collect_free_variables(ast, key, bound, free);
                if let Some(value) = value {
                    collect_free_variables(ast, value, bound, free);
                }
            }
        }
        Expr::Break
        | Expr::Continue
        | Expr::NativeFunction { .. }
        | Expr::NumberLiteral(_)
        | Expr::StringLiteral(_)
        | Expr::BooleanLiteral(_) => {}
        Expr::NilLiteral | Expr::ForLoop { .. } => todo!(),
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
    expressions: &[ExprId],
    dest: Option<Register>,
) -> Result<Register, Error> {
    for &expression in expressions.iter() {
        if let Expr::Function {
            name: Some(name_id),
            ..
        } = *ast.node(expression)
        {
            let Expr::Identifier(name) = *ast.node(name_id) else {
                unreachable!("function name must be parsed as identifier");
            };
            let register = env.allocate_register();
            env.insert_local(Local {
                name,
                register,
                kind: LocalKind::Variable,
            });
        }
    }

    let dest = dest.unwrap_or_else(|| env.allocate_temporary_register());

    for expression in expressions.iter().copied() {
        lower_expression(ast, functions, function, env, expression, Some(dest))?;
    }

    Ok(dest)
}

fn resolve_lhs_expression(
    ast: &Ast,
    functions: &mut Vec<Option<Function>>,
    function: &mut Function,
    env: &mut Environment,
    expression: ExprId,
    dest: Option<Register>,
) -> Result<Register, Error> {
    match *ast.node(expression) {
        Expr::Identifier(_) => todo!(),
        Expr::MemberAccess { .. } => todo!(),
        _ => panic!("this is not a valid lhs"),
    }
}

fn lower_expression(
    ast: &Ast,
    functions: &mut Vec<Option<Function>>,
    function: &mut Function,
    env: &mut Environment,
    expression: ExprId,
    dest: Option<Register>,
) -> Result<Register, Error> {
    let register = match *ast.node(expression) {
        Expr::NumberLiteral(value) => {
            let src = function.store_number_const(value);
            let dest = dest.unwrap_or_else(|| env.allocate_temporary_register());

            function.emit_instruction(Instruction::LoadConst {
                dest: dest.into(),
                src,
            });

            dest
        }
        Expr::StringLiteral(value) => {
            let src = function.store_string_const(value);
            let dest = dest.unwrap_or_else(|| env.allocate_temporary_register());

            function.emit_instruction(Instruction::LoadConst {
                dest: dest.into(),
                src,
            });

            dest
        }
        Expr::BooleanLiteral(value) => {
            let src = function.store_boolean_const(value);
            let dest = dest.unwrap_or_else(|| env.allocate_temporary_register());

            function.emit_instruction(Instruction::LoadConst {
                dest: dest.into(),
                src,
            });

            dest
        }
        Expr::NilLiteral => {
            let src = function.store_nil_const();
            let dest = dest.unwrap_or_else(|| env.allocate_temporary_register());

            function.emit_instruction(Instruction::LoadConst {
                dest: dest.into(),
                src,
            });

            dest
        }
        Expr::Identifier(name) => {
            let Some(Local { register, kind, .. }) = env.lookup(name) else {
                let span = ast
                    .span(expression)
                    .expect("identifier expression must have a span");

                let slice = INTERNER.lock().unwrap().resolve(name);
                return Err(report_error!(span, "{} is not declared", slice));
            };

            match kind {
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
            }
        }
        Expr::Variable { left, right } => {
            let Expr::Identifier(name) = *ast.node(left) else {
                unreachable!("let lhs must be an identifier");
            };

            let dest = env.allocate_register();

            env.insert_local(Local {
                name,
                register: dest,
                kind: LocalKind::Variable,
            });

            lower_expression(ast, functions, function, env, right, Some(dest))?;

            dest
        }
        Expr::Mut { left, right } => {
            let Expr::Identifier(name) = *ast.node(left) else {
                unreachable!("mut lhs must be an identifier");
            };

            let dest = env.allocate_register();
            env.insert_local(Local {
                name,
                register: dest,
                kind: LocalKind::Mut,
            });
            lower_expression(ast, functions, function, env, right, Some(dest))?;
            dest
        }
        Expr::Assign { left, right } => {
            let dest = lower_expression(ast, functions, function, env, left, None)?;
            let src = lower_expression(ast, functions, function, env, right, Some(dest))?;

            if src != dest {
                env.free_temporary_register(src);
            }

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
                    AssignOp::AddAssign => Instruction::AddK {
                        dest: dest.into(),
                        src1: dest.into(),
                        src2,
                    },
                    AssignOp::SubtractAssign => Instruction::SubtractRK {
                        dest: dest.into(),
                        src1: dest.into(),
                        src2,
                    },
                    AssignOp::MultiplyAssign => Instruction::MultiplyK {
                        dest: dest.into(),
                        src1: dest.into(),
                        src2,
                    },
                    AssignOp::DivideAssign => Instruction::DivideRK {
                        dest: dest.into(),
                        src1: dest.into(),
                        src2,
                    },
                    AssignOp::ModuloAssign => Instruction::ModuloRK {
                        dest: dest.into(),
                        src1: dest.into(),
                        src2,
                    },
                });
            } else {
                let src2 = lower_expression(ast, functions, function, env, right, None)?;

                function.emit_instruction(match operator {
                    AssignOp::AddAssign => Instruction::Add {
                        dest: dest.into(),
                        src1: dest.into(),
                        src2: src2.into(),
                    },
                    AssignOp::SubtractAssign => Instruction::Subtract {
                        dest: dest.into(),
                        src1: dest.into(),
                        src2: src2.into(),
                    },
                    AssignOp::MultiplyAssign => Instruction::Multiply {
                        dest: dest.into(),
                        src1: dest.into(),
                        src2: src2.into(),
                    },
                    AssignOp::DivideAssign => Instruction::Divide {
                        dest: dest.into(),
                        src1: dest.into(),
                        src2: src2.into(),
                    },
                    AssignOp::ModuloAssign => Instruction::Modulo {
                        dest: dest.into(),
                        src1: dest.into(),
                        src2: src2.into(),
                    },
                });

                env.free_temporary_register(src2);
            }

            dest
        }
        Expr::Binary {
            operator,
            left,
            right,
        } => {
            let dest = dest.unwrap_or_else(|| env.allocate_temporary_register());

            if let Some(src2) = as_number_const(ast, function, right) {
                let src1 = lower_expression(ast, functions, function, env, left, None)?;

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

                env.free_temporary_register(src1);

                return Ok(dest);
            }

            if let Some(src1) = as_number_const(ast, function, left) {
                let src2 = lower_expression(ast, functions, function, env, right, None)?;

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

                env.free_temporary_register(src2);

                return Ok(dest);
            }

            let src1 = lower_expression(ast, functions, function, env, left, None)?;
            let src2 = lower_expression(ast, functions, function, env, right, None)?;

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

            env.free_temporary_register(src1);
            env.free_temporary_register(src2);

            dest
        }
        Expr::Unary { operator, right } => {
            let dest = dest.unwrap_or_else(|| env.allocate_temporary_register());
            let src = lower_expression(ast, functions, function, env, right, None)?;

            function.emit_instruction(match operator {
                UnaryOp::Negate => Instruction::Negate {
                    dest: dest.into(),
                    src: src.into(),
                },
            });

            env.free_temporary_register(src);

            dest
        }
        Expr::LogicalNot(expression) => {
            let dest = dest.unwrap_or_else(|| env.allocate_temporary_register());
            let src = lower_expression(ast, functions, function, env, expression, None)?;

            function.emit_instruction(Instruction::Not {
                dest: dest.into(),
                src: src.into(),
            });

            env.free_temporary_register(src);

            dest
        }
        Expr::LogicalAnd { left, right } => {
            let dest = dest.unwrap_or_else(|| env.allocate_temporary_register());

            lower_expression(ast, functions, function, env, left, Some(dest))?;

            let jump_if_false = lower_jump_if_false(function, env, dest);

            lower_expression(ast, functions, function, env, right, Some(dest))?;

            patch_jump(
                function,
                jump_if_false,
                function.instructions.len() as i32 - jump_if_false as i32,
            );

            dest
        }
        Expr::LogicalOr { left, right } => {
            let dest = dest.unwrap_or_else(|| env.allocate_temporary_register());

            lower_expression(ast, functions, function, env, left, Some(dest))?;

            let jump_if_true = lower_jump_if_true(function, env, dest);

            lower_expression(ast, functions, function, env, right, Some(dest))?;

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
            let dest = dest.unwrap_or_else(|| env.allocate_temporary_register());

            lower_expression(ast, functions, function, env, condition, Some(dest))?;

            let jump_if_false = lower_jump_if_false(function, env, dest);

            lower_expression(ast, functions, function, env, then_branch, Some(dest))?;

            let jump_end = function.emit_instruction(Instruction::Jump { offset: 0 });

            patch_jump(
                function,
                jump_if_false,
                function.instructions.len() as i32 - jump_if_false as i32,
            );

            lower_expression(ast, functions, function, env, else_branch, Some(dest))?;

            patch_jump(
                function,
                jump_end,
                function.instructions.len() as i32 - jump_end as i32,
            );

            dest
        }
        Expr::WhileLoop { condition, block } => {
            let dest = dest.unwrap_or_else(|| env.allocate_temporary_register());

            let condition_register =
                lower_expression(ast, functions, function, env, condition, None)?;

            let jump_if_false = lower_jump_if_false(function, env, condition_register);
            env.free_temporary_register(condition_register);

            let loop_body = function.instructions.len();
            lower_expression(ast, functions, function, env, block, Some(dest))?;

            let condition_register =
                lower_expression(ast, functions, function, env, condition, None)?;
            let jump_if_true = lower_jump_if_true(function, env, condition_register);
            env.free_temporary_register(condition_register);

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
                let Expr::Identifier(name) = *ast.node(parameter) else {
                    unreachable!("parameter must be parsed as identifier");
                };

                let register = inner_env.allocate_register();
                inner_env.insert_local(Local {
                    name,
                    register,
                    kind: LocalKind::Variable,
                });
            }

            let mut bound: Vec<Symbol> = parameters
                .iter()
                .copied()
                .map(|p| {
                    let Expr::Identifier(name) = *ast.node(p) else {
                        unreachable!()
                    };
                    name
                })
                .collect();

            let mut free_names = Vec::new();
            collect_free_variables(ast, block, &mut bound, &mut free_names);

            for name in free_names.iter().copied() {
                if let Some(mut local) = inner_env.lookup_in_parent(name) {
                    let register = inner_env.allocate_register();
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
                inner_function.emit_instruction(Instruction::Return { src: src.into() });
            }

            functions[index] = Some(inner_function);

            *env = *inner_env.parent.take().unwrap();

            let dest = match name {
                Some(name) => lower_expression(ast, functions, function, env, name, None)?,
                None => dest.unwrap_or_else(|| env.allocate_temporary_register()),
            };

            function.emit_instruction(Instruction::CreateClosure {
                dest: dest.into(),
                src: index as u32,
            });

            for name in free_names.iter().copied() {
                let Local { register, .. } = env
                    .lookup(name)
                    .expect("name must've been declared to reach this point of the code");

                function.emit_instruction(Instruction::CaptureValue {
                    dest: dest.into(),
                    src: register.into(),
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
                let arg_dest = Register::Arg(index as u8);
                lower_expression(ast, functions, function, env, *argument, Some(arg_dest))?;
            }

            let dest = dest.unwrap_or_else(|| env.allocate_temporary_register());
            function.emit_instruction(Instruction::Call {
                dest: dest.into(),
                src: callee_src.into(),
                arity: arguments.len() as u8,
            });

            env.free_temporary_register(callee_src);
            dest
        }
        Expr::MemberAccess { object, property } => {
            let dest = dest.unwrap_or_else(|| env.allocate_temporary_register());
            let object = lower_expression(ast, functions, function, env, object, None)?;
            let key = lower_expression(ast, functions, function, env, property, None)?;
            function.emit_instruction(Instruction::GetField {
                dest: dest.into(),
                object: object.into(),
                key: key.into(),
            });
            env.free_temporary_register(object);
            env.free_temporary_register(key);
            dest
        }
        Expr::DictLiteral { ref fields } => {
            let dest = dest.unwrap_or_else(|| env.allocate_temporary_register());

            function.emit_instruction(Instruction::CreateDict { dest: dest.into() });

            for &(key, value) in fields.iter() {
                let key = lower_expression(ast, functions, function, env, key, None)?;
                let value = lower_expression(ast, functions, function, env, value.unwrap(), None)?;
                function.emit_instruction(Instruction::SetField {
                    object: dest.into(),
                    key: key.into(),
                    value: value.into(),
                });
                env.free_temporary_register(key);
                env.free_temporary_register(value);
            }
            dest
        }
        Expr::Return(expression) => {
            let src = lower_expression(ast, functions, function, env, expression, dest)?;

            function.emit_instruction(Instruction::Return { src: src.into() });

            env.free_temporary_register(src);

            src
        }
        Expr::NativeFunction { .. } => todo!(),
        Expr::ForLoop { .. } => todo!(),
        Expr::Break => todo!(),
        Expr::Continue => todo!(),
    };

    Ok(register)
}

fn expression_returns(ast: &Ast, expression: ExprId) -> bool {
    match *ast.node(expression) {
        Expr::Return(..) => true,
        Expr::Block(ref expressions) => expressions
            .iter()
            .copied()
            .any(|e| expression_returns(ast, e)),
        Expr::If {
            then_branch,
            else_branch,
            ..
        } => expression_returns(ast, then_branch) && expression_returns(ast, else_branch),
        _ => false,
    }
}

fn lower_jump_if_true(function: &mut Function, env: &mut Environment, register: Register) -> usize {
    let last = function.instructions.last().copied();

    match last {
        Some(Instruction::Equal { dest, src1, src2 }) => {
            function.instructions.pop();

            function.emit_instruction(Instruction::JumpIfEqual {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::NotEqual { dest, src1, src2 }) => {
            function.instructions.pop();

            function.emit_instruction(Instruction::JumpIfNotEqual {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::Less { dest, src1, src2 }) => {
            function.instructions.pop();

            function.emit_instruction(Instruction::JumpIfLess {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::LessEqual { dest, src1, src2 }) => {
            function.instructions.pop();

            function.emit_instruction(Instruction::JumpIfLessEqual {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::Greater { dest, src1, src2 }) => {
            function.instructions.pop();

            function.emit_instruction(Instruction::JumpIfGreater {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::GreaterEqual { dest, src1, src2 }) => {
            function.instructions.pop();

            function.emit_instruction(Instruction::JumpIfGreaterEqual {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::EqualK { dest, src1, src2 }) => {
            function.instructions.pop();

            function.emit_instruction(Instruction::JumpIfEqualK {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::NotEqualK { dest, src1, src2 }) => {
            function.instructions.pop();

            function.emit_instruction(Instruction::JumpIfNotEqualK {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::LessK { dest, src1, src2 }) => {
            function.instructions.pop();

            function.emit_instruction(Instruction::JumpIfLessK {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::LessEqualK { dest, src1, src2 }) => {
            function.instructions.pop();

            function.emit_instruction(Instruction::JumpIfLessEqualK {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::GreaterK { dest, src1, src2 }) => {
            function.instructions.pop();

            function.emit_instruction(Instruction::JumpIfGreaterK {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::GreaterEqualK { dest, src1, src2 }) => {
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

fn lower_jump_if_false(
    function: &mut Function,
    env: &mut Environment,
    register: Register,
) -> usize {
    let last = function.instructions.last().copied();

    match last {
        Some(Instruction::Equal { dest, src1, src2 }) => {
            function.instructions.pop();

            function.emit_instruction(Instruction::JumpIfNotEqual {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::NotEqual { dest, src1, src2 }) => {
            function.instructions.pop();

            function.emit_instruction(Instruction::JumpIfEqual {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::Less { dest, src1, src2 }) => {
            function.instructions.pop();

            function.emit_instruction(Instruction::JumpIfGreaterEqual {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::LessEqual { dest, src1, src2 }) => {
            function.instructions.pop();

            function.emit_instruction(Instruction::JumpIfGreater {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::Greater { dest, src1, src2 }) => {
            function.instructions.pop();

            function.emit_instruction(Instruction::JumpIfLessEqual {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::GreaterEqual { dest, src1, src2 }) => {
            function.instructions.pop();

            function.emit_instruction(Instruction::JumpIfLess {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::EqualK { dest, src1, src2 }) => {
            function.instructions.pop();

            function.emit_instruction(Instruction::JumpIfNotEqualK {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::NotEqualK { dest, src1, src2 }) => {
            function.instructions.pop();

            function.emit_instruction(Instruction::JumpIfEqualK {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::LessK { dest, src1, src2 }) => {
            function.instructions.pop();

            function.emit_instruction(Instruction::JumpIfGreaterEqualK {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::LessEqualK { dest, src1, src2 }) => {
            function.instructions.pop();

            function.emit_instruction(Instruction::JumpIfGreaterK {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::GreaterK { dest, src1, src2 }) => {
            function.instructions.pop();

            function.emit_instruction(Instruction::JumpIfLessEqualK {
                src1,
                src2,
                offset: 0,
            })
        }
        Some(Instruction::GreaterEqualK { dest, src1, src2 }) => {
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
