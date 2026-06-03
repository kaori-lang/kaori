use std::collections::{HashMap, HashSet};

use crate::{
    syntax::ast::{Ast, Node, NodeId, Spanned},
    util::string_interner::Symbol,
};

#[derive(Default)]
pub struct FreeVariables {
    cache: HashMap<NodeId, HashSet<Spanned<Symbol>>>,
}

impl FreeVariables {
    pub fn analyze_function(&mut self, ast: &Ast, id: NodeId) -> HashSet<Spanned<Symbol>> {
        if let Some(free) = self.cache.get(&id) {
            return free.clone();
        }

        let mut bound = HashSet::new();
        let mut free = HashSet::new();

        match *ast.node(id) {
            Node::Function {
                name,
                ref parameters,
                block,
            } => {
                bound.insert(name);

                for parameter in parameters.iter().copied() {
                    bound.insert(parameter);
                }

                self.collect_free_variables(ast, block, &mut bound, &mut free);
            }
            Node::Lambda {
                ref parameters,
                block,
            } => {
                for parameter in parameters.iter().copied() {
                    bound.insert(parameter);
                }

                self.collect_free_variables(ast, block, &mut bound, &mut free);
            }
            _ => unreachable!("analyze_function should only be called on function nodes"),
        }

        self.cache.insert(id, free.clone());

        free
    }

    fn collect_free_variables(
        &mut self,
        ast: &Ast,
        id: NodeId,
        bound: &mut HashSet<Spanned<Symbol>>,
        free: &mut HashSet<Spanned<Symbol>>,
    ) {
        match *ast.node(id) {
            Node::Identifier(name) => {
                if !bound.contains(&name) {
                    free.insert(name);
                }
            }
            Node::Variable { left, right } => {
                self.collect_free_variables(ast, right, bound, free);
                bound.insert(left);
            }
            Node::Constant { left, right } => {
                self.collect_free_variables(ast, right, bound, free);
                bound.insert(left);
            }
            Node::Ref { left, right } => {
                self.collect_free_variables(ast, right, bound, free);
                bound.insert(left);
            }
            Node::Lambda { .. } | Node::Function { .. } => {
                let inner_free = self.analyze_function(ast, id);
                free.extend(inner_free.difference(bound).cloned());
            }
            Node::Block {
                ref statements,
                tail,
            } => {
                let mut inner_bound = bound.clone();

                let statements: Vec<NodeId> = statements.iter().copied().chain(tail).collect();

                for id in statements.iter().copied() {
                    if let Node::Function { name, .. } = *ast.node(id) {
                        inner_bound.insert(name);
                    }
                }

                for id in statements.iter().copied() {
                    self.collect_free_variables(ast, id, &mut inner_bound, free);
                }
            }
            Node::Assign { left, right }
            | Node::Binary { left, right, .. }
            | Node::LogicalAnd { left, right }
            | Node::LogicalOr { left, right } => {
                self.collect_free_variables(ast, left, bound, free);
                self.collect_free_variables(ast, right, bound, free);
            }
            Node::Unary { operand, .. } => self.collect_free_variables(ast, operand, bound, free),
            Node::LogicalNot(expr) => self.collect_free_variables(ast, expr, bound, free),
            Node::Return(expr) => self.collect_free_variables(ast, expr, bound, free),
            Node::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.collect_free_variables(ast, condition, bound, free);
                self.collect_free_variables(ast, then_branch, bound, free);
                self.collect_free_variables(ast, else_branch, bound, free);
            }
            Node::WhileLoop { condition, block } => {
                self.collect_free_variables(ast, condition, bound, free);
                self.collect_free_variables(ast, block, bound, free);
            }
            Node::FunctionCall {
                callee,
                ref arguments,
            } => {
                self.collect_free_variables(ast, callee, bound, free);

                for argument in arguments.iter().copied() {
                    self.collect_free_variables(ast, argument, bound, free);
                }
            }
            Node::MemberAccess { object, .. } => {
                self.collect_free_variables(ast, object, bound, free)
            }
            Node::Map { ref entries } => {
                for (key, value) in entries.iter().copied() {
                    self.collect_free_variables(ast, key, bound, free);
                    self.collect_free_variables(ast, value, bound, free);
                }
            }
            Node::Break
            | Node::Continue
            | Node::Number(_)
            | Node::String(_)
            | Node::Boolean(_)
            | Node::Nil
            | Node::Import { .. } => {}
        }
    }
}
