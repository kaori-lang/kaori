use crate::{
    error::kaori_error::KaoriError,
    frontend::syntax::{
        decl::Decl,
        expr::{Expr, ExprKind},
        operator::{BinaryOp, UnaryOp},
        stmt::{Stmt, StmtKind},
    },
};

use super::{hir_decl::HirDecl, hir_expr::HirExpr, hir_stmt::HirStmt};

struct HirGen {}

impl HirGen {
    pub fn new() -> Self {
        Self {
            environment: Environment::default(),
            active_loops: 0,
        }
    }

    pub fn generate(&mut self, declarations: &mut [Decl]) -> Result<Vec<HirDecl>, KaoriError> {
        self.generate_main_function(declarations)?;

        for declaration in declarations.iter() {
            match &declaration.kind {
                DeclKind::Function { id, name, ty, .. } => {
                    if self.environment.search_current_scope(name).is_some() {
                        return Err(kaori_error!(
                            declaration.span,
                            "{} is already declared",
                            name
                        ));
                    }

                    let ty = self.generate_type(ty)?;

                    self.environment.declare_global(*id, name.to_owned(), ty);
                }
                DeclKind::Struct { id, name, ty, .. } => {
                    if self.environment.search_current_scope(name).is_some() {
                        return Err(kaori_error!(
                            declaration.span,
                            "{} is already declared",
                            name
                        ));
                    }

                    let ty = self.generate_type(ty)?;
                }
                _ => (),
            }
        }

        let resolved_declarations = declarations
            .iter()
            .map(|declaration| self.generate_declaration(declaration))
            .collect::<Result<Vec<ResolvedDecl>, KaoriError>>()?;

        Ok(resolved_declarations)
    }

    fn generate_main_function(&mut self, declarations: &mut [Decl]) -> Result<(), KaoriError> {
        for (index, declaration) in declarations.iter().enumerate() {
            if let DeclKind::Function { name, .. } = &declaration.kind
                && name == "main"
            {
                declarations.swap(0, index);
                return Ok(());
            }
        }

        Err(kaori_error!(
            Span::default(),
            "main function is not declared"
        ))
    }

    fn generate_nodes(&mut self, nodes: &[AstNode]) -> Result<Vec<ResolvedAstNode>, KaoriError> {
        let nodes = nodes
            .iter()
            .map(|node| self.generate_ast_node(node))
            .collect::<Result<Vec<ResolvedAstNode>, KaoriError>>()?;

        Ok(nodes)
    }

    fn generate_ast_node(&mut self, node: &AstNode) -> Result<ResolvedAstNode, KaoriError> {
        let resolved_node = match node {
            AstNode::Declaration(declaration) => {
                let declaration = self.generate_declaration(declaration)?;

                ResolvedAstNode::Declaration(declaration)
            }
            AstNode::Statement(statement) => {
                let statement = self.generate_statement(statement)?;

                ResolvedAstNode::Statement(statement)
            }
        };

        Ok(resolved_node)
    }

    fn generate_declaration(&mut self, declaration: &Decl) -> Result<ResolvedDecl, KaoriError> {
        let resolved_decl = match &declaration.kind {
            DeclKind::Variable { name, right, ty } => {
                let right = self.generate_expression(right)?;

                if self.environment.search_current_scope(name).is_some() {
                    return Err(kaori_error!(
                        declaration.span,
                        "{} is already declared",
                        name
                    ));
                };

                let ty = self.generate_type(ty)?;

                let offset = self
                    .environment
                    .declare_local(name.to_owned(), ty.to_owned());

                ResolvedDecl::variable(offset, right, ty, declaration.span)
            }
            DeclKind::Function {
                id,
                parameters,
                body,
                name,
                ty,
            } => {
                self.environment.enter_scope();

                for parameter in parameters {
                    if self
                        .environment
                        .search_current_scope(&parameter.name)
                        .is_some()
                    {
                        return Err(kaori_error!(
                            parameter.span,
                            "function {} can't have parameters with the same name",
                            name,
                        ));
                    };

                    let ty = self.generate_type(&parameter.ty)?;
                    let name = parameter.name.to_owned();

                    self.environment.declare_local(name, ty);
                }

                let body = self.generate_nodes(body)?;

                self.environment.exit_scope();

                let mut resolved_parameters = Vec::new();

                for parameter in parameters {
                    let ty = self.generate_type(&parameter.ty)?;
                    let span = parameter.span;
                    let parameter = ResolvedParameter { ty, span };

                    resolved_parameters.push(parameter);
                }

                let ty = self.generate_type(ty)?;

                ResolvedDecl::function(*id, resolved_parameters, body, ty, declaration.span)
            }
            DeclKind::Struct {
                id,
                name,
                fields,
                ty,
            } => todo!(),
        };

        Ok(resolved_decl)
    }

    fn generate_statement(&mut self, statement: &Stmt) -> HirStmt {
        match &statement.kind {
            StmtKind::Expression(expression) => {
                let expr = self.generate_expression(expression);

                HirStmt::expression(expr, statement.span)
            }
            StmtKind::Print(expression) => {
                let expr = self.generate_expression(expression);

                HirStmt::print(expr, statement.span)
            }
            StmtKind::Block(nodes) => {
                let nodes = self.generate_nodes(nodes);

                HirStmt::block(nodes, statement.span)
            }
            StmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition = self.generate_expression(condition);
                let then_branch = self.generate_statement(then_branch);
                let else_branch = if let Some(branch) = else_branch {
                    Some(self.generate_statement(branch))
                } else {
                    None
                };

                HirStmt::branch_(condition, then_branch, else_branch, statement.span)
            }
            StmtKind::WhileLoop { condition, block } => {
                let condition = self.generate_expression(condition);
                let block = self.generate_statement(block);

                HirStmt::while_loop(condition, block, statement.span)
            }
            StmtKind::Break => HirStmt::break_(statement.span),

            StmtKind::Continue => HirStmt::continue_(statement.span),

            StmtKind::Return(expr) => {
                let expr = match expr {
                    Some(expr) => Some(self.generate_expression(expr)),
                    None => None,
                };

                HirStmt::return_(expr, statement.span)
            }
        }
    }

    fn generate_expression(&self, expression: &Expr) -> HirExpr {
        match &expression.kind {
            ExprKind::Assign { left, right } => {
                let right = self.generate_expression(right);
                let left = self.generate_expression(left);

                HirExpr::assign(left, right, expression.span)
            }
            ExprKind::Binary {
                left,
                right,
                operator,
            } => {
                let left = self.generate_expression(left);
                let right = self.generate_expression(right);

                match operator {
                    BinaryOp::Add => HirExpr::add(left, right, expression.span),
                    BinaryOp::Subtract => HirExpr::sub(left, right, expression.span),
                    BinaryOp::Multiply => HirExpr::mul(left, right, expression.span),
                    BinaryOp::Divide => HirExpr::div(left, right, expression.span),
                    BinaryOp::Modulo => HirExpr::mod_(left, right, expression.span),
                    BinaryOp::Equal => HirExpr::equal(left, right, expression.span),
                    BinaryOp::NotEqual => HirExpr::not_equal(left, right, expression.span),
                    BinaryOp::Less => HirExpr::less(left, right, expression.span),
                    BinaryOp::LessEqual => HirExpr::less_equal(left, right, expression.span),
                    BinaryOp::Greater => HirExpr::greater(left, right, expression.span),
                    BinaryOp::GreaterEqual => HirExpr::greater_equal(left, right, expression.span),
                    BinaryOp::And => HirExpr::and(left, right, expression.span),
                    BinaryOp::Or => HirExpr::or(left, right, expression.span),
                }
            }
            ExprKind::Unary { right, operator } => {
                let right = self.generate_expression(right);

                match operator {
                    UnaryOp::Not => HirExpr::not(right, expression.span),
                    UnaryOp::Negate => HirExpr::negate(right, expression.span),
                }
            }
            ExprKind::FunctionCall { callee, arguments } => {
                let callee = self.generate_expression(callee);
                let arguments = arguments
                    .iter()
                    .map(|arg| self.generate_expression(arg))
                    .collect();

                HirExpr::function_call(callee, arguments, expression.span)
            }
            ExprKind::NumberLiteral(value) => HirExpr::number_literal(*value, expression.span),
            ExprKind::BooleanLiteral(value) => HirExpr::boolean_literal(*value, expression.span),
            ExprKind::StringLiteral(value) => {
                HirExpr::string_literal(value.to_owned(), expression.span)
            }
            ExprKind::Identifier { name } => HirExpr::identifier(name.to_owned(), expression.span),
        }
    }
}
