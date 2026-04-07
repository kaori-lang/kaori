use std::collections::HashMap;

use crate::{
    ast::{
        self,
        assign_op::AssignOpKind,
        binary_op::{BinaryOp, BinaryOpKind},
    },
    error::kaori_error::KaoriError,
    kaori_error,
    lexer::span::Span,
};

use super::{
    decl::Decl,
    expr::{Expr, ExprKind},
    node_id::NodeId,
    stmt::Stmt,
    symbol::SymbolKind,
    symbol_table::SymbolTable,
};

#[derive(Default)]
pub struct Resolver {
    symbol_table: SymbolTable,
    active_loops: u8,
    local_scope: bool,
    ast_to_hir: HashMap<ast::NodeId, NodeId>,
}

impl Resolver {
    pub fn enter_function(&mut self) {
        self.symbol_table.enter_scope();
        self.local_scope = true;
    }

    pub fn exit_function(&mut self) {
        self.symbol_table.exit_scope();
        self.local_scope = false;
    }

    pub fn generate_id(&mut self, id: ast::node_id::NodeId) -> NodeId {
        let _id = NodeId::default();

        self.ast_to_hir.insert(id, _id);

        _id
    }

    fn resolve_main_function(&mut self, declarations: &mut [ast::Decl]) -> Result<(), KaoriError> {
        for (index, declaration) in declarations.iter().enumerate() {
            match &declaration.kind {
                ast::DeclKind::Function { name, .. } => {
                    if name == "main" {
                        declarations.swap(0, index);

                        return Ok(());
                    }
                }
            }
        }

        Err(kaori_error!(
            Span::default(),
            "expected a main function to be declared in the program"
        ))
    }

    pub fn resolve(&mut self, declarations: &mut [ast::Decl]) -> Result<Vec<Decl>, KaoriError> {
        self.resolve_main_function(declarations)?;

        for declaration in declarations.iter() {
            match &declaration.kind {
                ast::DeclKind::Function { name, .. } => {
                    if self.symbol_table.search_current_scope(name).is_some() {
                        return Err(kaori_error!(
                            declaration.span,
                            "{} is already declared",
                            name
                        ));
                    }

                    let id = self.generate_id(declaration.id);

                    self.symbol_table.declare_function(id, name.to_owned());
                }
            };
        }

        let declarations = declarations
            .iter()
            .map(|declaration| self.resolve_declaration(declaration))
            .collect::<Result<Vec<Decl>, KaoriError>>()?;

        Ok(declarations)
    }

    fn resolve_declaration(&mut self, declaration: &ast::Decl) -> Result<Decl, KaoriError> {
        let _decl = match &declaration.kind {
            ast::DeclKind::Function {
                parameters, body, ..
            } => {
                self.enter_function();

                let parameters = parameters
                    .iter()
                    .map(|parameter| self.resolve_expression(parameter))
                    .collect::<Result<Vec<Expr>, KaoriError>>()?;

                let body = body
                    .iter()
                    .map(|stmt| self.resolve_statement(stmt))
                    .collect::<Result<Vec<Stmt>, KaoriError>>()?;

                self.exit_function();

                let id = self.ast_to_hir.get(&declaration.id).unwrap();

                Decl::function(*id, parameters, body, declaration.span)
            }
        };

        Ok(_decl)
    }

    fn resolve_statement(&mut self, statement: &ast::Stmt) -> Result<Stmt, KaoriError> {
        let _stmt = match &statement.kind {
            ast::StmtKind::Expression(expression) => {
                let expr = self.resolve_expression(expression)?;

                Stmt::expression(expr, statement.span)
            }
            ast::StmtKind::Print(expression) => {
                let expr = self.resolve_expression(expression)?;

                Stmt::print(expr, statement.span)
            }
            ast::StmtKind::Block(statements) => {
                self.symbol_table.enter_scope();

                let statements = statements
                    .iter()
                    .map(|stmt| self.resolve_statement(stmt))
                    .collect::<Result<Vec<Stmt>, KaoriError>>()?;

                self.symbol_table.exit_scope();

                Stmt::block(statements, statement.span)
            }
            ast::StmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition = self.resolve_expression(condition)?;
                let then_branch = self.resolve_statement(then_branch)?;
                let else_branch = match else_branch {
                    Some(branch) => Some(self.resolve_statement(branch)?),
                    _ => None,
                };

                Stmt::branch_(condition, then_branch, else_branch, statement.span)
            }
            ast::StmtKind::WhileLoop { condition, block } => {
                let init = None;
                let condition = self.resolve_expression(condition)?;
                let increment = None;

                self.active_loops += 1;
                let block = self.resolve_statement(block)?;
                self.active_loops -= 1;

                Stmt::loop_(init, condition, block, increment, statement.span)
            }
            ast::StmtKind::ForLoop {
                init,
                condition,
                increment,
                block,
            } => {
                self.symbol_table.enter_scope();

                let init = Some(self.resolve_expression(init)?);
                let condition = self.resolve_expression(condition)?;
                let increment = Some(self.resolve_statement(increment)?);

                self.active_loops += 1;
                let block = self.resolve_statement(block)?;
                self.active_loops -= 1;

                self.symbol_table.exit_scope();

                Stmt::loop_(init, condition, block, increment, statement.span)
            }
            ast::StmtKind::Break => {
                if self.active_loops == 0 {
                    return Err(kaori_error!(
                        statement.span,
                        "break statement can't appear outside of loops"
                    ));
                }

                Stmt::break_(statement.span)
            }
            ast::StmtKind::Continue => {
                if self.active_loops == 0 {
                    return Err(kaori_error!(
                        statement.span,
                        "continue statement can't appear outside of loops"
                    ));
                }

                Stmt::continue_(statement.span)
            }
            ast::StmtKind::Return(expr) => {
                let expr = match expr {
                    Some(e) => Some(self.resolve_expression(e)?),
                    _ => None,
                };

                Stmt::return_(expr, statement.span)
            }
        };

        Ok(_stmt)
    }

    fn resolve_expression(&mut self, expression: &ast::Expr) -> Result<Expr, KaoriError> {
        let _expr = match &expression.kind {
            ast::ExprKind::Parameter(name) => {
                if self.symbol_table.search_current_scope(name).is_some() {
                    return Err(kaori_error!(
                        expression.span,
                        "function can't have parameters with the same name: {}",
                        name,
                    ));
                };

                let id = NodeId::default();

                self.symbol_table.declare_variable(id, name.to_owned());

                Expr::parameter(id, expression.span)
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

                let ExprKind::Variable(_) = &left.kind else {
                    return Err(kaori_error!(
                        left.span,
                        "expected a valid left hand side to assign values to"
                    ));
                };

                let operator_kind = match &operator.kind {
                    AssignOpKind::AddAssign => BinaryOpKind::Add,
                    AssignOpKind::SubtractAssign => BinaryOpKind::Subtract,
                    AssignOpKind::MultiplyAssign => BinaryOpKind::Multiply,
                    AssignOpKind::DivideAssign => BinaryOpKind::Divide,
                    AssignOpKind::ModuloAssign => BinaryOpKind::Modulo,
                    AssignOpKind::Assign => return Ok(Expr::assign(left, right, expression.span)),
                };

                let operator = BinaryOp::new(operator_kind);

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

            ast::ExprKind::LogicalNot { expr } => {
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
            ast::ExprKind::BooleanLiteral(value) => Expr::boolean(*value, expression.span),
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
                    SymbolKind::Function => Expr::function(symbol.id, expression.span),
                    SymbolKind::Variable => Expr::variable(symbol.id, expression.span),
                }
            }
        };

        Ok(_expr)
    }
}
