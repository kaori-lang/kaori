use std::collections::HashSet;

use crate::{
    codegen::lower_ast::Lower,
    syntax::ast::{Node, NodeId, Spanned},
    util::string_interner::Symbol,
};

impl<'a> Lower<'a> {
    pub fn analyze_function(&mut self, id: NodeId) -> HashSet<Spanned<Symbol>> {
        if let Some(free_variables) = self.free_variables.get(&id) {
            return free_variables.clone();
        }

        let mut bound = HashSet::new();
        let mut free = HashSet::new();

        match *self.ast.node(id) {
            Node::Function {
                ref parameters,
                block,
                ..
            } => {
                for parameter in parameters.iter().copied() {
                    bound.insert(parameter);
                }

                self.collect_free_variables(block, &mut bound, &mut free);
            }
            Node::Lambda {
                ref parameters,
                block,
            } => {
                for parameter in parameters.iter().copied() {
                    bound.insert(parameter);
                }

                self.collect_free_variables(block, &mut bound, &mut free);
            }
            _ => unreachable!("analyze_function should only be called on function nodes"),
        }

        self.free_variables.insert(id, free.clone());

        free
    }

    fn collect_free_variables(
        &mut self,
        id: NodeId,
        bound: &mut HashSet<Spanned<Symbol>>,
        free: &mut HashSet<Spanned<Symbol>>,
    ) {
        match *self.ast.node(id) {
            Node::Identifier(name) => {
                if !bound.contains(&name) {
                    free.insert(name);
                }
            }
            Node::Variable { left, right } => {
                self.collect_free_variables(right, bound, free);
                bound.insert(left);
            }
            Node::Constant { left, right } => {
                self.collect_free_variables(right, bound, free);
                bound.insert(left);
            }
            Node::Ref { left, right } => {
                self.collect_free_variables(right, bound, free);
                bound.insert(left);
            }
            Node::Lambda { .. } | Node::Function { .. } => {
                let inner_free = self.analyze_function(id);
                free.extend(inner_free.difference(bound).cloned());
            }
            Node::Block {
                ref statements,
                tail,
            } => {
                let mut inner_bound = bound.clone();

                let statements: Vec<NodeId> = statements.iter().copied().chain(tail).collect();

                for id in statements.iter().copied() {
                    if let Node::Function { name, .. } = *self.ast.node(id) {
                        inner_bound.insert(name);
                    }
                }

                for id in statements.iter().copied() {
                    self.collect_free_variables(id, &mut inner_bound, free);
                }
            }
            Node::Assign { left, right }
            | Node::Binary { left, right, .. }
            | Node::LogicalAnd { left, right }
            | Node::LogicalOr { left, right } => {
                self.collect_free_variables(left, bound, free);
                self.collect_free_variables(right, bound, free);
            }
            Node::Unary { operand, .. } => self.collect_free_variables(operand, bound, free),
            Node::LogicalNot(expr) => self.collect_free_variables(expr, bound, free),
            Node::Return(expr) => self.collect_free_variables(expr, bound, free),
            Node::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.collect_free_variables(condition, bound, free);
                self.collect_free_variables(then_branch, bound, free);

                if let Some(id) = else_branch {
                    self.collect_free_variables(id, bound, free);
                }
            }
            Node::WhileLoop { condition, block } => {
                self.collect_free_variables(condition, bound, free);
                self.collect_free_variables(block, bound, free);
            }
            Node::FunctionCall {
                callee,
                ref arguments,
            } => {
                self.collect_free_variables(callee, bound, free);

                for argument in arguments.iter().copied() {
                    self.collect_free_variables(argument, bound, free);
                }
            }
            Node::MemberAccess { object, .. } => self.collect_free_variables(object, bound, free),
            Node::Map { ref entries } => {
                for (key, value) in entries.iter().copied() {
                    self.collect_free_variables(key, bound, free);

                    if let Some(id) = value {
                        self.collect_free_variables(id, bound, free);
                    }
                }
            }
            Node::Import {
                ref path,
                ref bindings,
            } => {
                if bindings.is_empty() {
                    bound.insert(path.last().copied().unwrap());
                } else {
                    for binding in bindings.iter().copied() {
                        bound.insert(binding);
                    }
                }
            }
            Node::Break
            | Node::Continue
            | Node::Number(_)
            | Node::String(_)
            | Node::Boolean(_)
            | Node::Nil => {}
        }
    }
}
