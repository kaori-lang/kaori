use crate::{
    syntax::ast::{Ast, Expr, ExprId},
    util::string_interner::Symbol,
};

pub fn collect_free_variables(ast: &Ast, function: ExprId) -> Vec<Symbol> {
    let Expr::Function {
        ref parameters,
        block,
        ..
    } = *ast.node(function)
    else {
        unreachable!("Collect free_variables variables should be called on a function node")
    };

    let mut free_variables = Vec::new();
    let mut bound = parameters
        .iter()
        .copied()
        .map(|parameter| ast.node(parameter).as_identifier())
        .collect();

    collect(ast, block, &mut bound, &mut free_variables);

    free_variables
}

fn collect(
    ast: &Ast,
    expression: ExprId,
    bound: &mut Vec<Symbol>,
    free_variables: &mut Vec<Symbol>,
) {
    match *ast.node(expression) {
        Expr::Identifier(name) => {
            if !bound.contains(&name) && !free_variables.contains(&name) {
                free_variables.push(name);
            }
        }
        Expr::Variable { left, right } => {
            collect(ast, right, bound, free_variables);

            let name = ast.node(left).as_identifier();

            bound.push(name);
        }
        Expr::Mut { left, right } => {
            collect(ast, right, bound, free_variables);

            let name = ast.node(left).as_identifier();

            bound.push(name);
        }
        Expr::Function { name, .. } => {
            if let Some(name) = name {
                let name = ast.node(name).as_identifier();

                bound.push(name);
            }
        }
        Expr::Block(ref expressions) => {
            let bound_size = bound.len();

            for expression in expressions.iter().copied() {
                if let Expr::Function {
                    name: Some(name), ..
                } = *ast.node(expression)
                {
                    let name = ast.node(name).as_identifier();

                    bound.push(name);
                }
            }

            for expression in expressions.iter().copied() {
                collect(ast, expression, bound, free_variables);
            }

            bound.truncate(bound_size);
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
            collect(ast, else_branch, bound, free_variables);
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
        Expr::DictLiteral { ref fields } => {
            for (key, value) in fields.iter().copied() {
                collect(ast, key, bound, free_variables);

                if let Some(value) = value {
                    collect(ast, value, bound, free_variables);
                }
            }
        }
        Expr::Break
        | Expr::Continue
        | Expr::NativeFunction { .. }
        | Expr::NumberLiteral(_)
        | Expr::StringLiteral(_)
        | Expr::BooleanLiteral(_)
        | Expr::NilLiteral => {}
        Expr::ForLoop { .. } => {}
    }
}
