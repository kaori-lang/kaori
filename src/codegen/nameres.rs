use std::collections::HashMap;

use crate::{
    diagnostics::error::Error,
    report_error,
    syntax::ast::{Ast, Expr, ExprId},
    util::string_interner::Symbol,
};

#[derive(Clone, Copy, Debug)]
pub enum Resolution {
    Var(ExprId),
    Const(ExprId),
}

#[derive(Clone, Copy)]
struct Local {
    pub id: ExprId,
    pub symbol: Symbol,
    pub kind: LocalKind,
}

#[derive(Clone, Copy)]
enum LocalKind {
    Var,
    Const,
}

#[derive(Default)]
pub struct NameResolution {
    pub resolutions: HashMap<ExprId, Resolution>,
}

#[derive(Default)]
struct Environment {
    parent: Option<Box<Environment>>,
    scopes: Vec<Vec<Local>>,
    captures: HashMap<Symbol, Vec<ExprId>>,
}

impl Environment {
    fn new() -> Self {
        Self {
            parent: None,
            scopes: vec![Vec::new()],
            captures: HashMap::new(),
        }
    }

    fn with_parent(parent: Environment) -> Self {
        Self {
            parent: Some(Box::new(parent)),
            scopes: vec![Vec::new()],
            captures: HashMap::new(),
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(Vec::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare_variable(&mut self, symbol: Symbol, id: ExprId) {
        let resolution = Local {
            id,
            symbol,
            kind: LocalKind::Var,
        };

        self.scopes
            .last_mut()
            .expect("expected scope to exist in scopes vec")
            .push(resolution);
    }

    fn declare_constant(&mut self, symbol: Symbol, id: ExprId) {
        let resolution = Local {
            id,
            symbol,
            kind: LocalKind::Const,
        };

        self.scopes
            .last_mut()
            .expect("expected scope to exist in scopes vec")
            .push(resolution);
    }

    fn lookup(&self, symbol: Symbol, ast: &mut Ast) -> Option<Resolution> {
        for scope in self.scopes.iter().rev() {
            for local in scope.iter().rev().copied() {
                if local.symbol == symbol {
                    return Some(match local.kind {
                        LocalKind::Const => Resolution::Const(local.id),
                        LocalKind::Var => Resolution::Var(local.id),
                    });
                }
            }
        }

        if let Some(parent) = &self.parent {
            let parent_lookup = parent.lookup(symbol, ast);

            if let Some(resolution) = parent_lookup {}
            return parent.lookup(symbol).map(|res| match res {
                // found in parent — becomes an upvalue regardless of depth
                Resolution::Var(id) | Resolution::Const(id) => Resolution::Const(id),
            });
        }

        None
    }
}

pub fn resolve(ast: &mut Ast) -> Result<NameResolution, Error> {
    let mut resolution = NameResolution::default();
    let mut env = Environment::new();
    let root = ast.last();

    resolve_expression(ast, &mut env, &mut resolution, root)?;

    Ok(resolution)
}

fn resolve_effect(
    ast: &mut Ast,
    env: &mut Environment,
    resolution: &mut NameResolution,
    id: ExprId,
) -> Result<(), Error> {
    match *ast.node_mut(id) {
        Expr::Variable { left, right } => {
            resolve_expression(ast, env, resolution, right)?;
            env.declare_variable(left.value, id);
        }
        Expr::Ref { left, right } => {
            resolve_expression(ast, env, resolution, right)?;
            env.declare_constant(left.value, id);
        }
        Expr::Assign { left, right } => {
            resolve_expression(ast, env, resolution, left)?;
            resolve_expression(ast, env, resolution, right)?;
        }
        Expr::WhileLoop { condition, block } => {
            resolve_expression(ast, env, resolution, condition)?;
            resolve_effect(ast, env, resolution, block)?;
        }
        Expr::Return(expression) => {
            resolve_expression(ast, env, resolution, expression)?;
        }
        Expr::Break | Expr::Continue => {}
        Expr::Block {
            ref expressions,
            tail,
        } => {
            env.push_scope();

            let expressions = expressions.to_vec();

            for &id in expressions.iter() {
                if let Expr::Function { name, .. } = *ast.node(id) {
                    env.declare_constant(name.value, id);
                }
            }

            for &id in expressions.iter() {
                resolve_effect(ast, env, resolution, id)?;
            }

            if let Some(id) = tail {
                return Err(report_error!(
                    ast.span(id),
                    "expected `;` after expression, only block expressions can produce values"
                ));
            }

            env.pop_scope();
        }
        Expr::Function {
            name,
            ref parameters,
            block,
            ref captures,
        } => {
            let mut inner_env = Environment::with_parent(std::mem::take(env));

            inner_env.declare_variable(name.value, id);

            for parameter in parameters.iter().copied() {
                inner_env.declare_variable(parameter.value, id);
            }

            resolve_expression(ast, &mut inner_env, resolution, block)?;

            *env = *inner_env.parent.unwrap();
        }
        _ => {
            resolve_expression(ast, env, resolution, id)?;
        }
    }

    Ok(())
}

fn resolve_expression(
    ast: &mut Ast,
    env: &mut Environment,
    resolution: &mut NameResolution,
    id: ExprId,
) -> Result<(), Error> {
    match *ast.node_mut(id) {
        Expr::Number(..) | Expr::String(..) | Expr::Boolean(..) | Expr::Nil => {}
        Expr::Identifier(name) => match env.lookup(name.value) {
            Some(res) => {
                resolution.resolutions.insert(id, res);
            }
            None => {
                return Err(report_error!(name.span, "undeclared variable"));
            }
        },

        Expr::Binary { left, right, .. } => {
            resolve_expression(ast, env, resolution, left)?;
            resolve_expression(ast, env, resolution, right)?;
        }

        Expr::Unary { operand, .. } => {
            resolve_expression(ast, env, resolution, operand)?;
        }

        Expr::LogicalNot(expression) => {
            resolve_expression(ast, env, resolution, expression)?;
        }

        Expr::LogicalAnd { left, right } | Expr::LogicalOr { left, right } => {
            resolve_expression(ast, env, resolution, left)?;
            resolve_expression(ast, env, resolution, right)?;
        }

        Expr::If {
            condition,
            then_branch,
            else_branch,
        } => {
            resolve_expression(ast, env, resolution, condition)?;
            resolve_expression(ast, env, resolution, then_branch)?;
            resolve_expression(ast, env, resolution, else_branch)?;
        }

        Expr::Block {
            ref expressions,
            tail,
        } => {
            env.push_scope();

            let expressions = expressions.to_vec();

            for &id in expressions.iter() {
                if let Expr::Function { name, .. } = *ast.node_mut(id) {
                    env.declare_constant(name.value, id);
                }
            }

            for id in expressions {
                resolve_effect(ast, env, resolution, id)?;
            }

            if let Some(id) = tail {
                resolve_expression(ast, env, resolution, id)?;
            }

            env.pop_scope();
        }

        Expr::FunctionCall {
            callee,
            ref arguments,
        } => {
            let arguments = arguments.to_vec();

            resolve_expression(ast, env, resolution, callee)?;

            for argument in arguments {
                resolve_expression(ast, env, resolution, argument)?;
            }
        }

        Expr::MemberAccess { object, .. } => {
            resolve_expression(ast, env, resolution, object)?;
        }

        Expr::Map { ref entries } => {
            for (key, value) in entries.to_vec() {
                resolve_expression(ast, env, resolution, key)?;
                resolve_expression(ast, env, resolution, value)?;
            }
        }

        Expr::Lambda {
            ref parameters,
            block,
            ref captures,
        } => {
            let mut inner_env = Environment::with_parent(std::mem::take(env));

            for parameter in parameters.iter().copied() {
                inner_env.declare_variable(parameter.value, id);
            }

            resolve_expression(ast, &mut inner_env, resolution, block)?;

            *env = *inner_env.parent.unwrap();
        }

        Expr::Import { .. } => {}

        Expr::Variable { .. }
        | Expr::Ref { .. }
        | Expr::Assign { .. }
        | Expr::WhileLoop { .. }
        | Expr::Function { .. } => {
            resolve_effect(ast, env, resolution, id)?;
        }

        Expr::Return(..) | Expr::Break | Expr::Continue => {
            return Err(report_error!(
                ast.span(id),
                "expression does not produce a value and cannot be used in value position"
            ));
        }
    }

    Ok(())
}
