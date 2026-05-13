use std::{
    collections::{HashMap, HashSet},
    vec,
};

use crate::{
    diagnostics::error::Error,
    program::INTERNER,
    report_error,
    syntax::ast::{Ast, Expr, ExprId},
    util::string_interner::StringIndex,
};

#[derive(Default)]
struct Environment {
    parent: Option<Box<Environment>>,
    scopes: Vec<HashSet<StringIndex>>,
    captures: Vec<StringIndex>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            parent: None,
            scopes: vec![HashSet::new()],
            captures: Vec::new(),
        }
    }

    pub fn with_parent(parent: Environment) -> Self {
        Self {
            parent: Some(Box::new(parent)),
            scopes: vec![HashSet::new()],
            captures: Vec::new(),
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashSet::new());
    }

    pub fn pop_scope(&mut self) {
        assert!(
            self.scopes.len() > 1,
            "tried to pop a scope with empty array"
        );

        self.scopes.pop();
    }

    pub fn insert(&mut self, name: StringIndex) {
        self.scopes
            .last_mut()
            .expect("scopes must never be empty")
            .insert(name);
    }

    pub fn lookup_local(&mut self, name: StringIndex) -> bool {
        for scope in self.scopes.iter().rev() {
            if scope.get(&name).is_some() {
                return true;
            }
        }

        for found_name in self.captures.iter().copied() {
            if found_name == name {
                return true;
            }
        }

        if let Some(parent) = &mut self.parent {
            self.captures.push(name);

            parent.lookup_local(name)
        } else {
            false
        }
    }
}

pub fn resolve(ast: &Ast) -> Result<HashMap<ExprId, Vec<StringIndex>>, Error> {
    let mut environment = Environment::new();
    let mut captures = HashMap::new();

    resolve_expression(ast, ast.entry(), &mut environment, &mut captures)?;

    Ok(captures)
}

fn resolve_block(
    ast: &Ast,
    expressions: &[ExprId],
    environment: &mut Environment,
    captures: &mut HashMap<ExprId, Vec<StringIndex>>,
) -> Result<(), Error> {
    for expression in expressions.iter().copied() {
        if let Expr::Function {
            name: Some(identifier),
            ..
        } = *ast.get(expression)
        {
            let Expr::Identifier(name) = *ast.get(identifier) else {
                unreachable!("function name must be parsed as identifier");
            };

            environment.insert(name);
        }
    }

    for &expression in expressions {
        resolve_expression(ast, expression, environment, captures)?;
    }

    Ok(())
}

fn resolve_expression(
    ast: &Ast,
    expression: ExprId,
    environment: &mut Environment,
    captures: &mut HashMap<ExprId, Vec<StringIndex>>,
) -> Result<(), Error> {
    match *ast.get(expression) {
        Expr::NativeFunction { .. } => {}

        Expr::Function {
            ref parameters,
            block,
            ..
        } => {
            let parent = std::mem::take(environment);
            let mut inner = Environment::with_parent(parent);

            for identifier in parameters.iter().copied() {
                let Expr::Identifier(name) = *ast.get(identifier) else {
                    unreachable!("parameter must be parsed as identifier");
                };

                inner.insert(name);
            }

            resolve_expression(ast, block, &mut inner, captures)?;

            captures.insert(expression, inner.captures);

            *environment = *inner.parent.unwrap();
        }

        Expr::DeclareAssign { left, right } => {
            resolve_expression(ast, right, environment, captures)?;

            let Expr::Identifier(name) = *ast.get(left) else {
                unreachable!("declare_assign lhs must be parsed as identifier");
            };

            environment.insert(name);
        }
        Expr::Assign { left, right, .. }
        | Expr::LogicalAnd { left, right }
        | Expr::LogicalOr { left, right }
        | Expr::Binary { left, right, .. } => {
            resolve_expression(ast, left, environment, captures)?;
            resolve_expression(ast, right, environment, captures)?;
        }
        Expr::LogicalNot(expr) => {
            resolve_expression(ast, expr, environment, captures)?;
        }
        Expr::Return(expr) => {
            if let Some(expr) = expr {
                resolve_expression(ast, expr, environment, captures)?;
            }
        }
        Expr::Unary { right, .. } => {
            resolve_expression(ast, right, environment, captures)?;
        }
        Expr::FunctionCall {
            callee,
            ref arguments,
        } => {
            resolve_expression(ast, callee, environment, captures)?;

            for argument in arguments.iter().copied() {
                resolve_expression(ast, argument, environment, captures)?;
            }
        }
        Expr::MemberAccess { object, .. } => {
            resolve_expression(ast, object, environment, captures)?;
        }
        Expr::Block(ref expressions) => {
            environment.push_scope();
            resolve_block(ast, expressions, environment, captures)?;
            environment.pop_scope();
        }
        Expr::If {
            condition,
            then_branch,
            else_branch,
        } => {
            resolve_expression(ast, condition, environment, captures)?;
            resolve_expression(ast, then_branch, environment, captures)?;

            if let Some(else_branch) = else_branch {
                resolve_expression(ast, else_branch, environment, captures)?;
            }
        }
        Expr::ForLoop { .. } => {
            todo!("ForLoop")
        }
        Expr::WhileLoop { condition, block } => {
            resolve_expression(ast, condition, environment, captures)?;
            resolve_expression(ast, block, environment, captures)?;
        }
        Expr::Break | Expr::Continue => {}
        Expr::Identifier(name) => {
            if !environment.lookup_local(name) {
                let slice = INTERNER.lock().unwrap().resolve(name);
                let span = ast.span(expression).unwrap().clone();
                return Err(report_error!(span, "`{}` is not declared", slice));
            };
        }
        Expr::StringLiteral(_) | Expr::NumberLiteral(_) | Expr::DictLiteral { .. } => {}
    };

    Ok(())
}
