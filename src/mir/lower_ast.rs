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

#[derive(Default)]
struct Environment {
    parent: Option<Box<Environment>>,
    scopes: Vec<Vec<(Local, Register)>>,
    captures: Vec<(Local, Register)>,
}

#[derive(Clone, Copy)]
struct Local {
    name: StringIndex,
    kind: LocalKind,
}

#[derive(Clone, Copy)]
enum LocalKind {
    Variable,
    Cell,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            parent: None,
            scopes: vec![Vec::new()],
            captures: Vec::new(),
        }
    }

    pub fn with_parent(parent: Environment) -> Self {
        Self {
            parent: Some(Box::new(parent)),
            scopes: vec![Vec::new()],
            captures: Vec::new(),
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

    pub fn insert_variable(&mut self, name: StringIndex, register: Register) {
        let local = Local {
            name,
            kind: LocalKind::Variable,
        };
        self.scopes
            .last_mut()
            .expect("scopes must never be empty")
            .push((local, register));
    }

    pub fn insert_cell(&mut self, name: StringIndex, register: Register) {
        let local = Local {
            name,
            kind: LocalKind::Cell,
        };
        self.scopes
            .last_mut()
            .expect("scopes must never be empty")
            .push((local, register));
    }

    pub fn lookup(&self, name: StringIndex) -> Option<Register> {
        for scope in self.scopes.iter().rev() {
            for (local, register) in scope.iter().copied().rev() {
                if local.name == name {
                    return Some(register);
                }
            }
        }
        for &(local, register) in self.captures.iter() {
            if local.name == name {
                return Some(register);
            }
        }
        if let Some(parent) = &self.parent {
            parent.lookup(name)
        } else {
            None
        }
    }

    pub fn lookup_in_parent(&self, name: StringIndex) -> Option<Register> {
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
                unreachable!("DeclareAssign left must be Identifier");
            };
            bound.push(name);
        }
        Expr::Mut { left, right } => {
            collect_free_variables(ast, right, bound, free);
            let Expr::Identifier(name) = *ast.get(left) else {
                unreachable!("Cell left must be Identifier");
            };
            bound.push(name);
        }
        Expr::Function { name, .. } => {
            if let Some(name_id) = name {
                let Expr::Identifier(name) = *ast.get(name_id) else {
                    unreachable!();
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

pub fn lower_ast(ast: Ast) -> Vec<Function> {
    let entry = ast.entry();
    let mut functions = Vec::new();
    functions.push(None);

    let mut env = Environment::new();
    let mut function = Function::new(0);

    let src = lower_expression(&ast, &mut functions, &mut function, &mut env, entry, None);

    if !expression_returns(&ast, entry) {
        function.emit_instruction(Instruction::Return { src });
    }

    functions[0] = Some(function);

    functions.into_iter().map(|f| f.unwrap()).collect()
}

fn lower_block(
    ast: &Ast,
    functions: &mut Vec<Option<Function>>,
    function: &mut Function,
    env: &mut Environment,
    expressions: &[ExprId],
    dest: Option<Register>,
) -> Register {
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
            env.insert_variable(name, register);
        }
    }

    let dest = dest.unwrap_or_else(|| function.allocate_register());

    expressions.iter().copied().fold(dest, |_, expression| {
        lower_expression(ast, functions, function, env, expression, Some(dest))
    })
}

fn lower_expression(
    ast: &Ast,
    functions: &mut Vec<Option<Function>>,
    function: &mut Function,
    env: &mut Environment,
    expression: ExprId,
    dest: Option<Register>,
) -> Register {
    match *ast.get(expression) {
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
            let register = env
                .lookup(name)
                .expect("identifier must be declared; should have been caught earlier");

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
        Expr::Variable { left, right } => {
            let Expr::Identifier(name) = *ast.get(left) else {
                panic!("DeclareAssign left must be Identifier");
            };
            let dest = function.allocate_register();
            env.insert_variable(name, dest);
            lower_expression(ast, functions, function, env, right, Some(dest));
            dest
        }
        Expr::Mut { left, right } => {
            let Expr::Identifier(name) = *ast.get(left) else {
                panic!("Cell left must be Identifier");
            };
            let dest = function.allocate_register();
            env.insert_cell(name, dest);
            lower_expression(ast, functions, function, env, right, Some(dest));
            dest
        }
        Expr::Assign { left, right } => {
            let dest = lower_expression(ast, functions, function, env, left, None);
            lower_expression(ast, functions, function, env, right, Some(dest));
            dest
        }
        Expr::CompoundAssign {
            operator,
            left,
            right,
        } => {
            let dest = lower_expression(ast, functions, function, env, left, None);

            if let Some(src2) = as_number_const(ast, function, right) {
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
                let src2 = lower_expression(ast, functions, function, env, right, None);
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

            if let Some(src2) = as_number_const(ast, function, right) {
                let src1 = lower_expression(ast, functions, function, env, left, None);
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

            if let Some(src1) = as_number_const(ast, function, left) {
                let src2 = lower_expression(ast, functions, function, env, right, None);
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

            let src1 = lower_expression(ast, functions, function, env, left, None);
            let src2 = lower_expression(ast, functions, function, env, right, None);
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
            let src = lower_expression(ast, functions, function, env, right, None);
            function.emit_instruction(match operator {
                UnaryOp::Negate => Instruction::Negate { dest, src },
            });
            dest
        }
        Expr::LogicalNot(expression) => {
            let dest = dest.unwrap_or_else(|| function.allocate_register());
            let src = lower_expression(ast, functions, function, env, expression, None);
            function.emit_instruction(Instruction::Not { dest, src });
            dest
        }
        Expr::LogicalAnd { left, right } => {
            let dest = dest.unwrap_or_else(|| function.allocate_register());
            lower_expression(ast, functions, function, env, left, Some(dest));
            let jump_if_false = lower_conditional_jump(function, dest, true);
            lower_expression(ast, functions, function, env, right, Some(dest));
            patch_jump(
                function,
                jump_if_false,
                function.instructions.len() as i32 - jump_if_false as i32,
            );
            dest
        }
        Expr::LogicalOr { left, right } => {
            let dest = dest.unwrap_or_else(|| function.allocate_register());
            lower_expression(ast, functions, function, env, left, Some(dest));
            let jump_if_true = lower_conditional_jump(function, dest, false);
            lower_expression(ast, functions, function, env, right, Some(dest));
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
            let condition = lower_expression(ast, functions, function, env, condition, None);
            let jump_if_false = lower_conditional_jump(function, condition, true);
            lower_expression(ast, functions, function, env, then_branch, Some(dest));
            let jump_end = function.emit_instruction(Instruction::Jump { offset: 0 });
            patch_jump(
                function,
                jump_if_false,
                function.instructions.len() as i32 - jump_if_false as i32,
            );
            if let Some(else_branch) = else_branch {
                lower_expression(ast, functions, function, env, else_branch, Some(dest));
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
                lower_expression(ast, functions, function, env, condition, None);
            let jump_if_false = lower_conditional_jump(function, condition_register, true);
            let loop_body = function.instructions.len();
            lower_expression(ast, functions, function, env, block, Some(dest));
            let condition_register =
                lower_expression(ast, functions, function, env, condition, None);
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
            for scope in env.scopes.iter() {
                for (_, register) in scope.iter().copied() {
                    for index in loop_body..instructions_len {
                        let instruction = function.instructions[index];
                        if touches_register(instruction, register) {
                            function.update_live_range(register, instructions_len);
                            break;
                        }
                    }
                }
            }

            dest
        }
        Expr::Block(ref expressions) => {
            env.push_scope();
            let result = lower_block(ast, functions, function, env, expressions, dest);
            env.pop_scope();
            result
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

            for &identifier in parameters.iter() {
                let Expr::Identifier(param_name) = *ast.get(identifier) else {
                    unreachable!("parameter must be parsed as identifier");
                };
                let register = inner_function.allocate_register();
                inner_env.insert_variable(param_name, register);
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
            let mut free = Vec::new();
            collect_free_variables(ast, block, &mut bound, &mut free);

            for free_name in free {
                if inner_env.lookup_in_parent(free_name).is_some() {
                    let register = inner_function.allocate_register();
                    let local = Local {
                        name: free_name,
                        kind: LocalKind::Variable,
                    };
                    inner_env.captures.push((local, register));
                }
            }

            let src = lower_expression(
                ast,
                functions,
                &mut inner_function,
                &mut inner_env,
                block,
                None,
            );

            if !expression_returns(ast, block) {
                inner_function.emit_instruction(Instruction::Return { src });
            }

            functions[index] = Some(inner_function);

            *env = *inner_env.parent.take().unwrap();

            let dest = match name {
                Some(name_id) => {
                    let Expr::Identifier(fn_name) = *ast.get(name_id) else {
                        unreachable!();
                    };
                    env.lookup(fn_name)
                        .unwrap_or_else(|| function.allocate_register())
                }
                None => dest.unwrap_or_else(|| function.allocate_register()),
            };

            function.emit_instruction(Instruction::CreateClosure {
                dest,
                src: index as u32,
            });

            for &(local, _inner_register) in inner_env.captures.iter() {
                let src = env
                    .lookup(local.name)
                    .expect("captured name must exist in parent");
                function.emit_instruction(Instruction::CaptureValue { dest, src });
            }

            dest
        }
        Expr::FunctionCall {
            callee,
            ref arguments,
        } => {
            let callee_src = lower_expression(ast, functions, function, env, callee, None);
            for (index, argument) in arguments.iter().enumerate() {
                let dest = Register(-((index + 1) as i16));
                lower_expression(ast, functions, function, env, *argument, Some(dest));
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
            let object = lower_expression(ast, functions, function, env, object, None);
            let key = lower_expression(ast, functions, function, env, property, None);
            function.emit_instruction(Instruction::GetField { dest, object, key });
            dest
        }
        Expr::DictLiteral { ref fields } => {
            let dest = dest.unwrap_or_else(|| function.allocate_register());
            function.emit_instruction(Instruction::CreateDict { dest });
            for &(key, value) in fields.iter() {
                let key = lower_expression(ast, functions, function, env, key, None);
                let value = lower_expression(ast, functions, function, env, value.unwrap(), None);
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
                Some(expr) => lower_expression(ast, functions, function, env, expr, None),
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
        | Instruction::JumpIfNotEqual { src1, src2, .. } => src1 == register || src2 == register,
        Instruction::JumpIfLessK { src1, .. }
        | Instruction::JumpIfLessEqualK { src1, .. }
        | Instruction::JumpIfGreaterK { src1, .. }
        | Instruction::JumpIfGreaterEqualK { src1, .. }
        | Instruction::JumpIfEqualK { src1, .. }
        | Instruction::JumpIfNotEqualK { src1, .. } => src1 == register,
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

fn lower_conditional_jump(function: &mut Function, register: Register, invert: bool) -> usize {
    let last = function.instructions.last().copied();

    match last {
        Some(Instruction::Less { dest, src1, src2 }) => {
            function.instructions.pop();
            function.remove_live_range(dest);
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
