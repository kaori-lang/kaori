use crate::{
    syntax::ast::{Ast, Expr, ExprId, Spanned},
    util::string_interner::Symbol,
};

pub fn collect_free_variables(
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
            collect_free_variables(ast, right, bound, free_variables);

            bound.push(left);
        }
        Expr::Ref { left, right } => {
            collect_free_variables(ast, right, bound, free_variables);

            bound.push(left);
        }
        Expr::Function { name, .. } => {
            bound.push(name);
        }
        Expr::Block {
            ref expressions,
            tail,
        } => {
            let size = bound.len();

            for id in expressions.iter().copied() {
                if let Expr::Function { .. } = ast.node(id) {
                    collect_free_variables(ast, id, bound, free_variables);
                }
            }

            if let Some(id) = tail
                && let Expr::Function { .. } = ast.node(id)
            {
                collect_free_variables(ast, id, bound, free_variables);
            }

            for id in expressions.iter().copied() {
                collect_free_variables(ast, id, bound, free_variables);
            }

            if let Some(id) = tail {
                collect_free_variables(ast, id, bound, free_variables);
            }

            bound.truncate(size);
        }
        Expr::Assign { left, right }
        | Expr::Binary { left, right, .. }
        | Expr::LogicalAnd { left, right }
        | Expr::LogicalOr { left, right } => {
            collect_free_variables(ast, left, bound, free_variables);
            collect_free_variables(ast, right, bound, free_variables);
        }
        Expr::Unary { operand, .. } => collect_free_variables(ast, operand, bound, free_variables),
        Expr::LogicalNot(expr) => collect_free_variables(ast, expr, bound, free_variables),
        Expr::Return(expr) => collect_free_variables(ast, expr, bound, free_variables),
        Expr::If {
            condition,
            then_branch,
            else_branch,
        } => {
            collect_free_variables(ast, condition, bound, free_variables);
            collect_free_variables(ast, then_branch, bound, free_variables);
            collect_free_variables(ast, else_branch, bound, free_variables);
        }
        Expr::WhileLoop { condition, block } => {
            collect_free_variables(ast, condition, bound, free_variables);
            collect_free_variables(ast, block, bound, free_variables);
        }
        Expr::FunctionCall {
            callee,
            ref arguments,
        } => {
            collect_free_variables(ast, callee, bound, free_variables);

            for argument in arguments.iter().copied() {
                collect_free_variables(ast, argument, bound, free_variables);
            }
        }
        Expr::MemberAccess { object, .. } => {
            collect_free_variables(ast, object, bound, free_variables)
        }
        Expr::Map { ref entries } => {
            for (key, value) in entries.iter().copied() {
                collect_free_variables(ast, key, bound, free_variables);
                collect_free_variables(ast, value, bound, free_variables);
            }
        }
        Expr::Break
        | Expr::Continue
        | Expr::Number(_)
        | Expr::String(_)
        | Expr::Boolean(_)
        | Expr::Nil
        | Expr::Import { .. }
        | Expr::Lambda { .. } => {}
    }
}
