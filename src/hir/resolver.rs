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
    decl::{Decl, Field, Parameter},
    expr::{Expr, ExprKind},
    node::Node,
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
            if let ast::DeclKind::Function { name, .. } = &declaration.kind
                && name == "main"
            {
                declarations.swap(0, index);

                return Ok(());
            }
        }

        Err(kaori_error!(
            Span::default(),
            "expected a main function to be declared in the program"
        ))
    }

    fn resolve_declaration_scope(&self, declaration: &ast::Decl) -> Result<(), KaoriError> {
        let has_error = match &declaration.kind {
            ast::DeclKind::Function { .. } if self.local_scope => true,
            ast::DeclKind::Struct { .. } if self.local_scope => true,
            ast::DeclKind::Variable { .. } if !self.local_scope => true,
            _ => false,
        };

        if has_error {
            Err(kaori_error!(
                declaration.span,
                "expected declaration to be made in the correct scope"
            ))
        } else {
            Ok(())
        }
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
                ast::DeclKind::Struct { name, .. } => {
                    if self.symbol_table.search_current_scope(name).is_some() {
                        return Err(kaori_error!(
                            declaration.span,
                            "{} is already declared",
                            name
                        ));
                    }

                    let id = self.generate_id(declaration.id);

                    self.symbol_table.declare_struct(id, name.to_owned());
                }
                _ => (),
            };
        }

        let declarations = declarations
            .iter()
            .map(|declaration| self.resolve_declaration(declaration))
            .collect::<Result<Vec<Decl>, KaoriError>>()?;

        Ok(declarations)
    }

    fn resolve_node(&mut self, node: &ast::Node) -> Result<Node, KaoriError> {
        let node = match node {
            ast::Node::Declaration(declaration) => {
                let declaration = self.resolve_declaration(declaration)?;
                Node::from(declaration)
            }
            ast::Node::Statement(statement) => {
                let statement = self.resolve_statement(statement)?;
                Node::from(statement)
            }
        };

        Ok(node)
    }

    fn resolve_parameter(&mut self, parameter: &ast::Parameter) -> Result<Parameter, KaoriError> {
        if self
            .symbol_table
            .search_current_scope(&parameter.name)
            .is_some()
        {
            return Err(kaori_error!(
                parameter.span,
                "function can't have parameters with the same name: {}",
                parameter.name,
            ));
        };

        let id = NodeId::default();

        self.symbol_table
            .declare_variable(id, parameter.name.to_owned());

        Ok(Parameter::new(id, parameter.span))
    }

    fn resolve_field(&mut self, field: &ast::Field) -> Result<Field, KaoriError> {
        if self
            .symbol_table
            .search_current_scope(&field.name)
            .is_some()
        {
            return Err(kaori_error!(
                field.span,
                "struct can't have fields with the same name: {}",
                field.name,
            ));
        };

        let id = NodeId::default();

        self.symbol_table
            .declare_variable(id, field.name.to_owned());

        Ok(Field::new(id, field.span))
    }

    fn resolve_declaration(&mut self, declaration: &ast::Decl) -> Result<Decl, KaoriError> {
        self.resolve_declaration_scope(declaration)?;

        let _decl = match &declaration.kind {
            ast::DeclKind::Variable { name, right } => {
                let right = self.resolve_expression(right)?;

                if self.symbol_table.search_current_scope(name).is_some() {
                    return Err(kaori_error!(
                        declaration.span,
                        "{} is already declared",
                        name
                    ));
                };

                let id = NodeId::default();

                self.symbol_table.declare_variable(id, name.to_owned());

                Decl::variable(id, right, declaration.span)
            }
            ast::DeclKind::Function {
                parameters, body, ..
            } => {
                self.enter_function();

                let parameters = parameters
                    .iter()
                    .map(|parameter| self.resolve_parameter(parameter))
                    .collect::<Result<Vec<Parameter>, KaoriError>>()?;

                let body = body
                    .iter()
                    .map(|node| self.resolve_node(node))
                    .collect::<Result<Vec<Node>, KaoriError>>()?;

                self.exit_function();

                let id = self.ast_to_hir.get(&declaration.id).unwrap();

                Decl::function(*id, parameters, body, declaration.span)
            }
            ast::DeclKind::Struct { fields, .. } => {
                let fields = fields
                    .iter()
                    .map(|field| self.resolve_field(field))
                    .collect::<Result<Vec<Field>, KaoriError>>()?;

                let id = self.ast_to_hir.get(&declaration.id).unwrap();

                Decl::struct_(*id, fields, declaration.span)
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
            ast::StmtKind::Block(nodes) => {
                self.symbol_table.enter_scope();

                let nodes = nodes
                    .iter()
                    .map(|node| self.resolve_node(node))
                    .collect::<Result<Vec<Node>, KaoriError>>()?;

                self.symbol_table.exit_scope();

                Stmt::block(nodes, statement.span)
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

                let init = Some(self.resolve_declaration(init)?);
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

    fn resolve_expression(&self, expression: &ast::Expr) -> Result<Expr, KaoriError> {
        let _expr = match &expression.kind {
            ast::ExprKind::Assign {
                operator,
                left,
                right,
            } => {
                let right = self.resolve_expression(right)?;
                let left = self.resolve_expression(left)?;

                let ExprKind::Variable(..) = &left.kind else {
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
                    _ => return Ok(Expr::assign(left, right, expression.span)),
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
            ast::ExprKind::NumberLiteral(value) => Expr::number(*value, expression.span),
            ast::ExprKind::BooleanLiteral(value) => Expr::boolean(*value, expression.span),
            ast::ExprKind::StringLiteral(value) => Expr::string(value.to_owned(), expression.span),
            ast::ExprKind::Identifier(name) => {
                let Some(symbol) = self.symbol_table.search(name) else {
                    return Err(kaori_error!(expression.span, "{} is not declared", name));
                };

                match symbol.kind {
                    SymbolKind::Function => Expr::function(symbol.id, expression.span),
                    SymbolKind::Variable => Expr::variable(symbol.id, expression.span),
                    SymbolKind::Struct => {
                        return Err(kaori_error!(expression.span, "{} is not a value", name));
                    }
                }
            }
        };

        Ok(_expr)
    }
}
