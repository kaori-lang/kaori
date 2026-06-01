use std::collections::{HashMap, HashSet};

use crate::{
    compiler::INTERNER,
    syntax::ast::{Ast, Expr, ExprId, Spanned},
    util::string_interner::Symbol,
};

#[derive(Default)]
pub struct FreeVariables {
    cache: HashMap<ExprId, HashSet<Spanned<Symbol>>>,
}

impl FreeVariables {
    pub fn analyze_function(&mut self, ast: &Ast, id: ExprId) -> HashSet<Spanned<Symbol>> {
        if let Some(free) = self.cache.get(&id) {
            return free.clone();
        }

        let mut bound = HashSet::new();
        let mut free = HashSet::new();

        match *ast.node(id) {
            Expr::Function {
                name,
                ref parameters,
                block,
            } => {
                bound.insert(name);

                for parameter in parameters.iter().copied() {
                    bound.insert(parameter);
                }

                self.collect_free_variables(ast, block, &mut bound, &mut free);

                println!("{}", INTERNER.lock().unwrap().resolve(name.value));

                for var in free.iter() {
                    print!("{} ", INTERNER.lock().unwrap().resolve(var.value));
                }

                println!("\n");
            }
            Expr::Lambda {
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
        id: ExprId,
        bound: &mut HashSet<Spanned<Symbol>>,
        free: &mut HashSet<Spanned<Symbol>>,
    ) {
        match *ast.node(id) {
            Expr::Identifier(name) => {
                if !bound.contains(&name) {
                    free.insert(name);
                }
            }
            Expr::Variable { left, right } => {
                self.collect_free_variables(ast, right, bound, free);
                bound.insert(left);
            }
            Expr::Constant { left, right } => {
                self.collect_free_variables(ast, right, bound, free);
                bound.insert(left);
            }
            Expr::Ref { left, right } => {
                self.collect_free_variables(ast, right, bound, free);
                bound.insert(left);
            }
            Expr::Lambda { .. } | Expr::Function { .. } => {
                let inner_free = self.analyze_function(ast, id);
                free.extend(inner_free.difference(bound).cloned());
            }
            Expr::Block {
                ref expressions,
                tail,
            } => {
                let mut inner_bound = bound.clone();

                let expressions: Vec<ExprId> = expressions.iter().copied().chain(tail).collect();

                for id in expressions.iter().copied() {
                    if let Expr::Function { name, .. } = *ast.node(id) {
                        inner_bound.insert(name);
                    }
                }

                for id in expressions.iter().copied() {
                    self.collect_free_variables(ast, id, &mut inner_bound, free);
                }
            }
            Expr::Assign { left, right }
            | Expr::Binary { left, right, .. }
            | Expr::LogicalAnd { left, right }
            | Expr::LogicalOr { left, right } => {
                self.collect_free_variables(ast, left, bound, free);
                self.collect_free_variables(ast, right, bound, free);
            }
            Expr::Unary { operand, .. } => self.collect_free_variables(ast, operand, bound, free),
            Expr::LogicalNot(expr) => self.collect_free_variables(ast, expr, bound, free),
            Expr::Return(expr) => self.collect_free_variables(ast, expr, bound, free),
            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.collect_free_variables(ast, condition, bound, free);
                self.collect_free_variables(ast, then_branch, bound, free);
                self.collect_free_variables(ast, else_branch, bound, free);
            }
            Expr::WhileLoop { condition, block } => {
                self.collect_free_variables(ast, condition, bound, free);
                self.collect_free_variables(ast, block, bound, free);
            }
            Expr::FunctionCall {
                callee,
                ref arguments,
            } => {
                self.collect_free_variables(ast, callee, bound, free);

                for argument in arguments.iter().copied() {
                    self.collect_free_variables(ast, argument, bound, free);
                }
            }
            Expr::MemberAccess { object, .. } => {
                self.collect_free_variables(ast, object, bound, free)
            }
            Expr::Map { ref entries } => {
                for (key, value) in entries.iter().copied() {
                    self.collect_free_variables(ast, key, bound, free);
                    self.collect_free_variables(ast, value, bound, free);
                }
            }
            Expr::Break
            | Expr::Continue
            | Expr::Number(_)
            | Expr::String(_)
            | Expr::Boolean(_)
            | Expr::Nil
            | Expr::Import { .. } => {}
        }
    }
}
