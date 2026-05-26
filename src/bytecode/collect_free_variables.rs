use crate::syntax::ast::{Ast, AstNode, Name, NodeId};

pub fn collect_free_variables(ast: &Ast, id: NodeId) -> Vec<Name> {
    let AstNode::Function {
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

fn collect(ast: &Ast, id: NodeId, bound: &mut Vec<Name>, free_variables: &mut Vec<Name>) {
    match *ast.node(id) {
        AstNode::Identifier(name) => {
            if !bound.iter().any(|found| found.symbol == name.symbol)
                && !free_variables
                    .iter()
                    .any(|found| found.symbol == name.symbol)
            {
                free_variables.push(name);
            }
        }
        AstNode::Variable { left, right } | AstNode::Mut { left, right } => {
            collect(ast, right, bound, free_variables);

            bound.push(left);
        }
        AstNode::Function { name, .. } => {
            if let Some(name) = name {
                bound.push(name);
            }
        }
        AstNode::Block {
            ref statements,
            tail,
        } => {
            let size = bound.len();

            for id in statements.iter().copied() {
                if let AstNode::Function { .. } = ast.node(id) {
                    collect(ast, id, bound, free_variables);
                }
            }

            if let Some(id) = tail
                && let AstNode::Function { .. } = ast.node(id)
            {
                collect(ast, id, bound, free_variables);
            }

            for id in statements.iter().copied() {
                collect(ast, id, bound, free_variables);
            }

            if let Some(id) = tail {
                collect(ast, id, bound, free_variables);
            }

            bound.truncate(size);
        }
        AstNode::Assign { left, right }
        | AstNode::Binary { left, right, .. }
        | AstNode::LogicalAnd { left, right }
        | AstNode::LogicalOr { left, right }
        | AstNode::CompoundAssign { left, right, .. } => {
            collect(ast, left, bound, free_variables);
            collect(ast, right, bound, free_variables);
        }
        AstNode::Unary { right, .. } => collect(ast, right, bound, free_variables),
        AstNode::LogicalNot(expr) => collect(ast, expr, bound, free_variables),
        AstNode::Return(expr) => collect(ast, expr, bound, free_variables),
        AstNode::If {
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
        AstNode::WhileLoop { condition, block } => {
            collect(ast, condition, bound, free_variables);
            collect(ast, block, bound, free_variables);
        }
        AstNode::FunctionCall {
            callee,
            ref arguments,
        } => {
            collect(ast, callee, bound, free_variables);

            for argument in arguments.iter().copied() {
                collect(ast, argument, bound, free_variables);
            }
        }
        AstNode::MemberAccess { object, .. } => collect(ast, object, bound, free_variables),
        AstNode::DictLiteral { ref fields } => {
            for (key, value) in fields.iter().copied() {
                collect(ast, key, bound, free_variables);
                collect(ast, value, bound, free_variables);
            }
        }
        AstNode::Break
        | AstNode::Continue
        | AstNode::NativeFunction { .. }
        | AstNode::NumberLiteral(_)
        | AstNode::StringLiteral(_)
        | AstNode::BooleanLiteral(_)
        | AstNode::NilLiteral => {}
        AstNode::ForLoop { .. } => {}
    }
}
