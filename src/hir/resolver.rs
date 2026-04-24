use std::collections::HashMap;

use crate::{
    ast::{
        self,
        ops::{AssignOp, BinaryOp},
    },
    error::kaori_error::KaoriError,
    kaori_error,
    lexer::span::Span,
};

use super::{
    expr::{Expr, ExprKind},
    node_id::NodeId,
    symbol::SymbolKind,
    symbol_table::SymbolTable,
};

#[derive(Default)]
pub struct Resolver {
    symbol_table: SymbolTable,
    active_loops: u8,
    ast_to_hir: HashMap<ast::NodeId, NodeId>,
}

impl Resolver {
    pub fn enter_function(&mut self) {
        self.symbol_table.enter_scope();
    }

    pub fn exit_function(&mut self) {
        self.symbol_table.exit_scope();
    }

    pub fn generate_id(&mut self, ast_id: ast::NodeId) -> NodeId {
        let hir_id = NodeId::default();

        self.ast_to_hir.insert(ast_id, hir_id);

        hir_id
    }

    fn resolve_main_function(&mut self, functions: &mut [ast::Expr]) -> Result<(), KaoriError> {
        for (index, function) in functions.iter().enumerate() {
            match &function.kind {
                ast::ExprKind::Function { name, .. } => {
                    if name == "main" {
                        functions.swap(0, index);

                        return Ok(());
                    }
                }
                _ => panic!("Expected only function the global scope"),
            }
        }

        Err(kaori_error!(
            Span::default(),
            "expected a main function to be declared in the program"
        ))
    }

    pub fn resolve(&mut self, expressions: &mut [ast::Expr]) -> Result<Vec<Expr>, KaoriError> {
        self.resolve_main_function(expressions)?;

        for expression in expressions.iter() {
            match &expression.kind {
                ast::ExprKind::Function { name, .. } => {
                    if self.symbol_table.search_current_scope(name).is_some() {
                        return Err(kaori_error!(
                            expression.span,
                            "{} is already declared",
                            name
                        ));
                    }

                    let id = self.generate_id(expression.id);

                    self.symbol_table.declare_function(id, name.to_owned());
                }
                _ => panic!("Expected only function the global scope"),
            };
        }

        let expressions = expressions
            .iter()
            .map(|expression| self.resolve_function(expression))
            .collect::<Result<Vec<Expr>, KaoriError>>()?;

        Ok(expressions)
    }

    fn resolve_function(&mut self, expression: &ast::Expr) -> Result<Expr, KaoriError> {
        Ok(match &expression.kind {
            ast::ExprKind::Function {
                parameters, body, ..
            } => {
                self.enter_function();

                let parameters = parameters
                    .iter()
                    .map(|(name, span)| {
                        if self.symbol_table.search_current_scope(name).is_some() {
                            return Err(kaori_error!(
                                *span,
                                "function can't have parameters with the same name: {}",
                                name,
                            ));
                        };

                        let id = NodeId::default();

                        self.symbol_table.declare_variable(id, name.to_owned());

                        Ok((id, *span))
                    })
                    .collect::<Result<Vec<(NodeId, Span)>, KaoriError>>()?;

                let body = body
                    .iter()
                    .map(|stmt| self.resolve_expression(stmt))
                    .collect::<Result<Vec<Expr>, KaoriError>>()?;

                self.exit_function();

                let id = self.ast_to_hir.get(&expression.id).unwrap();

                Expr::function(*id, parameters, body, expression.span)
            }
            _ => panic!("Should resolve only functions!"),
        })
    }

    fn resolve_expression(&mut self, expression: &ast::Expr) -> Result<Expr, KaoriError> {
        Ok(match &expression.kind {
            ast::ExprKind::Function {
                name,
                parameters,
                body,
            } => todo!(),
            ast::ExprKind::Print(expression) => {
                let expr = self.resolve_expression(expression)?;

                Expr::print(expr, expression.span)
            }
            ast::ExprKind::Block(expressions) => {
                self.symbol_table.enter_scope();

                let expressions = expressions
                    .iter()
                    .map(|expr| self.resolve_expression(expr))
                    .collect::<Result<Vec<Expr>, KaoriError>>()?;

                self.symbol_table.exit_scope();

                Expr::block(expressions, expression.span)
            }
            ast::ExprKind::UncheckedBlock(expressions) => {
                let expressions = expressions
                    .iter()
                    .map(|stmt| self.resolve_expression(stmt))
                    .collect::<Result<Vec<Expr>, KaoriError>>()?;

                Expr::unchecked_block(expressions, expression.span)
            }
            ast::ExprKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition = self.resolve_expression(condition)?;
                let then_branch = self.resolve_expression(then_branch)?;
                let else_branch = match else_branch {
                    Some(branch) => Some(self.resolve_expression(branch)?),
                    _ => None,
                };

                Expr::branch(condition, then_branch, else_branch, expression.span)
            }
            ast::ExprKind::WhileLoop { condition, block } => {
                let init = None;
                let condition = self.resolve_expression(condition)?;
                let increment = None;

                self.active_loops += 1;
                let block = self.resolve_expression(block)?;
                self.active_loops -= 1;

                Expr::loop_(init, condition, block, increment, expression.span)
            }
            ast::ExprKind::ForLoop {
                init,
                condition,
                increment,
                block,
            } => {
                self.symbol_table.enter_scope();

                let init = Some(self.resolve_expression(init)?);
                let condition = self.resolve_expression(condition)?;
                let increment = Some(self.resolve_expression(increment)?);

                self.active_loops += 1;
                let block = self.resolve_expression(block)?;
                self.active_loops -= 1;

                self.symbol_table.exit_scope();

                Expr::loop_(init, condition, block, increment, expression.span)
            }
            ast::ExprKind::Break => {
                if self.active_loops == 0 {
                    return Err(kaori_error!(
                        expression.span,
                        "break statement can't appear outside of loops"
                    ));
                }

                Expr::break_(expression.span)
            }
            ast::ExprKind::Continue => {
                if self.active_loops == 0 {
                    return Err(kaori_error!(
                        expression.span,
                        "continue statement can't appear outside of loops"
                    ));
                }

                Expr::continue_(expression.span)
            }
            ast::ExprKind::Return(expr) => {
                let expr = match expr {
                    Some(e) => Some(self.resolve_expression(e)?),
                    _ => None,
                };

                Expr::return_(expr, expression.span)
            }
            ast::ExprKind::DeclareAssign { left, right } => {
                let right = self.resolve_expression(right)?;

                let ast::ExprKind::Identifier(name) = &left.kind else {
                    return Err(kaori_error!(
                        left.span,
                        "expected identifier to declare variables",
                    ));
                };

                if self.symbol_table.search_current_scope(name).is_some() {
                    return Err(kaori_error!(left.span, "{} is already declared", name));
                };

                let id = NodeId::default();

                self.symbol_table.declare_variable(id, name.to_owned());

                Expr::declare_assign(id, right, expression.span)
            }
            ast::ExprKind::Assign {
                operator,
                left,
                right,
            } => {
                let right = self.resolve_expression(right)?;
                let left = self.resolve_expression(left)?;

                let (ExprKind::VariableRef(_) | ExprKind::MemberAccess { .. }) = &left.kind else {
                    return Err(kaori_error!(
                        left.span,
                        "expected a valid left hand side to assign values to"
                    ));
                };

                let operator = match operator {
                    AssignOp::AddAssign => BinaryOp::Add,
                    AssignOp::SubtractAssign => BinaryOp::Subtract,
                    AssignOp::MultiplyAssign => BinaryOp::Multiply,
                    AssignOp::DivideAssign => BinaryOp::Divide,
                    AssignOp::ModuloAssign => BinaryOp::Modulo,
                    AssignOp::Assign => return Ok(Expr::assign(left, right, expression.span)),
                };

                let right = Expr::binary(operator, left.to_owned(), right.to_owned(), right.span);

                Expr::assign(left, right, expression.span)
            }
            ast::ExprKind::LogicalAnd { left, right } => {
                let left = self.resolve_expression(left)?;
                let right = self.resolve_expression(right)?;

                Expr::logical_and(left, right, expression.span)
            }

            ast::ExprKind::LogicalOr { left, right } => {
                let left = self.resolve_expression(left)?;
                let right = self.resolve_expression(right)?;

                Expr::logical_or(left, right, expression.span)
            }

            ast::ExprKind::LogicalNot(expr) => {
                let expr = self.resolve_expression(expr)?;

                Expr::logical_not(expr, expression.span)
            }
            ast::ExprKind::Binary {
                left,
                right,
                operator,
            } => {
                let left = self.resolve_expression(left)?;
                let right = self.resolve_expression(right)?;

                Expr::binary(*operator, left, right, expression.span)
            }
            ast::ExprKind::Unary { right, operator } => {
                let right = self.resolve_expression(right)?;

                Expr::unary(*operator, right, expression.span)
            }
            ast::ExprKind::FunctionCall { callee, arguments } => {
                let callee = self.resolve_expression(callee)?;
                let arguments = arguments
                    .iter()
                    .map(|argument| self.resolve_expression(argument))
                    .collect::<Result<Vec<Expr>, KaoriError>>()?;

                Expr::function_call(callee, arguments, expression.span)
            }
            ast::ExprKind::MemberAccess { object, property } => {
                let object = self.resolve_expression(object)?;

                let property = self.resolve_expression(property)?;

                Expr::member_access(object, property, expression.span)
            }
            ast::ExprKind::NumberLiteral(value) => Expr::number(*value, expression.span),
            ast::ExprKind::BooleanLiteral(value) => {
                Expr::number(*value as u8 as f64, expression.span)
            }
            ast::ExprKind::StringLiteral(value) => Expr::string(value.to_owned(), expression.span),
            ast::ExprKind::DictLiteral { fields } => {
                let fields = fields
                    .iter()
                    .map(|(key, value)| {
                        let value = match value {
                            Some(value) => self.resolve_expression(value),
                            None => self.resolve_expression(key),
                        }?;

                        let key = match &key.kind {
                            ast::ExprKind::Identifier(name) => {
                                Expr::string(name.to_owned(), key.span)
                            }
                            _ => self.resolve_expression(key)?,
                        };

                        Ok((key, value))
                    })
                    .collect::<Result<Vec<(Expr, Expr)>, KaoriError>>()?;

                Expr::dict_literal(fields, expression.span)
            }
            ast::ExprKind::Identifier(name) => {
                let Some(symbol) = self.symbol_table.search(name) else {
                    return Err(kaori_error!(expression.span, "{} is not declared", name));
                };

                match symbol.kind {
                    SymbolKind::Function => Expr::function_ref(symbol.id, expression.span),
                    SymbolKind::Variable => Expr::variable_ref(symbol.id, expression.span),
                }
            }
        })
    }
}
