use crate::{
    error::kaori_error::KaoriError,
    frontend::syntax::{
        ast_node::AstNode,
        decl::{Decl, DeclKind},
        expr::{Expr, ExprKind},
        operator::{BinaryOp, UnaryOp},
        stmt::{Stmt, StmtKind},
    },
};

use super::{hir_ast_node::HirAstNode, hir_decl::HirDecl, hir_expr::HirExpr, hir_stmt::HirStmt};

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

        for declaration in declarations.iter() {}

        Ok(resolved_declarations)
    }

    fn swap_main_function(&mut self, declarations: &mut [Decl]) {
        for (index, declaration) in declarations.iter().enumerate() {
            if let DeclKind::Function { name, .. } = &declaration.kind
                && name == "main"
            {
                declarations.swap(0, index);
                break;
            }
        }
    }

    fn generate_nodes(&mut self, nodes: &[AstNode]) -> Vec<HirAstNode> {
        nodes
            .iter()
            .map(|node| self.generate_ast_node(node))
            .collect()
    }

    fn generate_ast_node(&mut self, node: &AstNode) -> HirAstNode {
        match node {
            AstNode::Declaration(declaration) => {
                let declaration = self.generate_declaration(declaration);

                HirAstNode::Declaration(declaration)
            }
            AstNode::Statement(statement) => {
                let statement = self.generate_statement(statement);

                HirAstNode::Statement(statement)
            }
        }
    }

    fn generate_declaration(&mut self, declaration: &Decl) -> HirDecl {
        let resolved_decl = match &declaration.kind {
            DeclKind::Variable { name, right, ty } => {
                let right = self.generate_expression(right);

                HirDecl::variable(name.to_owned(), right, ty.to_owned(), declaration.span)
            }
            DeclKind::Function {
                parameters,
                body,
                name,
                ty,
            } => {
                let body = self.generate_nodes(body);

                HirDecl::function(
                    name.to_owned(),
                    parameters.to_owned(),
                    body,
                    ty.to_owned(),
                    declaration.span,
                )
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
