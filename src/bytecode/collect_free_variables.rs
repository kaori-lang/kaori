use crate::{
    syntax::ast::{Ast, Expr, ExprId, Spanned},
    util::string_interner::Symbol,
};

pub fn collect_free_variables(ast: &Ast, id: ExprId) -> Vec<Spanned<Symbol>> {
    let Expr::Function {
        ref parameters,
        block,
        ..
    } = *ast.node(id)
    else {
        unreachable!("collect_free_variables should be called on a function node")
    };

    let mut free_variables = Vec::new();
    let mut bound = parameters.to_vec();

    collect(ast, block, &mut bound, &mut free_variables);

    free_variables
}

fn collect(
    ast: &Ast,
    id: ExprId,
    bound: &mut Vec<Spanned<Symbol>>,
    free_variables: &mut Vec<Spanned<Symbol>>,
) {
    match *ast.node(id) {
        Expr::Identifier(name) => {
            if !bound.iter().any(|found| found.value == name.value)
                && !free_variables.iter().any(|found| found.value == name.value)
            {
                free_variables.push(name);
            }
        }
        Expr::Variable { left, right } => {
            collect(ast, right, bound, free_variables);

            bound.push(left);
        }
        Expr::Function { name, .. } => {
            if let Some(name) = name {
                bound.push(name);
            }
        }
        Expr::Block {
            ref expressions,
            tail,
        } => {
            let size = bound.len();

            for id in expressions.iter().copied() {
                if let Expr::Function { .. } = ast.node(id) {
                    collect(ast, id, bound, free_variables);
                }
            }

            if let Some(id) = tail
                && let Expr::Function { .. } = ast.node(id)
            {
                collect(ast, id, bound, free_variables);
            }

            for id in expressions.iter().copied() {
                collect(ast, id, bound, free_variables);
            }

            if let Some(id) = tail {
                collect(ast, id, bound, free_variables);
            }

            bound.truncate(size);
        }
        Expr::Assign { left, right }
        | Expr::Binary { left, right, .. }
        | Expr::LogicalAnd { left, right }
        | Expr::LogicalOr { left, right }
        | Expr::CompoundAssign { left, right, .. } => {
            collect(ast, left, bound, free_variables);
            collect(ast, right, bound, free_variables);
        }
        Expr::Unary { right, .. } => collect(ast, right, bound, free_variables),
        Expr::LogicalNot(expr) => collect(ast, expr, bound, free_variables),
        Expr::Return(expr) => collect(ast, expr, bound, free_variables),
        Expr::If {
            condition,
            then_branch,
            else_branch,
        } => {
            collect(ast, condition, bound, free_variables);
            collect(ast, then_branch, bound, free_variables);

            if let Some(id) = else_branch {
                collect(ast, id, bound, free_variables);
            }
        }
        Expr::WhileLoop { condition, block } => {
            collect(ast, condition, bound, free_variables);
            collect(ast, block, bound, free_variables);
        }
        Expr::FunctionCall {
            callee,
            ref arguments,
        } => {
            collect(ast, callee, bound, free_variables);

            for argument in arguments.iter().copied() {
                collect(ast, argument, bound, free_variables);
            }
        }
        Expr::MemberAccess { object, .. } => collect(ast, object, bound, free_variables),
        Expr::Map { ref entries } => {
            for (key, value) in entries.iter().copied() {
                collect(ast, key, bound, free_variables);
                collect(ast, value, bound, free_variables);
            }
        }
        Expr::Break
        | Expr::Continue
        | Expr::Number(_)
        | Expr::String(_)
        | Expr::Boolean(_)
        | Expr::Nil => {}
        Expr::ForLoop { .. } => {}
    }
}
